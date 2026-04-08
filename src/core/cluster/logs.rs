use std::{
    collections::BTreeMap,
    fmt::Debug,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use coyote_core::{instrumented_mutex::InstrumentedMutex, task::spawn_blocking_in_current_span};
use jiff::Timestamp;
use openraft::{
    OptionalSend, RaftLogReader, RaftTypeConfig,
    storage::{IOFlushed, RaftLogStorage},
    type_config::alias::{LogIdOf, VoteOf},
};
use parking_lot::Mutex;
use serde::{Serialize, de::DeserializeOwned};
use tap::{Pipe, Tap, TapOptional};
use tracing::Span;

use super::{NodeId, raft::TypeConfig};
use crate::{
    cfg::Dir,
    core::{cluster::ClusterId, metrics::LogMetrics},
};

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

fn io_err(error: anyhow::Error) -> std::io::Error {
    std::io::Error::other(error)
}

impl RaftLogReader<TypeConfig> for CoyoteLogs {
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

impl RaftLogStorage<TypeConfig> for CoyoteLogs {
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

struct SqliteFixedKey<T: Serialize + DeserializeOwned> {
    key: &'static str,
    data: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> SqliteFixedKey<T> {
    const fn new(key: &'static str) -> Self {
        Self {
            key,
            data: PhantomData,
        }
    }

    fn get<TX>(&self, tx: &TX) -> anyhow::Result<Option<T>>
    where
        TX: std::ops::Deref<Target = rusqlite::Connection>,
    {
        let row = match tx.query_row(
            "SELECT value FROM metadata WHERE key = ?1",
            [self.key],
            |row| -> rusqlite::Result<Box<[u8]>, _> { row.get(0) },
        ) {
            Ok(row) => row,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(e.into()),
        };
        let row = rmp_serde::from_slice(&row)?;
        Ok(Some(row))
    }

    fn store_tx<TX>(&self, tx: &TX, value: &T) -> anyhow::Result<()>
    where
        TX: std::ops::Deref<Target = rusqlite::Connection>,
    {
        let serialized = rmp_serde::to_vec_named(value)?;
        tx.execute("INSERT INTO metadata(key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value", (self.key, serialized))?;
        Ok(())
    }
}

static NODE_ID: SqliteFixedKey<NodeId> = SqliteFixedKey::new("node_id");
static LAST_PURGED_LOG_ID: SqliteFixedKey<LogId> = SqliteFixedKey::new("last_purged_log_id");
static VOTE: SqliteFixedKey<Vote> = SqliteFixedKey::new("vote");
static COMMITTED: SqliteFixedKey<Option<LogId>> = SqliteFixedKey::new("committed");
static POISONED: SqliteFixedKey<ClusterId> = SqliteFixedKey::new("poisoned");

#[derive(Clone)]
pub struct CoyoteLogs {
    db: InstrumentedMutex<rusqlite::Connection>,
    log_cache: LogCache,
    metrics: Option<LogMetrics>,
    last_vote: Arc<Mutex<Option<Vote>>>,
}

async fn spawn_blocking_with_db<T, F, O>(
    label: &'static str,
    db: &InstrumentedMutex<T>,
    f: F,
) -> anyhow::Result<O>
where
    T: Send + 'static,
    F: FnOnce(&mut T) -> anyhow::Result<O> + Send + 'static,
    O: Send + 'static,
{
    let db = db.clone();
    spawn_blocking_in_current_span(move || {
        let mut db = db.lock(label);
        f(&mut db)
    })
    .await
    .expect("failed to join thread")
}

impl CoyoteLogs {
    pub fn new(path: Dir, synchronous: bool) -> anyhow::Result<Self> {
        let path = path.as_path().join("logs.sqlite");
        let db = rusqlite::Connection::open(path)?;

        db.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS logs(
                log_index UNSIGNED BIGINT NOT NULL PRIMARY KEY,
                entry BLOB NOT NULL
            );
            CREATE TABLE IF NOT EXISTS log_timestamps(
                timestamp UNSIGNED BIGINT NOT NULL PRIMARY KEY,
                log_index UNSIGNED BIGINT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_log_timestamps_on_log_index
                ON log_timestamps(log_index);
            CREATE TABLE IF NOT EXISTS metadata(
                key TEXT NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
            );
            "#,
        )
        .context("initializing schema")?;
        db.pragma_update(None, "journal_mode", "WAL")
            .context("enabling WAL mode")?;
        if synchronous {
            db.pragma_update(None, "synchronous", "full")
                .context("enabling 'full' synchronous mode")?;
        } else {
            db.pragma_update(None, "synchronous", "normal")
                .context("enabling 'normal' synchronous mode")?;
        }
        db.pragma_update(None, "journal_size_limit", (100 * 1000 * 1000).to_string())
            .context("increasing max journal size")?;

        Ok(Self {
            db: InstrumentedMutex::new(db),
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
        spawn_blocking_with_db("record_log_timestamp", &self.db, move |db| {
            tracing::debug!(?timestamp, log_index, "storing log timestamp");
            let tx = db.transaction().context("creating transaction to append")?;
            tx.execute(
                "INSERT INTO log_timestamps(timestamp, log_index) VALUES (?1, ?2)",
                (timestamp.as_millisecond(), log_index as i64),
            )?;
            tx.commit()?;
            Ok(())
        })
        .await
    }

    /// Get the NodeId (or, if we don't have one, make a new one)
    pub async fn get_node_id(&mut self) -> anyhow::Result<NodeId> {
        spawn_blocking_with_db("get_node_id", &self.db, move |db| {
            if let Some(node_id) = NODE_ID.get(&db)? {
                tracing::info!(%node_id, "starting up with existing node ID");
                node_id
            } else {
                let node_id = NodeId::generate();
                tracing::info!(%node_id, "generated a new node ID");
                let tx = db.transaction()?;
                NODE_ID
                    .store_tx(&tx, &node_id)
                    .context("saving node ID to logs database")?;
                tx.commit()?;
                node_id
            }
            .pipe(Ok)
        })
        .await
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

        let persisted_entries = entries.clone();
        spawn_blocking_with_db("append_entries", &self.db, move |db| {
            let _guard = tracing::info_span!("append:write_entries").entered();
            let tx = db.transaction().context("creating transaction to append")?;
            for entry in persisted_entries {
                tracing::trace!(log_index = entry.log_id.index, "appending an entry");
                let serialized = rmp_serde::to_vec_named(&entry).context("serializing log")?;
                tx.execute(
                    "INSERT INTO logs(log_index, entry) VALUES(?1, ?2)",
                    (entry.log_id.index as i64, serialized),
                )
                .context("appending log")?;
            }
            tx.commit()?;
            Ok(())
        })
        .await?;

        callback.io_completed(Ok(()));

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

        spawn_blocking_with_db("truncate_entries", &self.db, move |db| {
            tracing::trace!(?log_id, "truncating entries after (exclusive)");
            let txn = db.transaction().context("creating truncate transaction")?;
            txn.execute("DELETE FROM logs WHERE log_index >= ?1", [start as i64])
                .context("truncating logs")?;
            txn.execute(
                "DELETE FROM log_timestamps WHERE log_index >= ?1",
                [start as i64],
            )
            .context("truncating log indexes")?;
            txn.commit().context("committing truncate transaction")?;
            Ok(())
        })
        .await
    }

    /// Purge logs upto log_id, inclusive
    async fn purge_entries_(&self, log_id: LogId) -> anyhow::Result<()> {
        self.log_cache.purge(log_id.index);
        spawn_blocking_with_db("purge_entries", &self.db, move |db| {
            tracing::trace!(?log_id, "truncating entries before (inclusive)");
            let txn = db.transaction().context("creating purge transaction")?;
            txn.execute(
                "DELETE FROM logs WHERE log_index <= ?1",
                [log_id.index as i64],
            )
            .context("purging logs")?;
            txn.execute(
                "DELETE FROM log_timestamps WHERE log_index <= ?1",
                [log_id.index as i64],
            )
            .context("purging log indexes")?;
            LAST_PURGED_LOG_ID.store_tx(&txn, &log_id)?;
            txn.commit().context("committing purge transaction")?;
            Ok(())
        })
        .await
    }

    async fn get_log_state_(&mut self) -> anyhow::Result<openraft::LogState<TypeConfig>> {
        spawn_blocking_with_db("get_log_state", &self.db, move |db| {
            let last_purged_log_id = LAST_PURGED_LOG_ID.get(&db)?;
            let entry = match db.query_row(
                "SELECT entry FROM logs ORDER BY log_index DESC LIMIT 1",
                [],
                |r| -> rusqlite::Result<Box<[u8]>, _> { r.get(0) },
            ) {
                Ok(r) => Some(rmp_serde::from_slice::<LogEntry>(&r)?),
                Err(rusqlite::Error::QueryReturnedNoRows) => None,
                Err(e) => return Err(e.into()),
            };
            let last_log_id = if let Some(entry) = entry {
                Some(entry.log_id)
            } else {
                last_purged_log_id
            };
            Ok(openraft::LogState {
                last_purged_log_id,
                last_log_id,
            })
        })
        .await
        .tap(|state| tracing::trace!(?state, "read initial log state"))
    }

    async fn read_log_entries<RB>(&mut self, range: RB) -> anyhow::Result<Vec<LogEntry>>
    where
        RB: RangeBounds<u64> + Clone + Debug + OptionalSend,
    {
        // the most common case is that we just wrote a log entry in append_entries_ and now we're
        // reading it out to apply it. we don't need to go to disk for that!
        match (range.start_bound(), range.end_bound()) {
            (Bound::Included(i), Bound::Excluded(j)) if i + 1 == *j => {
                if let Some(entry) = self.log_cache.get(i) {
                    tracing::trace!("short-circuiting for single-log read");
                    return Ok(vec![entry]);
                }
            }
            _ => {}
        }

        // For some reason, RB isn't specified as Send in the trait, so we can't
        // use it directly across the boundary. ARGH!
        let lower_bound = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(i) => *i,
            Bound::Excluded(i) => i.saturating_add(1),
        };
        let upper_bound = match range.end_bound() {
            Bound::Unbounded => i64::MAX as u64,
            Bound::Included(i) => i.saturating_add(1),
            Bound::Excluded(i) => *i,
        };
        let value: Vec<LogEntry> = spawn_blocking_with_db("read_log_entries", &self.db, move |db| {
            tracing::trace!(lower_bound, upper_bound, "reading log entries from underlying db");
            let mut stmt =
                db.prepare("SELECT entry FROM logs WHERE log_index >= ?1 AND log_index < ?2 ORDER BY log_index ASC")?;
            stmt.query_map(
                    [lower_bound as i64, upper_bound as i64],
                    |row| -> rusqlite::Result<Box<[u8]>, _> { row.get(0) },
                )
                .context("reading log IDs")?
                .map(|data| rmp_serde::from_slice(&data?).context("unable to parse data as msgpack"))
                .collect()
        })
        .await?;
        self.metric_record(|m| m.record_log_read(value.len()));
        Ok(value)
    }

    async fn save_vote_(&self, vote: Vote) -> anyhow::Result<()> {
        spawn_blocking_with_db("save_vote", &self.db, move |db| {
            tracing::trace!(?vote, "saving a vote");
            VOTE.store_tx(&db, &vote)
        })
        .await
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
        let Some(vote) =
            spawn_blocking_with_db("read_vote", &self.db, move |db| VOTE.get(&db)).await?
        else {
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
        spawn_blocking_with_db("save_committed", &self.db, move |db| {
            tracing::trace!(?committed, "saving committed state");
            COMMITTED.store_tx(&db, &committed)
        })
        .await
        .context("saving committed state")
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId>> {
        spawn_blocking_with_db("read_committed", &self.db, move |db| {
            COMMITTED
                .get(&db)?
                .tap_some(|committed| tracing::trace!(?committed, "read committed state"))
                .flatten()
                .pipe(Ok)
        })
        .await
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
                    move || {
                        let db = db.lock("get_metrics");
                        db.query_row("SELECT SUM(\"pgsize\")", [], |r| -> rusqlite::Result<i64> {
                            r.get(0)
                        })
                    }
                })
                .await
                .expect("Failed joining blocking task")
                {
                    Ok(bytes) => metrics.bytes_used(bytes as _),
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
        spawn_blocking_with_db("log_index_before", &self.db, move |db| {
            tracing::debug!(timestamp=%ts, "looking for highest index before");
            match db.query_row(
                "SELECT log_index FROM log_timestamps WHERE timestamp < ?1 ORDER BY timestamp DESC LIMIT 1",
                [ts.as_millisecond()],
                |r| -> rusqlite::Result<i64, _> { r.get(0) },
            ) {
                Ok(r) => Ok(Some(r as u64)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
        .await
    }

    /// Return the highest log index that we know occurred at or after the given timestamp,
    pub async fn log_index_after(&self, ts: Timestamp) -> anyhow::Result<Option<u64>> {
        spawn_blocking_with_db("log_index_after", &self.db, move |db| {
            tracing::debug!(timestamp=%ts, "looking for highest index after");
            match db.query_row(
                "SELECT log_index FROM log_timestamps WHERE timestamp >= ?1 ORDER BY timestamp ASC LIMIT 1",
                [ts.as_millisecond()],
                |r| -> rusqlite::Result<i64, _> { r.get(0) },
            ) {
                Ok(r) => Ok(Some(r as u64)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
        .await
    }

    pub(super) async fn get_last_timestamp(&self) -> anyhow::Result<Option<Timestamp>> {
        spawn_blocking_with_db("get_last_timestamp", &self.db, move |db| {
            tracing::debug!("looking for last timestamp");
            match db.query_row(
                "SELECT timestamp FROM log_timestamps ORDER BY timestamp DESC LIMIT 1",
                [],
                |r| -> rusqlite::Result<i64, _> { r.get(0) },
            ) {
                Ok(r) => Timestamp::from_millisecond(r)?.pipe(|ts| Ok(Some(ts))),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
        .await
    }

    pub(crate) async fn poison(&self, cluster_id: ClusterId) -> anyhow::Result<()> {
        spawn_blocking_with_db("poison", &self.db, move |db| {
            POISONED.store_tx(&db, &cluster_id)
        })
        .await
        .context("saving poisoned state")
    }

    pub(crate) async fn is_poisoned(&self) -> anyhow::Result<bool> {
        spawn_blocking_with_db("is_poisoned", &self.db, move |db| {
            POISONED
                .get(&db)?
                .tap_some(|cluster_id| {
                    tracing::error!(?cluster_id, "this node was previously poisoned")
                })
                .is_some()
                .pipe(Ok)
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::CoyoteLogs;
    use crate::cfg::Dir;
    use jiff::{Span, Timestamp};
    use tempfile::TempDir;
    use test_utils::TestResult;

    struct TestContext {
        _workdir: TempDir,
        logs: CoyoteLogs,
    }

    impl TestContext {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let logdir = Dir::new(&workdir).unwrap();
            let logs = CoyoteLogs::new(logdir, false).unwrap();
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
