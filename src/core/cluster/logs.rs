use std::{
    collections::VecDeque,
    fmt::Debug,
    ops::{Bound, RangeBounds},
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use diom_derive::PersistableValue;
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
use tokio_util::sync::CancellationToken;
use tracing::Span;

use super::{NodeId, raft::TypeConfig};
use crate::{
    cfg::{Dir, FsyncMode, SyncMode},
    core::{cluster::ClusterId, metrics::LogMetrics},
};
use diom_core::task::spawn_blocking_in_current_span;
use diom_error::Result;

// This is an implementation of an openraft Logs store backed by fjall

type LogEntry = <TypeConfig as RaftTypeConfig>::Entry;
type LogId = LogIdOf<TypeConfig>;
type Vote = VoteOf<TypeConfig>;
type IoFlushedCallback = IOFlushed<TypeConfig>;

#[derive(Debug)]
struct LogCacheInner {
    inner: VecDeque<LogEntry>,
    capacity: usize,
    // log_id.index of inner[0], if the deque is non-empty
    base_index: Option<u64>,
}

impl LogCacheInner {
    fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
            base_index: None,
        }
    }

    fn push(&mut self, entry: LogEntry) {
        if self.inner.len() >= self.capacity {
            self.inner.pop_front();
            self.base_index = self.inner.front().map(|e| e.log_id.index);
        }
        if self.base_index.is_none() {
            self.base_index = Some(entry.log_id.index);
        }
        self.inner.push_back(entry);
    }

    fn purge(&mut self, log_index: u64) {
        while let Some(front) = self.inner.front()
            && front.log_id.index <= log_index
        {
            self.inner.pop_front();
        }

        self.base_index = self.inner.front().map(|e| e.log_id.index);
    }

    fn truncate(&mut self, log_index: u64) {
        while let Some(back) = self.inner.back()
            && back.log_id.index >= log_index
        {
            self.inner.pop_back();
        }
        if self.inner.is_empty() {
            self.base_index = None;
        }
    }

    fn get(&self, log_index: &u64) -> Option<&LogEntry> {
        let base = self.base_index?;
        let offset = log_index.checked_sub(base)? as usize;
        let ret = self.inner.get(offset);
        debug_assert!(ret.is_none_or(|e| e.log_id.index == *log_index));
        ret
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

// this one right here officer, this is the bad one
impl diom_core::persistable_value::PersistableValue for Log {}

impl TableRow for Log {
    const ROW_TYPE: u8 = RowType::Log as u8;
}

impl MonotonicTableRow for Log {
    type KeyType = u64;

    fn get_key(&self) -> u64 {
        self.0.log_id.index
    }
}

#[derive(Debug, Serialize, Deserialize, PersistableValue)]
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

impl std::error::Error for BackgroundFsyncFailedError {}

struct SimpleEstimator<const COUNT: usize> {
    samples: [Option<Duration>; COUNT],
    count: usize,
}

impl<const COUNT: usize> SimpleEstimator<COUNT> {
    fn new(initial: Duration) -> Self {
        let mut samples = [None; COUNT];
        samples[0] = Some(initial);
        Self { samples, count: 1 }
    }

    fn push(&mut self, sample: Duration) {
        self.samples[self.count % 5] = Some(sample);
        self.count += 1;
    }

    fn estimate(&self) -> Duration {
        let count = self.samples.iter().filter(|x| x.is_some()).count() as u64;
        if count == 0 {
            panic!("this is impossible, there's always at least one count")
        }
        let sum: u64 = self
            .samples
            .iter()
            .filter_map(|o| o.map(|d| d.as_micros().try_into().unwrap_or(u64::MAX)))
            .sum();

        Duration::from_micros(sum / count)
    }
}

enum FlushMessage {
    EnableMetrics(LogMetrics),
    Callback(IoFlushedCallback),
}

struct FlushDebouncer {
    interval: Duration,
    deadline: Option<tokio::time::Instant>,
}

impl FlushDebouncer {
    fn new(interval: Duration) -> Self {
        Self {
            interval,
            deadline: None,
        }
    }

    fn enable(&mut self) {
        if self.deadline.is_none() {
            tracing::trace!(interval=?self.interval, "debouncing log fsync");
            self.deadline = Some(tokio::time::Instant::now() + self.interval)
        }
    }

    async fn wait(&mut self) {
        if let Some(deadline) = self.deadline {
            tokio::time::sleep_until(deadline).await;
        } else {
            std::future::pending().await
        }
        self.deadline.take();
    }
}

struct FlushWorker {
    db: Database,
    rx: tokio::sync::mpsc::Receiver<FlushMessage>,
    commits_before_fsync: usize,
    autoscale_duration: bool,
    duration_estimator: SimpleEstimator<7>,
    persist_mode: PersistMode,
    shutting_down: CancellationToken,
    pending: Vec<IoFlushedCallback>,
    metrics: Option<LogMetrics>,
    done: bool,
    debouncer: FlushDebouncer,
}

impl FlushWorker {
    fn new(
        db: Database,
        rx: tokio::sync::mpsc::Receiver<FlushMessage>,
        commits_before_fsync: usize,
        duration_before_fsync: Duration,
        autoscale_duration: bool,
        sync_mode: SyncMode,
        fsync_mode: FsyncMode,
        shutting_down: CancellationToken,
    ) -> Self {
        let persist_mode = sync_mode.into_persist_mode(fsync_mode);
        tracing::debug!(
            ?commits_before_fsync,
            ?duration_before_fsync,
            ?persist_mode,
            "initializing background flush worker"
        );
        Self {
            db,
            rx,
            commits_before_fsync,
            autoscale_duration,
            persist_mode,
            shutting_down,
            pending: Vec::new(),
            duration_estimator: SimpleEstimator::<7>::new(duration_before_fsync),
            metrics: None,
            done: false,
            debouncer: FlushDebouncer::new(duration_before_fsync),
        }
    }

    fn handle_message(&mut self, message: FlushMessage) {
        match message {
            FlushMessage::EnableMetrics(new_metrics) => {
                tracing::info!("enabling metrics in background flush worker");
                self.metrics = Some(new_metrics)
            }
            FlushMessage::Callback(callback) => {
                self.pending.push(callback);
                if self.commits_before_fsync != 1 {
                    self.debouncer.enable();
                }
            }
        }
    }

    async fn run(mut self) {
        while !self.done {
            let synced = self.run_one_loop().await;
            if self.autoscale_duration && synced && self.commits_before_fsync != 1 {
                self.update_fsync_estimate();
            }
        }
        if let Err(err) = self.db.persist(self.persist_mode) {
            tracing::error!(?err, "error flushing fjall at shutdown");
        }
    }

    #[tracing::instrument("logs:flush_worker", skip_all)]
    async fn run_one_loop(&mut self) -> bool {
        const FLUSH_BUF: usize = 10;
        let mut buf = Vec::with_capacity(FLUSH_BUF);
        let mut sync_from_ticker = false;
        buf.clear();

        tokio::select! {
            num_messages = self.rx.recv_many(&mut buf, FLUSH_BUF) => {
                if num_messages == 0 {
                    self.done = true
                } else {
                    for message in buf.drain(..) {
                        self.handle_message(message);
                    }
                }
            },
            _ = self.shutting_down.cancelled() => {
                self.done = true
            },
            _ = self.debouncer.wait() => {
                tracing::trace!("flushing after debounce");
                sync_from_ticker = true
            }
        }

        if self.pending.is_empty() {
            return false;
        }

        let sync_from_count =
            self.commits_before_fsync > 0 && self.pending.len() >= self.commits_before_fsync;

        if sync_from_count {
            tracing::trace!("flushing after count");
        }

        if sync_from_ticker || sync_from_count {
            let db = self.db.clone();
            let num_commits = self.pending.len();
            let persist_mode = self.persist_mode;
            let result = spawn_blocking_in_current_span(
                move || -> Result<Duration, BackgroundFsyncFailedError> {
                    let _guard =
                        tracing::info_span!("logs:flush_worker:flush", num_commits).entered();
                    tracing::trace!(?persist_mode, "flushing logs to disk");
                    let start_persist = Instant::now();
                    db.persist(persist_mode).map_err(|err| {
                        tracing::error!(?err, "error flushing fjall");
                        BackgroundFsyncFailedError(err.to_string())
                    })?;
                    let persist_time = start_persist.elapsed();
                    Ok(persist_time)
                },
            )
            .await
            .expect("failed joining blocking task")
            .map(|persist_time| {
                self.duration_estimator.push(persist_time);
                if let Some(metrics) = &self.metrics {
                    metrics.record_fsync(persist_time, self.pending.len());
                }
            });
            tracing::trace!(num_pending = self.pending.len(), "committed for some items");
            {
                let _guard = tracing::info_span!("logs:flush_worker:drain").entered();
                for callback in self.pending.drain(..) {
                    callback.io_completed(result.clone().map_err(std::io::Error::other))
                }
            }
        }
        true
    }

    fn update_fsync_estimate(&mut self) {
        let new_estimate = self.duration_estimator.estimate();
        if new_estimate < Duration::from_micros(1) {
            tracing::trace!(?new_estimate, "ignoring obviously bogus fsync time")
        } else if new_estimate.abs_diff(self.debouncer.interval) > Duration::from_micros(100) {
            // only update when it changed significantly so we're not tearing down and
            // recreating the tokio timer all the time
            tracing::trace!(
                last_estimate = ?self.debouncer.interval,
                ?new_estimate,
                "updating fsync time estimate"
            );
            self.debouncer.interval = new_estimate;
        }
    }
}

struct PurgeWorker {
    rx: tokio::sync::mpsc::Receiver<(Instant, LogId)>,
    db: Database,
    log_keyspace: Keyspace,
}

impl PurgeWorker {
    const DELETE_BATCH_SIZE: usize = 10_000;

    fn new(
        db: Database,
        log_keyspace: Keyspace,
        rx: tokio::sync::mpsc::Receiver<(Instant, LogId)>,
    ) -> Self {
        Self {
            db,
            log_keyspace,
            rx,
        }
    }

    async fn run(mut self) {
        while let Some((start_time, log_id)) = self.rx.recv().await {
            if let Err(err) = self.purge_one(start_time, log_id).await {
                tracing::error!(?err, "error while purging logs");
            }
        }
        tracing::debug!("purge worker shutting down");
    }

    async fn purge_one(&self, start: Instant, log_id: LogId) -> anyhow::Result<()> {
        // precondition; the LAST_PURGED_LOG_ID has already been set
        let log_keyspace = self.log_keyspace.clone();
        let db = self.db.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            let fjall_start = Instant::now();
            // do the very slow purge. If we crash here, we might leak some rows
            // in the database, but they'll be cleared on the next purge since
            // we always start at 0.
            let deleted = Log::remove_keys_in_range(
                &db,
                &log_keyspace,
                ..=log_id.index,
                Self::DELETE_BATCH_SIZE,
                PersistMode::Buffer,
            )?;
            let fjall_purge_time = fjall_start.elapsed();
            let total_purge_time = start.elapsed();
            tracing::debug!(
                ?total_purge_time,
                ?fjall_purge_time,
                deleted,
                "deleted entries for purge"
            );
            Ok(())
        })
        .await?
    }
}

