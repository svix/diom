use anyhow::Context;
use byteorder::{BigEndian, ByteOrder};
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode, Readable, Slice, UserKey};
use openraft::storage::{LogFlushed, RaftLogStorage};
use openraft::{
    Entry, LogId, OptionalSend, RaftLogId, RaftLogReader, RaftTypeConfig, StorageError, Vote,
};
use std::fmt::Debug;
use std::ops::{Bound, RangeBounds};
use std::path::Path;
use tap::{Pipe, Tap, TapFallible};
use tracing::{Instrument as _, Span};

use super::NodeId;
use super::errors::*;
use super::raft::TypeConfig;

// This is an implementation of an openraft Logs store backed by fjall

#[derive(Debug, PartialEq, Clone)]
enum LogError {
    KeyDeserializationError,
}

impl std::error::Error for LogError {
    fn description(&self) -> &str {
        "Internal error processing logs"
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: fix this
        write!(f, "{self:?}")
    }
}

type StorageResult<T> = Result<T, StorageError<NodeId>>;

#[derive(Clone)]
pub struct CoyoteLogs {
    db: Database,
    meta_keyspace: Keyspace,
    log_keyspace: Keyspace,
}

type KeyType = [u8; 8];

fn index_to_key(index: u64) -> KeyType {
    let mut buf = [0u8; 8];
    BigEndian::write_u64(&mut buf, index);
    buf
}

fn key_to_index(key: Slice) -> Result<u64, LogError> {
    if key.len() != 8 {
        return Err(LogError::KeyDeserializationError);
    }
    Ok(BigEndian::read_u64(&key))
}

fn range_to_start<RB: RangeBounds<u64>>(range: RB) -> KeyType {
    match range.start_bound() {
        Bound::Included(i) => index_to_key(*i),
        Bound::Excluded(i) => index_to_key(*i + 1),
        Bound::Unbounded => index_to_key(0),
    }
}

