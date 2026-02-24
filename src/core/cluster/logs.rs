use std::{
    borrow::Cow,
    fmt::Debug,
    ops::{Bound, RangeBounds},
    path::Path,
};

use anyhow::Context;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use fjall_utils::{FjallFixedKey, MonotonicTableRowExt, TableRow};
use jiff::Timestamp;
use openraft::{
    Entry, LogId, OptionalSend, RaftLogId, RaftLogReader, RaftTypeConfig, StorageError, Vote,
    storage::{LogFlushed, RaftLogStorage},
};
use serde::{Deserialize, Serialize};
use tap::{Pipe, Tap, TapFallible, TapOptional};
use tracing::{Instrument as _, Span};

use super::{NodeId, errors::*, raft::TypeConfig};

// This is an implementation of an openraft Logs store backed by fjall

type StorageResult<T> = Result<T, StorageError<NodeId>>;

type LogEntry = <TypeConfig as RaftTypeConfig>::Entry;

#[derive(Clone)]
pub struct CoyoteLogs {
    db: Database,
    meta_keyspace: Keyspace,
    log_keyspace: Keyspace,
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

impl RaftLogReader<TypeConfig> for CoyoteLogs {
    #[tracing::instrument(skip(self))]
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + OptionalSend>(
        &mut self,
        range: RB,
    ) -> Result<Vec<LogEntry>, StorageError<NodeId>> {
        let output = self
            .read_log_entries::<RB>(range.clone())
            .await
            .map_err(read_logs_err)?;
        let output_keys = output.iter().map(|e| e.get_log_id());
        tracing::trace!(?range, ?output_keys, "read log entries");
        Ok(output)
    }
}

impl RaftLogStorage<TypeConfig> for CoyoteLogs {
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
            .instrument(Span::current())
            .await
            .map_err(write_vote_err)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        self.read_vote_()
            .instrument(Span::current())
            .await
            .map_err(read_vote_err)
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
            .instrument(Span::current())
            .await
            .map_err(write_logs_err)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn truncate(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        self.truncate_entries_(log_id)
            .instrument(Span::current())
            .await
            .map_err(write_logs_err)
            .tap(|_| self.trace_logs())
    }

    #[tracing::instrument(skip(self))]
    async fn purge(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        self.purge_entries_(log_id)
            .instrument(Span::current())
            .await
            .map_err(write_logs_err)
            .tap(|_| self.trace_logs())
    }

    #[tracing::instrument(skip(self))]
    async fn save_committed(&mut self, committed: Option<LogId<NodeId>>) -> StorageResult<()> {
        self.save_committed_(committed)
            .instrument(Span::current())
            .await
            .map_err(write_err)
    }

