use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::Debug,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use anyhow::Context;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use fjall_utils::{FjallFixedKey, MonotonicTableRowExt, TableRow};
use jiff::Timestamp;
use openraft::{
    Entry, LogId, OptionalSend, RaftLogReader, RaftTypeConfig, StorageError, Vote,
    storage::{LogFlushed, RaftLogStorage},
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tap::{Pipe, Tap, TapFallible, TapOptional};
use tracing::Span;

use super::{NodeId, errors::*, raft::TypeConfig};
use crate::cfg::{Dir, FsyncMode};

// This is an implementation of an openraft Logs store backed by fjall

type StorageResult<T> = Result<T, StorageError<NodeId>>;

type LogEntry = <TypeConfig as RaftTypeConfig>::Entry;

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
        self.inner.get(&log_index)
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
        self.0.lock().get(log_index).map(|e| e.clone())
    }
}

#[derive(Clone)]
pub struct DiomLogs {
    db: Database,
    meta_keyspace: Keyspace,
    log_keyspace: Keyspace,
    flush_tx: tokio::sync::mpsc::Sender<LogFlushed<TypeConfig>>,
    log_cache: LogCache,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct Log(LogEntry);

impl TableRow for Log {
    const TABLE_PREFIX: &str = "log";
    type Key = u64;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(self.0.log_id.index)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LogIndex {
    unix_timestamp_millis: u64,
    log_id: u64,
}

impl TableRow for LogIndex {
    const TABLE_PREFIX: &str = "timestamps";
    type Key = u64;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(self.unix_timestamp_millis)
    }
}

impl RaftLogReader<TypeConfig> for DiomLogs {
    #[tracing::instrument(skip(self), fields(num_entries_found))]
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + OptionalSend>(
        &mut self,
        range: RB,
    ) -> Result<Vec<LogEntry>, StorageError<NodeId>> {
        let output = self
            .read_log_entries::<RB>(range.clone())
            .await
            .map_err(read_logs_err)?;
        Span::current().record("num_entries_found", output.len());
        Ok(output)
    }
}

impl RaftLogStorage<TypeConfig> for DiomLogs {
    type LogReader = Self;

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    #[tracing::instrument(skip(self))]
    async fn get_log_state(
        &mut self,
    ) -> Result<openraft::LogState<TypeConfig>, StorageError<NodeId>> {
        self.get_log_state_().await.map_err(read_err)
    }

    #[tracing::instrument(skip(self))]
    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        self.save_vote_(vote.to_owned())
            .await
            .map_err(write_vote_err)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        self.read_vote_().await.map_err(read_vote_err)
    }