impl<C> RaftLogReader<C> for CoyoteLogs
where
    C: RaftTypeConfig,
{
    #[tracing::instrument(skip(self))]
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + OptionalSend>(
        &mut self,
        range: RB,
    ) -> Result<Vec<C::Entry>, StorageError<C::NodeId>> {
        let output = self
            .read_log_entries::<C, RB>(range.clone())
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

impl CoyoteLogs {
    pub async fn new(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let db = Database::builder(path).worker_threads(1).open()?;
        let log_keyspace = db.keyspace("cluster:logs", KeyspaceCreateOptions::default)?;
        let meta_keyspace = db.keyspace("cluster:meta", KeyspaceCreateOptions::default)?;
        Ok(Self {
            db,
            log_keyspace,
            meta_keyspace,
        })
    }

    /// Get the NodeId (or, if we don't have one, make a new one)
    pub async fn get_node_id(&mut self) -> anyhow::Result<NodeId> {
        let meta_keyspace = self.meta_keyspace.clone();
        let db = self.db.clone();
        tokio::task::spawn_blocking(move || {
            if let Some(raw_node_id) = meta_keyspace
                .get("node_id")
                .context("fetching node ID from logs database")?
            {
                let node_id = rmp_serde::from_slice(&raw_node_id)?;
                tracing::debug!(?node_id, "starting up with existing node ID");
                node_id
            } else {
                let node_id = NodeId::generate();
                tracing::info!(?node_id, "generated a new node ID");
                meta_keyspace
                    .insert("node_id", rmp_serde::to_vec(&node_id)?)
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
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let keys = entries.iter().map(|entry| entry.log_id).collect::<Vec<_>>();
            tracing::trace!(?keys, "appending some entries");
            let key_vs = entries
                .into_iter()
                .map(|entry| (index_to_key(entry.log_id.index), entry));
            let mut ingest = keyspace.start_ingestion()?;
            for (id, raw_value) in key_vs {
                let value = rmp_serde::to_vec(&raw_value)?;
                ingest.write(id, value)?;
            }
            ingest.finish()?;
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
            let keys: Vec<UserKey> = log_keyspace
                .range(index_to_key(log_id.index)..)
                .map(|g| g.key())
                .collect::<Result<Vec<_>, fjall::Error>>()?;
            tracing::trace!(?keys, ?log_id, "deleting items after log_id (inclusive)");
            for key in keys {
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
        let serialized_log_id = rmp_serde::encode::to_vec(&log_id)?;
        let mut tx = self.db.batch().durability(Some(PersistMode::SyncAll));
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            let keys: Vec<UserKey> = log_keyspace
                .range(..=index_to_key(log_id.index))
                .map(|g| g.key())
                .collect::<Result<Vec<_>, fjall::Error>>()?;
            tracing::trace!(?keys, ?log_id, "deleting items before log_id (inclusive)");
            for key in keys {
                tx.remove(&log_keyspace, key);
            }
            tx.insert(&meta_keyspace, "last_purged_log_id", serialized_log_id);
            tx.commit()?;
            Ok(())
        })
        .await?
    }

    fn trace_logs(&self) {
        tracing::trace!("BEGINNING LOG TRACE");
        let last_purged_log_id: Option<LogId<NodeId>> = self
            .meta_keyspace
            .get("last_purged_log_id")
            .unwrap()
            .map(|l| rmp_serde::from_slice(&l).unwrap());
        let first_id = self
            .log_keyspace
            .first_key_value()
            .map(|g| key_to_index(g.key().unwrap()).unwrap());
        let last_id = self
            .log_keyspace
            .last_key_value()
            .map(|g| key_to_index(g.key().unwrap()).unwrap());
        tracing::trace!(?last_purged_log_id, first_id, last_id, "log metadata");
        let tx = self.db.snapshot();
        for guard in tx.iter(&self.log_keyspace) {
            let (key, value) = guard.into_inner().unwrap();
            let value: Entry<TypeConfig> = rmp_serde::from_slice(&value).unwrap();
            let index = key_to_index(key).unwrap();
            tracing::trace!(?value, ?index, "log");
        }
        tracing::trace!("END LOG TRACE");
    }

    async fn get_log_state_(&mut self) -> anyhow::Result<openraft::LogState<TypeConfig>> {
        let db = self.db.clone();
        let log_keyspace = self.log_keyspace.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || {
            let tx = db.snapshot();
            let last_purged_log_id =
                if let Some(value) = tx.get(&meta_keyspace, "last_purged_log_id")? {
                    Some(rmp_serde::from_slice(&value)?)
                } else {
                    None
                };
            let last_log_id = if let Some(last_guard) = tx.last_key_value(&log_keyspace) {
                let raw_entry = last_guard.value()?;
                let value: <TypeConfig as RaftTypeConfig>::Entry =
                    rmp_serde::from_slice(&raw_entry)?;
                Some(value.log_id)
            } else {
                last_purged_log_id
            };
            Ok(openraft::LogState {
                last_purged_log_id,
                last_log_id,
            })
        })
        .await?
    }

    async fn read_log_entries<C, RB>(&mut self, range: RB) -> anyhow::Result<Vec<C::Entry>>
    where
        C: RaftTypeConfig,
        RB: RangeBounds<u64> + Clone + Debug + OptionalSend,
    {
        let db = self.db.clone();
        let log_keyspace = self.log_keyspace.clone();
        let iter_range = range_to_start(range.clone())..;
        // why isn't RB always Send? it's a goddamn range...
        let end = match range.end_bound() {
            Bound::Unbounded => None,
            Bound::Included(i) => Some(*i + 1),
            Bound::Excluded(i) => Some(*i),
        };
        tokio::task::spawn_blocking(move || {
            let mut output = vec![];
            let tx = db.snapshot();
            for guard in tx.range(&log_keyspace, iter_range) {
                let (key, value) = guard
                    .into_inner()
                    .tap_err(|err| tracing::warn!(?err, "Error reading values from log"))?;
                let index = key_to_index(key)
                    .tap_err(|err| tracing::warn!(?err, "Error parsing key from log"))?;
                if let Some(end) = end
                    && index >= end
                {
                    break;
                }
                let value = rmp_serde::from_slice(&value)?;
                output.push(value);
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
            let serialized = rmp_serde::to_vec(&vote)?;
            meta_keyspace.insert("vote", serialized)?;
            db.persist(PersistMode::SyncAll)?;
            Ok(())
        })
        .await?
    }

    async fn read_vote_(&self) -> anyhow::Result<Option<Vote<NodeId>>> {
        let Some(raw) = self.meta_keyspace.get("vote")? else {
            tracing::trace!("couldn't find a vote");
            return Ok(None);
        };
        let vote = rmp_serde::from_slice(&raw)?;
        tracing::trace!(?vote, "reading a vote");
        Ok(Some(vote))
    }

    async fn save_committed_(&self, committed: Option<LogId<NodeId>>) -> anyhow::Result<()> {
        let meta_keyspace = self.meta_keyspace.clone();
        tracing::trace!(?committed, "saving committed state");
        tokio::task::spawn_blocking(move || {
            let serialized = rmp_serde::to_vec(&committed)?;
            meta_keyspace.insert("committed", serialized)?;
            Ok(())
        })
        .await?
    }

    async fn read_committed_(&self) -> anyhow::Result<Option<LogId<NodeId>>> {
        let meta_keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || {
            let committed = meta_keyspace
                .get("committed")?
                .map(|c| rmp_serde::from_slice(&c))
                .transpose()?;
            tracing::trace!(?committed, "read committed state");
            Ok(committed)
        })
        .await?
    }
}