    #[tracing::instrument(skip(self))]
    async fn read_committed(&mut self) -> Result<Option<LogId<NodeId>>, StorageError<NodeId>> {
        self.read_committed_()
            .instrument(Span::current())
            .await
            .map_err(write_err)
    }
}

static NODE_ID: FjallFixedKey<NodeId> = FjallFixedKey::new("node_id");
static LAST_PURGED_LOG_ID: FjallFixedKey<LogId<NodeId>> = FjallFixedKey::new("last_purged_log_id");
static VOTE: FjallFixedKey<Vote<NodeId>> = FjallFixedKey::new("vote");
static COMMITTED: FjallFixedKey<Option<LogId<NodeId>>> = FjallFixedKey::new("committed");

impl CoyoteLogs {
    pub fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let db = Database::builder(path).worker_threads(1).open()?;
        let log_keyspace = db.keyspace("cluster:logs", KeyspaceCreateOptions::default)?;
        let meta_keyspace = db.keyspace("cluster:meta", KeyspaceCreateOptions::default)?;
        Ok(Self {
            db,
            log_keyspace,
            meta_keyspace,
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
        tokio::task::spawn_blocking(move || -> fjall::Result<()> { keyspace.insert(k, v) })
            .await??;
        Ok(())
    }

    /// Get the NodeId (or, if we don't have one, make a new one)
    pub async fn get_node_id(&mut self) -> anyhow::Result<NodeId> {
        let db = self.db.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || {
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

    async fn append_entries_(
        &mut self,
        entries: Vec<Entry<TypeConfig>>,
        callback: LogFlushed<TypeConfig>,
    ) -> anyhow::Result<()> {
        let keyspace = self.log_keyspace.clone();
        let mut batch = self.db.batch().durability(Some(PersistMode::Buffer));
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let keys = entries.iter().map(|entry| entry.log_id).collect::<Vec<_>>();
            tracing::trace!(?keys, "appending some entries");
            for entry in entries {
                let (k, v) = Log(entry).to_fjall_entry()?;
                batch.insert(&keyspace, k, v);
            }
            batch.commit()?;
            Ok(())
        })
        .await??;

        // callback should be called after writing the entries but before syncing them; ok
        callback.log_io_completed(Ok(()));

        let db = self.db.clone();
        tokio::task::spawn_blocking(move || db.persist(PersistMode::SyncAll)).await??;

        Ok(())
    }

    /// Truncate logs since log_id, inclusive
    async fn truncate_entries_(&self, log_id: LogId<NodeId>) -> anyhow::Result<()> {
        let log_keyspace = self.log_keyspace.clone();
        let mut tx = self.db.batch().durability(Some(PersistMode::SyncAll));
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
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
        let meta_keyspace = self.meta_keyspace.clone();
        let log_keyspace = self.log_keyspace.clone();
        let mut tx = self.db.batch().durability(Some(PersistMode::SyncAll));
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            for key in Log::keys_in_range(&log_keyspace, ..=log_id.index)? {
                tx.remove(&log_keyspace, key);
            }
            LAST_PURGED_LOG_ID.store_tx(&mut tx, &meta_keyspace, &log_id)?;
            tx.commit()?;
            Ok(())
        })
        .await?
    }

    fn trace_logs(&self) {
        tracing::trace!("BEGINNING LOG TRACE");
        let last_purged_log_id = LAST_PURGED_LOG_ID.get(&self.meta_keyspace);
        let first_id = Log::range(&self.log_keyspace, ..)
            .next()
            .map(|l| l.unwrap().0);
        let last_id = Log::range(&self.log_keyspace, ..)
            .next_back()
            .map(|l| l.unwrap().0);
        tracing::trace!(?last_purged_log_id, first_id, last_id, "log metadata");
        for row in Log::range(&self.log_keyspace, ..) {
            let (index, value) = row.unwrap();
            tracing::trace!(?index, ?value, "log");
        }
        tracing::trace!("END LOG TRACE");
    }

    async fn get_log_state_(&mut self) -> anyhow::Result<openraft::LogState<TypeConfig>> {
        let log_keyspace = self.log_keyspace.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || {
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
        tokio::task::spawn_blocking(move || {
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
        tokio::task::spawn_blocking(move || {
            VOTE.store(&meta_keyspace, &vote)?;
            db.persist(PersistMode::SyncAll)?;
            Ok(())
        })
        .await?
    }

    async fn read_vote_(&self) -> anyhow::Result<Option<Vote<NodeId>>> {
        let keyspace = self.meta_keyspace.clone();
        let Some(vote) = tokio::task::spawn_blocking(move || VOTE.get(&keyspace)).await?? else {
            tracing::trace!("couldn't find a vote");
            return Ok(None);
        };
        tracing::trace!(?vote, "read a vote");
        Ok(Some(vote))
    }

    async fn save_committed_(&self, committed: Option<LogId<NodeId>>) -> anyhow::Result<()> {
        let meta_keyspace = self.meta_keyspace.clone();
        tracing::trace!(?committed, "saving committed state");
        tokio::task::spawn_blocking(move || COMMITTED.store(&meta_keyspace, &committed))
            .await?
            .context("saving committed state")
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId<NodeId>>> {
        let meta_keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || {
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
        tokio::task::spawn_blocking(move || {
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
        tokio::task::spawn_blocking(move || {
            if let Some(row) = LogIndex::range(&log_keyspace, range).next() {
                Ok(Some(row?.1.log_id))
            } else {
                Ok(None)
            }
        })
        .await?
    }
}

#[cfg(test)]
mod tests {
    use super::CoyoteLogs;
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
            let logs = CoyoteLogs::new(&workdir).unwrap();
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