    #[tracing::instrument(skip_all)]
    async fn append<I>(&mut self, entries: I, callback: LogFlushed<TypeConfig>) -> StorageResult<()>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
        I::IntoIter: Send,
    {
        // TODO: figure out a way to do this without collecting into a vec here; the problem
        // is that I is Send, but isn't 'static, so it can't be sent over with tokio::task::spawn_blocking...
        let entries = entries.into_iter().collect();
        self.append_entries_(entries, callback)
            .await
            .map_err(write_logs_err)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn truncate(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        self.truncate_entries_(log_id).await.map_err(write_logs_err)
    }

    #[tracing::instrument(skip(self))]
    async fn purge(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        self.purge_entries_(log_id).await.map_err(write_logs_err)
    }

    #[tracing::instrument(skip(self))]
    async fn save_committed(&mut self, committed: Option<LogId<NodeId>>) -> StorageResult<()> {
        self.save_committed_(committed).await.map_err(write_err)
    }

    #[tracing::instrument(skip(self))]
    async fn read_committed(&mut self) -> Result<Option<LogId<NodeId>>, StorageError<NodeId>> {
        self.read_committed_().await.map_err(write_err)
    }
}

static NODE_ID: FjallFixedKey<NodeId> = FjallFixedKey::new("node_id");
static LAST_PURGED_LOG_ID: FjallFixedKey<LogId<NodeId>> = FjallFixedKey::new("last_purged_log_id");
static VOTE: FjallFixedKey<Vote<NodeId>> = FjallFixedKey::new("vote");
static COMMITTED: FjallFixedKey<Option<LogId<NodeId>>> = FjallFixedKey::new("committed");

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

async fn flush_every_commit(
    db: Database,
    mut channel: tokio::sync::mpsc::Receiver<LogFlushed<TypeConfig>>,
) {
    // coalesce requests to flush the database; this doesn't actually do anything in openraft 0.9,
    // but in v0.10, we will be able to coalesce
    let mut buf = Vec::with_capacity(10);
    while channel.recv_many(&mut buf, 10).await > 0 {
        let mut new_buf = Vec::with_capacity(10);
        let db = db.clone();
        std::mem::swap(&mut buf, &mut new_buf);
        tokio::task::spawn_blocking(move || {
            let _guard = tracing::info_span!("logs:flush_every_commit").entered();

            let result = db
                .persist(PersistMode::SyncAll)
                // fjall::Error isn't Clone
                .map_err(|err| {
                    tracing::error!(?err, "error flushing fjall in background");
                    BackgroundFsyncFailedError(err.to_string())
                });
            for callback in new_buf.drain(..) {
                callback.log_io_completed(result.clone().map_err(std::io::Error::other));
            }
        })
        .await
        .expect("failed joining blocking task");
    }
}

async fn flush_every_second(
    db: Database,
    mut channel: tokio::sync::mpsc::Receiver<LogFlushed<TypeConfig>>,
) {
    let mut has_changes = false;
    let mut done = false;
    let shutting_down = crate::shutting_down_token();
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));
    while !done {
        tokio::select! {
            message = channel.recv() => {
                has_changes = true;
                if let Some(callback) = message {
                    let db = db.clone();
                    tokio::task::spawn_blocking(move || {
                        let _guard =
                            tracing::info_span!("logs:flush_every_second:buffer").entered();
                        let result = db.persist(PersistMode::Buffer)
                            // fjall::Error isn't Clone
                            .map_err(|err| {
                                tracing::error!(?err, "error flushing fjall in background");
                                BackgroundFsyncFailedError(err.to_string())
                            });
                        callback.log_io_completed(result.map_err(std::io::Error::other));
                    })
                    .await
                    .expect("failed joining blocking task");
                } else {
                    done = true;
                }
            },
            _ = shutting_down.cancelled() => {
                done = true
            },
            _ = ticker.tick() => {
                if has_changes {
                    let db = db.clone();
                    tokio::task::spawn_blocking(move || {
                        let _guard =
                            tracing::info_span!("logs:flush_every_second:flush").entered();
                        tracing::debug!("running periodic sync of logs");
                        if let Err(err) = db.persist(PersistMode::SyncAll) {
                            tracing::error!(?err, "error flushing fjall");
                        }
                    })
                    .await
                    .expect("failed joining blocking task");
                    has_changes = false
                } else {
                    tracing::trace!("no changes in the last interval, doing nothing");
                }
            }
        }
    }
    if let Err(err) = db.persist(PersistMode::SyncAll) {
        tracing::error!(?err, "error flushing fjall at shutdown");
    }
}

impl DiomLogs {
    pub fn new(path: Dir, fsync_mode: FsyncMode) -> anyhow::Result<Self> {
        let pb: std::path::PathBuf = path.into();
        let db = Database::builder(&pb).worker_threads(1).open()?;
        let log_keyspace = db.keyspace("cluster:logs", || {
            KeyspaceCreateOptions::default()
                .manual_journal_persist(true)
                .expect_point_read_hits(true)
        })?;
        let meta_keyspace = db.keyspace("cluster:meta", KeyspaceCreateOptions::default)?;
        let (flush_tx, flush_rx) = tokio::sync::mpsc::channel(1000);
        if fsync_mode == FsyncMode::EveryCommit {
            tokio::spawn(flush_every_commit(db.clone(), flush_rx));
        } else {
            tokio::spawn(flush_every_second(db.clone(), flush_rx));
        }
        Ok(Self {
            db,
            log_keyspace,
            meta_keyspace,
            flush_tx,
            log_cache: LogCache::new(100),
        })
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn record_log_timestamp(
        &self,
        timestamp: Timestamp,
        log_index: u64,
    ) -> anyhow::Result<()> {
        let rec = LogIndex {
            unix_timestamp_millis: timestamp.as_millisecond() as u64,
            log_id: log_index,
        };
        tracing::debug!(?rec, "recording log/timestamp checkpoint");
        let (k, v) = rec.to_fjall_entry()?;
        let keyspace = self.log_keyspace.clone();
        spawn_blocking_in_current_span(move || -> fjall::Result<()> { keyspace.insert(k, v) })
            .await??;
        Ok(())
    }

    /// Get the NodeId (or, if we don't have one, make a new one)
    pub async fn get_node_id(&mut self) -> anyhow::Result<NodeId> {
        let db = self.db.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            if let Some(node_id) = NODE_ID.get(&meta_keyspace)? {
                tracing::debug!(?node_id, "starting up with existing node ID");
                node_id
            } else {
                let node_id = NodeId::generate();
                tracing::info!(?node_id, "generated a new node ID");
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
        entries: Vec<Entry<TypeConfig>>,
        callback: LogFlushed<TypeConfig>,
    ) -> anyhow::Result<()> {
        Span::current().record("num_entries", entries.len());

        let keyspace = self.log_keyspace.clone();
        let mut batch = fjall::OwnedWriteBatch::with_capacity(self.db.clone(), entries.len())
            .durability(Some(PersistMode::Buffer));
        let persisted_entries = entries.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            let _guard = tracing::info_span!("append:write_entries").entered();
            for entry in persisted_entries {
                let (k, v) = Log(entry).to_fjall_entry()?;
                batch.insert(&keyspace, k, v);
            }
            batch.commit()?;
            Ok(())
        })
        .await??;

        self.flush_tx
            .send(callback)
            .await
            .context("requesting background fsync")?;

        for entry in entries {
            self.log_cache.push(entry);
        }

        Ok(())
    }

