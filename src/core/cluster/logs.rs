use std::{
    collections::BTreeMap,
    fmt::Debug,
    ops::{Bound, RangeBounds},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use fjall_utils::{FjallFixedKey, KeyspaceExt, MonotonicTableRow, TableRow, WriteBatchExt};
use jiff::Timestamp;
use openraft::{
    EntryPayload, OptionalSend, RaftLogReader, RaftTypeConfig,
    storage::{IOFlushed, RaftLogStorage},
    type_config::alias::{LogIdOf, VoteOf},
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tap::{Pipe, Tap, TapFallible, TapOptional};
use tracing::{Instrument, Span};

use super::{NodeId, raft::TypeConfig};
use crate::{
    cfg::{Dir, FsyncMode},
    core::{cluster::ClusterId, metrics::LogMetrics},
};
use diom_core::task::spawn_blocking_in_current_span;
use diom_error::Result;

// This is an implementation of an openraft Logs store backed by fjall

type LogEntry = <TypeConfig as RaftTypeConfig>::Entry;
type LogId = LogIdOf<TypeConfig>;
type Vote = VoteOf<TypeConfig>;

#[derive(Debug)]
struct LogCacheInner {
    inner: BTreeMap<u64, LogEntry>,
    capacity: usize,
}

impl LogCacheInner {
    fn new(capacity: usize) -> Self {
        Self {
            inner: BTreeMap::new(),
            capacity,
        }
    }

    fn push(&mut self, entry: LogEntry) {
        while self.inner.len() >= self.capacity {
            self.inner.pop_first();
        }
        self.inner.insert(entry.log_id.index, entry);
    }

    fn purge(&mut self, log_index: u64) {
        // https://github.com/rust-lang/rust/issues/81074
        let keys = self
            .inner
            .range(..=log_index)
            .map(|(k, _v)| *k)
            .collect::<Vec<_>>();
        for key in keys {
            self.inner.remove(&key);
        }
    }

    fn truncate(&mut self, log_index: u64) {
        // https://github.com/rust-lang/rust/issues/81074
        let keys = self
            .inner
            .range(log_index..)
            .map(|(k, _v)| *k)
            .collect::<Vec<_>>();
        for key in keys {
            self.inner.remove(&key);
        }
    }

    fn get(&self, log_index: &u64) -> Option<&LogEntry> {
        self.inner.get(log_index)
    }
}

#[derive(Clone)]
struct LogCache(Arc<Mutex<LogCacheInner>>);

impl LogCache {
    fn new(capacity: usize) -> Self {
        Self(Arc::new(Mutex::new(LogCacheInner::new(capacity))))
    }

    fn push(&self, entry: LogEntry) {
        self.0.lock().push(entry)
    }

    fn purge(&self, log_index: u64) {
        self.0.lock().purge(log_index)
    }

    fn truncate(&self, log_index: u64) {
        self.0.lock().truncate(log_index);
    }

    fn get(&self, log_index: &u64) -> Option<LogEntry> {
        self.0.lock().get(log_index).cloned()
    }
}

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Log = 0,
    LogIndex = 1,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Log(LogEntry);

impl TableRow for Log {
    const ROW_TYPE: u8 = RowType::Log as u8;
}

impl MonotonicTableRow for Log {
    type KeyType = u64;

    fn get_key(&self) -> u64 {
        self.0.log_id.index
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LogIndex {
    unix_timestamp_ms: u64,
    log_id: u64,
}

impl TableRow for LogIndex {
    const ROW_TYPE: u8 = RowType::LogIndex as u8;
}

impl MonotonicTableRow for LogIndex {
    type KeyType = u64;

    fn get_key(&self) -> u64 {
        self.unix_timestamp_ms
    }
}

fn io_err(error: anyhow::Error) -> std::io::Error {
    std::io::Error::other(error)
}

impl RaftLogReader<TypeConfig> for DiomLogs {
    #[tracing::instrument(skip_all, fields(num_entries_found))]
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + OptionalSend>(
        &mut self,
        range: RB,
    ) -> std::io::Result<Vec<LogEntry>> {
        let output = self
            .read_log_entries::<RB>(range.clone())
            .await
            .map_err(io_err)?;
        Span::current().record("num_entries_found", output.len());
        Ok(output)
    }

    #[tracing::instrument(skip_all)]
    async fn read_vote(&mut self) -> std::io::Result<Option<Vote>> {
        self.read_vote_().await.map_err(io_err)
    }
}

impl RaftLogStorage<TypeConfig> for DiomLogs {
    type LogReader = Self;

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    #[tracing::instrument(skip_all)]
    async fn get_log_state(&mut self) -> std::io::Result<openraft::LogState<TypeConfig>> {
        self.get_log_state_().await.map_err(io_err)
    }

    #[tracing::instrument(skip_all)]
    async fn save_vote(&mut self, vote: &Vote) -> std::io::Result<()> {
        self.save_vote_(vote.to_owned()).await.map_err(io_err)?;
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn append<I>(
        &mut self,
        entries: I,
        callback: IOFlushed<TypeConfig>,
    ) -> std::io::Result<()>
    where
        I: IntoIterator<Item = LogEntry> + Send,
        I::IntoIter: Send,
    {
        // TODO: figure out a way to do this without collecting into a vec here; the problem
        // is that I is Send, but isn't 'static, so it can't be sent over with tokio::task::spawn_blocking...
        let entries = entries.into_iter().collect();
        self.append_entries_(entries, callback)
            .await
            .map_err(io_err)?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(?log_id))]
    async fn truncate_after(&mut self, log_id: Option<LogId>) -> std::io::Result<()> {
        self.truncate_entries_(log_id).await.map_err(io_err)
    }

    #[tracing::instrument(skip_all, fields(?log_id))]
    async fn purge(&mut self, log_id: LogId) -> std::io::Result<()> {
        self.purge_entries_(log_id).await.map_err(io_err)
    }

    #[tracing::instrument(skip_all, fields(log_id = ?committed))]
    async fn save_committed(&mut self, committed: Option<LogId>) -> std::io::Result<()> {
        self.save_committed_(committed).await.map_err(io_err)
    }

    #[tracing::instrument(skip_all)]
    async fn read_committed(&mut self) -> std::io::Result<Option<LogId>> {
        self.read_committed_().await.map_err(io_err)
    }
}

static NODE_ID: FjallFixedKey<NodeId> = FjallFixedKey::new("node_id");
static LAST_PURGED_LOG_ID: FjallFixedKey<LogId> = FjallFixedKey::new("last_purged_log_id");
static VOTE: FjallFixedKey<Vote> = FjallFixedKey::new("vote");
static COMMITTED: FjallFixedKey<Option<LogId>> = FjallFixedKey::new("committed");
static POISONED: FjallFixedKey<ClusterId> = FjallFixedKey::new("poisoned");

#[derive(Debug, Clone)]
pub(super) struct BackgroundFsyncFailedError(String);

impl std::fmt::Display for BackgroundFsyncFailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background fsync failed: {}", self.0)
    }
}

impl std::error::Error for BackgroundFsyncFailedError {
    fn description(&self) -> &str {
        "background fsync failed"
    }
}

// Specialization for `flush_worker` when commits_before_fsync is 1 (which implies
// that ack_immediately is false) which just fsyncs as fast as it can.
async fn flush_every_worker(
    db: Database,
    mut channel: tokio::sync::mpsc::Receiver<IOFlushed<TypeConfig>>,
) {
    while let Some(callback) = channel.recv().await {
        let db = db.clone();
        // this shouldn't inherit our span
        #[allow(clippy::disallowed_methods)]
        let result = tokio::task::spawn_blocking(move || {
            let _guard = tracing::info_span!("logs:flush_worker:every").entered();
            tracing::trace!("fsyncing logs to disk");
            db.persist(PersistMode::SyncAll)
                // fjall::Error isn't Clone
                .map_err(|err| {
                    tracing::error!(?err, "error flushing fjall in background");
                    BackgroundFsyncFailedError(err.to_string())
                })
        })
        .await
        .expect("failed joining blocking task");
        callback.io_completed(result.map_err(std::io::Error::other));
    }
}

/// General background worker for flushing the fjall database
async fn flush_worker(
    db: Database,
    mut channel: tokio::sync::mpsc::Receiver<IOFlushed<TypeConfig>>,
    commits_before_fsync: usize,
    duration_before_fsync: Duration,
    ack_immediately: bool,
    fsync_mode: FsyncMode,
) {
    let mut pending = Vec::new();
    let mut done = false;
    let shutting_down = crate::shutting_down_token();
    let mut ticker = tokio::time::interval(duration_before_fsync);
    while !done {
        async {
            let mut sync_now = false;

            tokio::select! {
                message = channel.recv() => {
                    if let Some(callback) = message {
                        if ack_immediately {
                            let db = db.clone();
                            let result = spawn_blocking_in_current_span(move || {
                                let _guard = tracing::info_span!("logs:flush_worker:buffer").entered();
                                db.persist(PersistMode::Buffer)
                                    // fjall::Error isn't Clone
                                    .map_err(|err| {
                                        tracing::error!(?err, "error flushing fjall in background");
                                        BackgroundFsyncFailedError(err.to_string())
                                    })
                            })
                            .await
                            .expect("failed joining blocking task");
                            callback.io_completed(result.map_err(std::io::Error::other));
                            pending.push(None);
                        } else {
                            pending.push(Some(callback))
                        }
                    } else {
                        done = true;
                    }
                },
                _ = shutting_down.cancelled() => {
                    done = true
                },
                _ = ticker.tick() => {
                    sync_now = !pending.is_empty()
                }
            }

            if sync_now || (commits_before_fsync > 0 && pending.len() >= commits_before_fsync) {
                let db = db.clone();
                let num_commits = pending.len();
                let result = spawn_blocking_in_current_span(
                    move || -> Result<(), BackgroundFsyncFailedError> {
                        let _guard =
                            tracing::info_span!("logs:flush_worker:flush", num_commits).entered();
                        tracing::trace!("flushing logs to disk");
                        db.persist(fsync_mode.into()).map_err(|err| {
                            tracing::error!(?err, "error flushing fjall");
                            BackgroundFsyncFailedError(err.to_string())
                        })
                    },
                )
                .await
                .expect("failed joining blocking task");
                tracing::trace!(num_pending = pending.len(), "committed for some items");
                tracing::info_span!("logs:flush_worker:drain").in_scope(|| {
                    for callback in pending.drain(..).flatten() {
                        callback.io_completed(result.clone().map_err(std::io::Error::other))
                    }
                });
            }
        }
        .instrument(tracing::info_span!("logs:flush_worker"))
        .await
    }
    if let Err(err) = db.persist(fsync_mode.into()) {
        tracing::error!(?err, "error flushing fjall at shutdown");
    }
}

#[derive(Clone)]
pub struct DiomLogs {
    db: Database,
    meta_keyspace: Keyspace,
    log_keyspace: Keyspace,
    flush_tx: tokio::sync::mpsc::Sender<IOFlushed<TypeConfig>>,
    log_cache: LogCache,
    metrics: Option<LogMetrics>,
    last_vote: Arc<Mutex<Option<Vote>>>,
}

impl DiomLogs {
    const DELETE_BATCH_SIZE: usize = 10_000;

    pub fn new(
        path: Dir,
        commits_before_fsync: usize,
        duration_before_fsync: Duration,
        ack_immediately: bool,
        fsync_mode: FsyncMode,
    ) -> anyhow::Result<Self> {
        let pb: std::path::PathBuf = path.into();
        let db = Database::builder(&pb).worker_threads(1).open()?;
        let log_keyspace = db.keyspace("cluster:logs", || {
            KeyspaceCreateOptions::default()
                .manual_journal_persist(true)
                .expect_point_read_hits(true)
        })?;
        let meta_keyspace = db.keyspace("cluster:meta", KeyspaceCreateOptions::default)?;
        let (flush_tx, flush_rx) = tokio::sync::mpsc::channel(65536);
        if commits_before_fsync == 1 {
            tokio::spawn(flush_every_worker(db.clone(), flush_rx));
        } else {
            tokio::spawn(flush_worker(
                db.clone(),
                flush_rx,
                commits_before_fsync,
                duration_before_fsync,
                ack_immediately,
                fsync_mode,
            ));
        }
        Ok(Self {
            db,
            log_keyspace,
            meta_keyspace,
            flush_tx,
            log_cache: LogCache::new(100),
            metrics: None,
            last_vote: Arc::new(Mutex::new(None)),
        })
    }

    pub(crate) fn enable_metrics(&mut self, metrics: LogMetrics) {
        self.metrics = Some(metrics.clone());
        self.start_metrics(metrics);
    }

    fn metric_record<F>(&self, f: F)
    where
        F: FnOnce(&LogMetrics),
    {
        if let Some(metrics) = &self.metrics {
            f(metrics)
        }
    }

    #[tracing::instrument(skip_all, fields(?timestamp, ?log_index))]
    pub(crate) async fn record_log_timestamp(
        &self,
        timestamp: Timestamp,
        log_index: u64,
    ) -> anyhow::Result<()> {
        let rec = LogIndex {
            unix_timestamp_ms: timestamp.as_millisecond() as u64,
            log_id: log_index,
        };
        tracing::trace!(?rec, "recording log/timestamp checkpoint");
        let keyspace = self.log_keyspace.clone();
        spawn_blocking_in_current_span(move || keyspace.insert_row(rec.key(), &rec)).await??;
        Ok(())
    }

    /// Get the NodeId (or, if we don't have one, make a new one)
    pub async fn get_node_id(&mut self) -> anyhow::Result<NodeId> {
        let db = self.db.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            if let Some(node_id) = NODE_ID.get(&meta_keyspace)? {
                tracing::info!(%node_id, "starting up with existing node ID");
                node_id
            } else {
                let node_id = NodeId::generate();
                tracing::info!(%node_id, "generated a new node ID");
                NODE_ID
                    .store(&meta_keyspace, &node_id)
                    .context("saving node ID to logs database")?;
                db.persist(PersistMode::SyncAll)?;
                node_id
            }
            .pipe(Ok)
        })
        .await?
    }

    #[tracing::instrument(skip_all, fields(num_entries))]
    async fn append_entries_(
        &mut self,
        entries: Vec<LogEntry>,
        callback: IOFlushed<TypeConfig>,
    ) -> anyhow::Result<()> {
        Span::current().record("num_entries", entries.len());
        let start = std::time::Instant::now();
        let num_entries = entries.len();

        let keyspace = self.log_keyspace.clone();
        let persisted_entries = entries.clone();
        // set durability to None because we're going to sync it in the flush worker
        let mut batch =
            fjall::OwnedWriteBatch::with_capacity(self.db.clone(), entries.len()).durability(None);
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            let _guard = tracing::info_span!("append:write_entries").entered();
            for entry in persisted_entries {
                let log = Log(entry);
                batch.insert_row(&keyspace, log.key(), &log)?;
            }
            batch.commit()?;
            Ok(())
        })
        .await??;

        self.flush_tx
            .send(callback)
            .await
            .context("requesting background fsync")?;

        tracing::trace!(num_entries, "appended some entries");

        for entry in entries {
            self.log_cache.push(entry);
        }

        self.metric_record(|m| m.record_append(start.elapsed()));

        Ok(())
    }

    /// Truncate logs since log_id, exclusive
    async fn truncate_entries_(&self, log_id: Option<LogId>) -> anyhow::Result<()> {
        let start = log_id.map(|l| l.index + 1).unwrap_or(0);
        self.log_cache.truncate(start);
        let log_keyspace = self.log_keyspace.clone();
        let db = self.db.clone();
        spawn_blocking_in_current_span(move || {
            let deleted = Log::remove_keys_in_range(
                &db,
                &log_keyspace,
                start..,
                Self::DELETE_BATCH_SIZE,
                PersistMode::Buffer,
            )?;
            tracing::debug!(deleted, "deleted entries for truncation");
            Ok(())
        })
        .await?
    }

    /// Purge logs upto log_id, inclusive
    async fn purge_entries_(&self, log_id: LogId) -> anyhow::Result<()> {
        self.log_cache.purge(log_id.index);
        let meta_keyspace = self.meta_keyspace.clone();
        let log_keyspace = self.log_keyspace.clone();
        let db = self.db.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            LAST_PURGED_LOG_ID.store(&meta_keyspace, &log_id)?;
            let deleted = Log::remove_keys_in_range(
                &db,
                &log_keyspace,
                ..=log_id.index,
                Self::DELETE_BATCH_SIZE,
                PersistMode::Buffer,
            )?;
            tracing::debug!(deleted, "deleted entries for purge");
            Ok(())
        })
        .await?
    }

    async fn get_log_state_(&mut self) -> anyhow::Result<openraft::LogState<TypeConfig>> {
        let log_keyspace = self.log_keyspace.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            let last_purged_log_id = LAST_PURGED_LOG_ID.get(&meta_keyspace)?;
            let last_log_id =
                if let Some(Ok(last_guard)) = Log::range(&log_keyspace, ..).next_back() {
                    Some(last_guard.1.0.log_id)
                } else {
                    last_purged_log_id
                };
            Ok(openraft::LogState {
                last_purged_log_id,
                last_log_id,
            })
        })
        .await?
        .tap(|state| tracing::trace!(?state, "read initial log state"))
    }

    async fn read_log_entries<RB>(&mut self, range: RB) -> anyhow::Result<Vec<LogEntry>>
    where
        RB: RangeBounds<u64> + Clone + Debug + OptionalSend,
    {
        let log_keyspace = self.log_keyspace.clone();
        // the most common case is that we just wrote a log entry in append_entries_ and now we're
        // reading it out to apply it. we don't need to go to disk for that!
        match (range.start_bound(), range.end_bound()) {
            (Bound::Included(i), Bound::Excluded(j)) if i + 1 == *j => {
                tracing::trace!("short-circuiting for single-log read");
                if let Some(entry) = self.log_cache.get(i) {
                    return Ok(vec![entry]);
                }
            }
            _ => {}
        }

        // For some reason, RB isn't specified as Send in the trait, so we can't
        // use it directly across the boundary. ARGH!
        let send_range = match range.start_bound() {
            Bound::Unbounded => 0..,
            Bound::Included(i) => *i..,
            Bound::Excluded(i) => (*i + 1)..,
        };
        // why isn't RB always Send? it's a goddamn range...
        let end = match range.end_bound() {
            Bound::Unbounded => None,
            Bound::Included(i) => Some(*i + 1),
            Bound::Excluded(i) => Some(*i),
        };
        let value = spawn_blocking_in_current_span(move || -> anyhow::Result<_> {
            let mut output = vec![];
            for row in Log::range(&log_keyspace, send_range) {
                let (key, value) =
                    row.tap_err(|err| tracing::warn!(?err, "Error reading values from log"))?;
                if let Some(end) = end
                    && key >= end
                {
                    break;
                }

                output.push(value.0);
            }
            Ok(output)
        })
        .await??;
        self.metric_record(|m| m.record_log_read(value.len()));
        Ok(value)
    }

    async fn save_vote_(&self, vote: Vote) -> anyhow::Result<()> {
        tracing::trace!(?vote, "saving a vote");
        let db = self.db.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            VOTE.store(&meta_keyspace, &vote)?;
            tracing::info_span!("save_vote:persist")
                .in_scope(|| db.persist(PersistMode::SyncAll))?;
            Ok(())
        })
        .await?
        .context("saving vote")?;
        let mut guard = self.last_vote.lock();
        *guard = Some(vote);
        Ok(())
    }

    async fn read_vote_(&self) -> anyhow::Result<Option<Vote>> {
        {
            let guard = self.last_vote.lock();
            if let Some(vote) = &*guard {
                return Ok(Some(*vote));
            }
        }
        let keyspace = self.meta_keyspace.clone();
        let Some(vote) = spawn_blocking_in_current_span(move || VOTE.get(&keyspace)).await?? else {
            tracing::trace!("couldn't find a vote");
            return Ok(None);
        };
        tracing::trace!(?vote, "read a vote");
        {
            let mut guard = self.last_vote.lock();
            *guard = Some(vote);
        }
        Ok(Some(vote))
    }

    async fn save_committed_(&self, committed: Option<LogId>) -> anyhow::Result<()> {
        let meta_keyspace = self.meta_keyspace.clone();
        tracing::trace!(?committed, "saving committed state");
        spawn_blocking_in_current_span(move || COMMITTED.store(&meta_keyspace, &committed))
            .await?
            .context("saving committed state")
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId>> {
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            COMMITTED
                .get(&meta_keyspace)?
                .tap_some(|committed| tracing::trace!(?committed, "read committed state"))
                .flatten()
                .pipe(Ok)
        })
        .await?
    }

    fn start_metrics(&self, metrics: LogMetrics) {
        let mut logs = self.clone();
        let db = self.db.clone();
        let shutdown = crate::shutting_down_token();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            while shutdown.run_until_cancelled(ticker.tick()).await.is_some() {
                match spawn_blocking_in_current_span({
                    let db = db.clone();
                    move || db.disk_space()
                })
                .await
                .expect("Failed joining blocking task")
                {
                    Ok(bytes) => metrics.bytes_used(bytes),
                    Err(err) => tracing::info!(?err, "failed to read log disk space"),
                }

                match logs.get_log_state_().await {
                    Ok(state) => {
                        let last = state.last_log_id.map(|id| id.index).unwrap_or(0);
                        let purged = state.last_purged_log_id.map(|id| id.index).unwrap_or(0);
                        metrics.entry_count(last.saturating_sub(purged));
                    }
                    Err(err) => tracing::info!(?err, "failed to read log state for metrics"),
                }
            }
        });
    }

    /// Return the highest log index that we know occurred before the given timestamp,
    pub async fn log_index_before(&self, ts: Timestamp) -> anyhow::Result<Option<u64>> {
        let log_keyspace = self.log_keyspace.clone();
        let range = ..(ts.as_millisecond() as u64);
        spawn_blocking_in_current_span(move || {
            if let Some(row) = LogIndex::range(&log_keyspace, range).next_back() {
                Ok(Some(row?.1.log_id))
            } else {
                Ok(None)
            }
        })
        .await?
    }

    /// Return the highest log index that we know occurred at or after the given timestamp,
    pub async fn log_index_after(&self, ts: Timestamp) -> anyhow::Result<Option<u64>> {
        let log_keyspace = self.log_keyspace.clone();
        let range = (ts.as_millisecond() as u64)..;
        spawn_blocking_in_current_span(move || {
            if let Some(row) = LogIndex::range(&log_keyspace, range).next() {
                Ok(Some(row?.1.log_id))
            } else {
                Ok(None)
            }
        })
        .await?
    }

    pub(super) async fn get_last_timestamp(&self) -> anyhow::Result<Option<Timestamp>> {
        let log_keyspace = self.log_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            for guard in Log::range(&log_keyspace, ..).rev() {
                if let Ok(guard) = guard
                    && let EntryPayload::Normal(req) = guard.1.0.payload
                {
                    return Some(req.timestamp);
                }
            }
            None
        })
        .await
        .context("failed to join")
    }

    pub(crate) async fn poison(&self, cluster_id: ClusterId) -> anyhow::Result<()> {
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || POISONED.store(&meta_keyspace, &cluster_id))
            .await?
            .context("saving poisoned state")
    }

    pub(crate) async fn is_poisoned(&self) -> anyhow::Result<bool> {
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            POISONED
                .get(&meta_keyspace)?
                .tap_some(|cluster_id| {
                    tracing::error!(?cluster_id, "this node was previously poisoned")
                })
                .is_some()
                .pipe(Ok)
        })
        .await?
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::DiomLogs;
    use crate::cfg::{Dir, FsyncMode};
    use jiff::{Span, Timestamp};
    use tempfile::TempDir;
    use test_utils::TestResult;

    struct TestContext {
        _workdir: TempDir,
        logs: DiomLogs,
    }

    impl TestContext {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let logdir = Dir::new(&workdir).unwrap();
            let logs = DiomLogs::new(
                logdir,
                0,
                Duration::from_hours(1),
                true,
                FsyncMode::default(),
            )
            .unwrap();
            Self {
                _workdir: workdir,
                logs,
            }
        }
    }

    #[tokio::test]
    async fn test_log_timestamps() -> TestResult {
        let context = TestContext::new();
        #[allow(clippy::disallowed_methods)]
        let now = Timestamp::now();
        context
            .logs
            .record_log_timestamp(now - Span::new().hours(1), 1)
            .await?;
        context
            .logs
            .record_log_timestamp(now - Span::new().minutes(30), 10)
            .await?;
        context
            .logs
            .record_log_timestamp(now - Span::new().minutes(1), 20)
            .await?;

        assert_eq!(
            context
                .logs
                .log_index_before(now - Span::new().hours(1))
                .await?,
            None
        );
        assert_eq!(
            context
                .logs
                .log_index_before(now - Span::new().seconds(3599))
                .await?,
            Some(1)
        );
        assert_eq!(
            context
                .logs
                .log_index_before(now + Span::new().seconds(1))
                .await?,
            Some(20)
        );
        assert_eq!(
            context
                .logs
                .log_index_after(now - Span::new().hours(1))
                .await?,
            Some(1)
        );
        assert_eq!(
            context
                .logs
                .log_index_after(now + Span::new().seconds(1))
                .await?,
            None
        );
        Ok(())
    }
}