#[derive(Clone)]
pub struct DiomLogs {
    db: Database,
    meta_keyspace: Keyspace,
    log_keyspace: Keyspace,
    flush_tx: tokio::sync::mpsc::Sender<FlushMessage>,
    purge_tx: tokio::sync::mpsc::Sender<(Instant, LogId)>,
    purged_index: Option<u64>,
    log_cache: LogCache,
    metrics: Option<LogMetrics>,
    last_vote: Arc<Mutex<Option<Vote>>>,
    fsync_mode: FsyncMode,
    cancellation_token: CancellationToken,
}

impl DiomLogs {
    const DELETE_BATCH_SIZE: usize = 10_000;

    pub async fn new(
        path: Dir,
        commits_before_fsync: usize,
        duration_before_fsync: Duration,
        autoscale_duration: bool,
        sync_mode: SyncMode,
        fsync_mode: FsyncMode,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let pb: std::path::PathBuf = path.into();
        let db = Database::builder(&pb).worker_threads(1).open()?;
        let log_keyspace = db.keyspace("cluster:logs", || {
            KeyspaceCreateOptions::default()
                .manual_journal_persist(true)
                .expect_point_read_hits(true)
        })?;
        let meta_keyspace = db.keyspace("cluster:meta", || {
            KeyspaceCreateOptions::default()
                .manual_journal_persist(true)
                .expect_point_read_hits(true)
        })?;
        let (flush_tx, flush_rx) = tokio::sync::mpsc::channel(65536);
        let flush_worker = FlushWorker::new(
            db.clone(),
            flush_rx,
            commits_before_fsync,
            duration_before_fsync,
            autoscale_duration,
            sync_mode,
            fsync_mode,
            cancellation_token.clone(),
        );
        tokio::spawn(flush_worker.run());
        let (purge_tx, purge_rx) = tokio::sync::mpsc::channel(2);
        let purge_worker = PurgeWorker::new(db.clone(), log_keyspace.clone(), purge_rx);
        tokio::spawn(purge_worker.run());
        let mut this = Self {
            db,
            log_keyspace,
            meta_keyspace,
            flush_tx,
            purge_tx,
            purged_index: None,
            log_cache: LogCache::new(100),
            metrics: None,
            last_vote: Arc::new(Mutex::new(None)),
            fsync_mode,
            cancellation_token,
        };
        let state = this.get_log_state().await?;
        this.purged_index = state.last_purged_log_id.map(|l| l.index);
        Ok(this)
    }