    /// Truncate logs since log_id, inclusive
    async fn truncate_entries_(&self, log_id: LogId<NodeId>) -> anyhow::Result<()> {
        self.log_cache.truncate(log_id.index);
        let log_keyspace = self.log_keyspace.clone();
        let mut tx = self.db.batch().durability(Some(PersistMode::Buffer));
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            for key in Log::keys_in_range(&log_keyspace, log_id.index..)? {
                tx.remove(&log_keyspace, key);
            }
            tx.commit()?;
            Ok(())
        })
        .await?
    }

    /// Purge logs upto log_id, inclusive
    async fn purge_entries_(&self, log_id: LogId<NodeId>) -> anyhow::Result<()> {
        self.log_cache.purge(log_id.index);
        let meta_keyspace = self.meta_keyspace.clone();
        let log_keyspace = self.log_keyspace.clone();
        let mut tx = self.db.batch().durability(Some(PersistMode::Buffer));
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            for key in Log::keys_in_range(&log_keyspace, ..=log_id.index)? {
                tx.remove(&log_keyspace, key);
            }
            LAST_PURGED_LOG_ID.store_tx(&mut tx, &meta_keyspace, &log_id)?;
            tx.commit()?;
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
        spawn_blocking_in_current_span(move || {
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
        .await?
    }

    async fn save_vote_(&self, vote: Vote<NodeId>) -> anyhow::Result<()> {
        tracing::trace!(?vote, "saving a vote");
        let db = self.db.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || {
            VOTE.store(&meta_keyspace, &vote)?;
            tracing::info_span!("save_vote:persist")
                .in_scope(|| db.persist(PersistMode::SyncAll))?;
            Ok(())
        })
        .await?
    }

    async fn read_vote_(&self) -> anyhow::Result<Option<Vote<NodeId>>> {
        let keyspace = self.meta_keyspace.clone();
        let Some(vote) = spawn_blocking_in_current_span(move || VOTE.get(&keyspace)).await?? else {
            tracing::trace!("couldn't find a vote");
            return Ok(None);
        };
        tracing::trace!(?vote, "read a vote");
        Ok(Some(vote))
    }

    async fn save_committed_(&self, committed: Option<LogId<NodeId>>) -> anyhow::Result<()> {
        let meta_keyspace = self.meta_keyspace.clone();
        tracing::trace!(?committed, "saving committed state");
        spawn_blocking_in_current_span(move || COMMITTED.store(&meta_keyspace, &committed))
            .await?
            .context("saving committed state")
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId<NodeId>>> {
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
}

fn spawn_blocking_in_current_span<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> tokio::task::JoinHandle<T> {
    let current_span = Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

#[cfg(test)]
mod tests {
    use super::DiomLogs;
    use crate::cfg::Dir;
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
            let logs = DiomLogs::new(logdir, crate::cfg::FsyncMode::default()).unwrap();
            Self {
                _workdir: workdir,
                logs,
            }
        }
    }

    #[tokio::test]
    async fn test_log_timestamps() -> TestResult {
        let context = TestContext::new();
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