    async fn read_metadata<T: Serialize + serde::de::DeserializeOwned + Send + Sync + 'static>(
        &self,
        key: &'static FjallFixedKey<T>,
    ) -> anyhow::Result<Option<T>> {
        let keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || key.get(&keyspace))
            .await?
            .with_context(|| format!("reading metadata for {}", key.key))
    }

    async fn save_metadata<T: Serialize + serde::de::DeserializeOwned + Send + Sync + 'static>(
        &self,
        key: &'static FjallFixedKey<T>,
        value: T,
        sync: bool,
    ) -> anyhow::Result<()> {
        let keyspace = self.meta_keyspace.clone();
        let db = self.db.clone();
        let durability = if sync {
            self.fsync_mode.into()
        } else {
            PersistMode::Buffer
        };
        spawn_blocking_in_current_span(move || {
            let mut batch = db.batch().durability(Some(durability));
            key.store_tx(&mut batch, &keyspace, &value)?;
            batch.commit()?;
            Ok(())
        })
        .await?
    }

    pub(crate) async fn enable_metrics(&mut self, metrics: LogMetrics) -> anyhow::Result<()> {
        self.metrics = Some(metrics.clone());
        self.flush_tx
            .send(FlushMessage::EnableMetrics(metrics.clone()))
            .await?;
        self.start_metrics(metrics);
        Ok(())
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
        if let Some(node_id) = self.read_metadata(&NODE_ID).await? {
            tracing::info!(%node_id, "starting up with existing node ID");
            node_id
        } else {
            let node_id = NodeId::generate();
            tracing::info!(%node_id, "generated a new node ID");
            self.save_metadata(&NODE_ID, node_id, true).await?;
            node_id
        }
        .pipe(Ok)
    }

    #[tracing::instrument(skip_all, fields(num_entries))]
    async fn append_entries_(
        &mut self,
        entries: Vec<LogEntry>,
        callback: IOFlushed<TypeConfig>,
    ) -> anyhow::Result<()> {
        Span::current().record("num_entries", entries.len());
        let start = Instant::now();
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
            .send(FlushMessage::Callback(callback))
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
    async fn purge_entries_(&mut self, log_id: LogId) -> anyhow::Result<()> {
        tracing::debug!(?log_id, "scheduling background purge of logs");

        let start = Instant::now();

        // synchronously purge the cache and set the flag
        self.log_cache.purge(log_id.index);
        self.purged_index = Some(log_id.index);

        // first, set LAST_PURGED_LOG_ID so that if we crash and restart, we'll
        // ignore any IDs that have been purged
        spawn_blocking_in_current_span({
            let db = self.db.clone();
            let meta_keyspace = self.meta_keyspace.clone();
            move || -> anyhow::Result<()> {
                let mut tx = db.batch().durability(Some(PersistMode::Buffer));
                LAST_PURGED_LOG_ID.store_tx(&mut tx, &meta_keyspace, &log_id)?;
                tx.commit()?;
                Ok(())
            }
        })
        .await??;

        // now run the rest in a background task
        self.purge_tx.send((start, log_id)).await?;
        Ok(())
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

        let purged_index = self.purged_index.unwrap_or(0);

        // For some reason, RB isn't specified as Send in the trait, so we can't
        // use it directly across the boundary. ARGH!
        let send_range = match range.start_bound() {
            Bound::Unbounded => purged_index..,
            Bound::Included(i) => (*i).max(purged_index)..,
            Bound::Excluded(i) => (*i + 1).max(purged_index)..,
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
        self.save_metadata(&VOTE, vote, true).await?;
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
        let Some(vote) = self.read_metadata(&VOTE).await? else {
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
        tracing::trace!(?committed, "saving committed state");
        self.save_metadata(&COMMITTED, committed, false).await
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId>> {
        self.read_metadata(&COMMITTED)
            .await?
            .tap_some(|committed| tracing::trace!(?committed, "read committed state"))
            .flatten()
            .pipe(Ok)
    }

    fn start_metrics(&self, metrics: LogMetrics) {
        let mut logs = self.clone();
        let db = self.db.clone();
        let shutdown = self.cancellation_token.clone();
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
                    return Some(req.timestamp.into());
                }
            }
            None
        })
        .await
        .context("failed to join")
    }

    pub(crate) async fn poison(&self, cluster_id: ClusterId) -> anyhow::Result<()> {
        self.save_metadata(&POISONED, cluster_id, true).await
    }

    pub(crate) async fn is_poisoned(&self) -> anyhow::Result<bool> {
        self.read_metadata(&POISONED).await.map(|c| {
            c.is_some_and(|cluster_id| {
                tracing::error!(?cluster_id, "this node was previously poisoned");
                true
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::DiomLogs;
    use crate::cfg::{Dir, FsyncMode, SyncMode};
    use diom_error::CanFailExt;
    use jiff::{Span, Timestamp};
    use tempfile::TempDir;
    use test_utils::TestResult;
    use tokio_util::sync::CancellationToken;

    struct TestContext {
        workdir: Option<TempDir>,
        logs: DiomLogs,
        cancellation_token: CancellationToken,
    }

    impl TestContext {
        async fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let logdir = Dir::new(&workdir).unwrap();
            let token = CancellationToken::new();
            let logs = DiomLogs::new(
                logdir,
                0,
                Duration::from_hours(1),
                false,
                SyncMode::Buffer,
                FsyncMode::default(),
                token.clone(),
            )
            .await
            .unwrap();
            Self {
                workdir: Some(workdir),
                logs,
                cancellation_token: token,
            }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            self.cancellation_token.cancel();
            if let Some(workdir) = self.workdir.take() {
                workdir.close().can_fail("failure to close workdir")
            }
        }
    }

    #[tokio::test]
    async fn test_log_timestamps() -> TestResult {
        let context = TestContext::new().await;
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
