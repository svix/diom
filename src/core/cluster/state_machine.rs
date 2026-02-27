use std::{
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};

use crate::{cfg::Dir, core::db::Databases};
use anyhow::Context;
use diom_namespace::entities::StorageType;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use fjall_utils::{FjallFixedKey, ReadonlyKeyspace};
use openraft::{
    EntryPayload, LogId, RaftSnapshotBuilder, RaftTypeConfig, Snapshot, SnapshotMeta,
    StorageIOError, StoredMembership, storage::RaftStateMachine,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tap::TapOptional;
use tokio::sync::RwLock as TokioRwLock;
use uuid::Uuid;

use super::{
    Node, NodeId, errors::*, handle::Response, logs::DiomLogs, raft::TypeConfig,
    serialized_state_machine,
};
use crate::AppState;

type StorageError = openraft::StorageError<NodeId>;
type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LastSnapshot {
    meta: SnapshotMeta<NodeId, Node>,
    path: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct ClusterId(#[serde(with = "uuid::serde::simple")] Uuid);

impl ClusterId {
    pub(super) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone)]
pub struct StoreHandle(Arc<TokioRwLock<Store>>);

/// The actual meat of the database; a wrapper around fjall state
/// and any number of module states
pub struct Stores {
    pub databases: Databases,
    pub stream_state: stream_deprecated::State,
    pub msgs_state: diom_msgs::State,
}

impl From<Store> for StoreHandle {
    fn from(value: Store) -> Self {
        Self(Arc::new(TokioRwLock::new(value)))
    }
}

/// The raft store; has to encapsulate all stored state
pub struct Store {
    pub(super) state: AppState,
    // This is wrapped an an RwLock (even though the StoreHandle has its own RwLock)
    // because it gets sent to `tokio::task::spawn_blocking` invocations all over the place,
    // and it's possible that we could get snapshot-unsafe behavior if one of them outlived
    // the lock on the outer structure, so we only actually lock it later; the lock should
    // almost never be contended unless we're dropping futures.
    //
    // The only thing that gets a write lock to it is taking and applying a snapshot; everything
    // else only needs a read lock.
    stores: Arc<RwLock<Stores>>,
    snapshot_directory: PathBuf,
    meta_keyspace: Keyspace,
    readonly_meta_keyspace: ReadonlyKeyspace,
    snapshot_idx: u64,
    last_applied_log_id: Option<LogId<NodeId>>,
    last_membership: StoredMembership<NodeId, Node>,
    last_snapshot: Arc<RwLock<Option<LastSnapshot>>>,
    cluster_id: Option<ClusterId>,
    pub(super) logs: DiomLogs,
}

trait SnapshotIdx {
    fn snapshot_idx(&self) -> anyhow::Result<u64>;
}

impl SnapshotIdx for SnapshotMeta<NodeId, Node> {
    fn snapshot_idx(&self) -> anyhow::Result<u64> {
        let last = self
            .snapshot_id
            .split('-')
            .next_back()
            .ok_or_else(|| anyhow::anyhow!("invalid snapshot id {}", self.snapshot_id))?;
        last.parse()
            .map_err(|e| anyhow::anyhow!("snapshot id not a u64: {e}"))
    }
}

const METADATA_KEYSPACE: &str = "_raft_metadata";

static LAST_APPLIED_LOG_ID: FjallFixedKey<LogId<NodeId>> =
    FjallFixedKey::new("last_applied_log_id");
static LAST_SNAPSHOT: FjallFixedKey<LastSnapshot> = FjallFixedKey::new("last_snapshot");
static LAST_MEMBERSHIP: FjallFixedKey<StoredMembership<NodeId, Node>> =
    FjallFixedKey::new("last_membership");
static CLUSTER_UUID: FjallFixedKey<ClusterId> = FjallFixedKey::new("cluster_uuid");

impl Store {
    pub async fn new(
        persistent_db: Database,
        ephemeral_db: Database,
        snapshot_directory: Dir,
        app_state: AppState,
        logs: DiomLogs,
    ) -> anyhow::Result<Self> {
        let meta_keyspace =
            persistent_db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let stream_state = stream_deprecated::State::init(persistent_db.clone())
            .context("initializing stream state")?;
        let msgs_state =
            diom_msgs::State::init(persistent_db.clone()).context("initializing msgs state")?;

        let databases = Databases::new(persistent_db, ephemeral_db);

        let stores = Stores {
            databases,
            stream_state,
            msgs_state,
        };

        let mut this = Self {
            stores: Arc::new(RwLock::new(stores)),
            state: app_state,
            snapshot_directory: snapshot_directory.into(),
            readonly_meta_keyspace: ReadonlyKeyspace::from(meta_keyspace.clone()),
            meta_keyspace,
            last_snapshot: Arc::new(RwLock::new(None)),
            snapshot_idx: 0,
            last_applied_log_id: None,
            last_membership: Default::default(),
            cluster_id: None,
            logs,
        };
        this.load_information().await?;
        Ok(this)
    }

    pub fn cluster_id(&self) -> Option<&ClusterId> {
        self.cluster_id.as_ref()
    }

    pub fn db_handle(&self) -> impl std::ops::Deref<Target = Stores> {
        self.stores.read_arc()
    }

    pub(super) async fn set_cluster_id(&mut self, id: ClusterId) -> anyhow::Result<()> {
        let keyspace = self.meta_keyspace.clone();
        tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
            CLUSTER_UUID.store(&keyspace, &id)?;
            Ok(())
        })
        .await??;
        self.cluster_id = Some(id);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn load_information(&mut self) -> anyhow::Result<()> {
        let keyspace = self.readonly_meta_keyspace.clone();

        let (last_applied_log_id, last_membership, last_snapshot, cluster_id) =
            tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
                let last_applied_log_id = LAST_APPLIED_LOG_ID.get(&keyspace)?;
                let last_membership = LAST_MEMBERSHIP
                    .get(&keyspace)?
                    .tap_none(|| tracing::trace!("found no last membership in the database!"))
                    .unwrap_or_default();
                let last_snapshot = LAST_SNAPSHOT.get(&keyspace)?;
                let cluster_id = CLUSTER_UUID.get(&keyspace)?;

                Ok((
                    last_applied_log_id,
                    last_membership,
                    last_snapshot,
                    cluster_id,
                ))
            })
            .await??;
        self.last_applied_log_id = last_applied_log_id;
        self.last_membership = last_membership;
        self.snapshot_idx = last_snapshot
            .as_ref()
            .map_or(Ok(0), |s| s.meta.snapshot_idx())?;
        self.cluster_id = cluster_id;
        *self.last_snapshot.write() = last_snapshot;
        Ok(())
    }

    async fn set_last_snapshot_(
        &mut self,
        meta: SnapshotMeta<NodeId, Node>,
        snapshot_path: PathBuf,
    ) -> anyhow::Result<()> {
        let handle = self.stores.clone();
        let keyspace = self.meta_keyspace.clone();
        self.snapshot_idx = std::cmp::max(self.snapshot_idx + 1, meta.snapshot_idx()?);
        let data = LastSnapshot {
            meta,
            path: snapshot_path,
        };
        tracing::trace!(last_snapshot=?data, "setting last_snapshot");
        let data = tokio::task::spawn_blocking(move || -> anyhow::Result<LastSnapshot> {
            let db = &handle.read().databases.persistent;
            LAST_SNAPSHOT.store(&keyspace, &data)?;
            db.persist(PersistMode::SyncAll)?;
            Ok(data)
        })
        .await??;
        *self.last_snapshot.write() = Some(data);
        Ok(())
    }

    async fn record_ids_(&mut self) -> anyhow::Result<()> {
        tracing::trace!(
            last_applied_log_id=?self.last_applied_log_id,
            last_membership=?self.last_membership,
            "storing id values");
        let handle = self.stores.clone();
        let meta_keyspace = self.meta_keyspace.clone();
        let last_applied_log_id = self.last_applied_log_id;
        let last_membership = self.last_membership.clone();
        tokio::task::spawn_blocking(move || {
            let mut tx = handle
                .read()
                .databases
                .persistent
                .batch()
                .durability(Some(PersistMode::Buffer));
            if let Some(log_id) = &last_applied_log_id {
                LAST_APPLIED_LOG_ID.store_tx(&mut tx, &meta_keyspace, log_id)?;
            } else {
                LAST_APPLIED_LOG_ID.remove_tx(&mut tx, &meta_keyspace)?;
            }
            LAST_MEMBERSHIP.store_tx(&mut tx, &meta_keyspace, &last_membership)?;
            tx.commit()?;
            Ok(())
        })
        .await?
    }

    async fn delete_unused_snapshots(&self, keep_path: &Path) -> anyhow::Result<()> {
        tracing::debug!("cleaning up unused snapshots");
        let mut dents = tokio::fs::read_dir(&self.snapshot_directory).await?;
        while let Some(dent) = dents.next_entry().await? {
            if let Some(preserve_path) = keep_path.file_name() {
                if dent.file_name() == preserve_path {
                    tracing::trace!(filename=?dent.file_name(), "preserving used snapshot");
                    continue;
                }
            } else {
                tracing::warn!(path=?keep_path, "very weird snapshot path");
            }
            tracing::debug!(filename=?dent.file_name(), "deleting unused snapshot");
            tokio::fs::remove_file(dent.path()).await?;
        }
        Ok(())
    }

    async fn install_snapshot_(
        &mut self,
        meta: &SnapshotMeta<NodeId, Node>,
        snapshot: Box<StoredSnapshot>,
    ) -> anyhow::Result<()> {
        tracing::debug!("starting snapshot installation");
        let mut f = snapshot.file.into_std().await;
        let handle = self.stores.clone();
        tokio::task::spawn_blocking(move || {
            let stores = handle.write();
            serialized_state_machine::load_from_file(&stores.databases, &mut f)
        })
        .await??;
        self.last_applied_log_id = meta.last_log_id;
        self.last_membership = meta.last_membership.clone();
        self.record_ids_().await?;
        self.delete_unused_snapshots(snapshot.path.as_path())
            .await?;
        self.set_last_snapshot_(meta.clone(), snapshot.path).await?;
        Ok(())
    }

    async fn apply_<I>(&mut self, entries: I) -> StorageResult<Vec<Response>>
    where
        I: IntoIterator<Item = <TypeConfig as RaftTypeConfig>::Entry> + openraft::OptionalSend,
        I::IntoIter: openraft::OptionalSend,
    {
        let mut replies = vec![];
        let mut changed_log_id = false;
        let mut changed_membership = false;
        for item in entries {
            self.last_applied_log_id = Some(item.log_id);
            changed_log_id = true;

            match item.payload {
                EntryPayload::Blank => {
                    tracing::trace!("heartbeat");
                    replies.push(Response::Blank)
                }
                EntryPayload::Normal(req) => {
                    tracing::trace!(log_id=?item.log_id, request=?req, "applying user request");
                    let reply = match super::applier::apply_request(req, self, item.log_id).await {
                        Ok(o) => o,
                        Err(e) => {
                            tracing::error!("failed to apply raft log");
                            return Err(StorageError::IO {
                                source: StorageIOError::apply(item.log_id, e),
                            });
                        }
                    };
                    replies.push(reply);
                }
                EntryPayload::Membership(last_membership) => {
                    tracing::trace!("changing cluster membership");
                    self.last_membership =
                        StoredMembership::new(Some(item.log_id), last_membership);
                    changed_membership = true;
                    replies.push(Response::Blank)
                }
            }
        }
        if changed_log_id || changed_membership {
            self.record_ids_().await.map_err(write_err)?;
        }
        Ok(replies)
    }

    async fn begin_receiving_snapshot_(
        &mut self,
    ) -> StorageResult<Box<<TypeConfig as RaftTypeConfig>::SnapshotData>> {
        let path = self
            .snapshot_directory
            .with_file_name(format!("diom-incoming-snapshot-{}", self.snapshot_idx));
        self.snapshot_idx += 1;
        let f = tokio::fs::File::create_new(path.clone())
            .await
            .map_err(|e| write_snapshot_err(&e))?;
        Ok(Box::new(StoredSnapshot { path, file: f }))
    }

    fn prep_snapshot_builder_(&mut self) {
        self.snapshot_idx += 1;
    }

    #[tracing::instrument(skip(self))]
    async fn get_current_snapshot_(&mut self) -> StorageResult<Option<Snapshot<TypeConfig>>> {
        // clone to avoid holding a lock over an await point
        let last_snapshot = self.last_snapshot.read().clone();
        if let Some(last_snapshot) = last_snapshot {
            tracing::trace!(?last_snapshot, "found last_snapshot");
            let f = tokio::fs::File::open(&last_snapshot.path)
                .await
                .map_err(|e| read_snapshot_err(&e))?;
            Ok(Some(Snapshot {
                meta: last_snapshot.meta.clone(),
                snapshot: Box::new(StoredSnapshot {
                    file: f,
                    path: last_snapshot.path.clone(),
                }),
            }))
        } else {
            tracing::trace!("found no last_snapshot");
            Ok(None)
        }
    }

    async fn build_snapshot_(&mut self) -> StorageResult<Snapshot<TypeConfig>> {
        let last_log_id = self.last_applied_log_id;
        let last_membership = self.last_membership.clone();

        let snapshot_id = if let Some(last) = last_log_id {
            format!("{}-{}-{}", last.leader_id, last.index, self.snapshot_idx)
        } else {
            format!("x-x-{}", self.snapshot_idx)
        };

        let meta = SnapshotMeta {
            last_log_id,
            last_membership,
            snapshot_id,
        };

        let handle = self.stores.clone();

        fn list_keyspaces(db: &Database) -> Vec<String> {
            db.list_keyspace_names()
                .into_iter()
                .filter(|s| s.as_bytes() != METADATA_KEYSPACE.as_bytes())
                .map(|s| s.to_string())
                .collect()
        }

        let targets = tokio::task::spawn_blocking(move || {
            let store = handle.write();
            let dbs = &store.databases;

            vec![
                (
                    StorageType::Persistent,
                    dbs.persistent.clone(),
                    dbs.persistent.snapshot(),
                    list_keyspaces(&dbs.persistent),
                ),
                (
                    StorageType::Ephemeral,
                    dbs.ephemeral.clone(),
                    dbs.ephemeral.snapshot(),
                    list_keyspaces(&dbs.ephemeral),
                ),
            ]
        })
        .await
        .map_err(|err| write_snapshot_err(anyhow::anyhow!(err)))?;

        let snapshot = StoredSnapshot::new(&meta, &self.snapshot_directory, targets)
            .await
            .map_err(write_snapshot_err)?;

        self.set_last_snapshot_(meta.clone(), snapshot.path.clone())
            .await
            .map_err(write_snapshot_err)?;

        let snapshot = Box::new(snapshot);

        Ok(Snapshot { meta, snapshot })
    }
}

// Wrapper around a snapshot that has been written to disk and has both a filename
// and a concrete File
pub struct StoredSnapshot {
    file: tokio::fs::File,
    path: PathBuf,
}

impl std::fmt::Debug for StoredSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StoredSnapshot {{ path: {:?} }}", self.path)
    }
}

impl tokio::io::AsyncWrite for StoredSnapshot {
    fn is_write_vectored(&self) -> bool {
        self.file.is_write_vectored()
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_shutdown(cx)
    }

    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.file).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<std::io::Result<usize>> {
        Pin::new(&mut self.file).poll_write_vectored(cx, bufs)
    }
}

impl tokio::io::AsyncRead for StoredSnapshot {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        Pin::new(&mut self.file).poll_read(cx, buf)
    }
}

impl tokio::io::AsyncSeek for StoredSnapshot {
    fn poll_complete(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<u64>> {
        Pin::new(&mut self.file).poll_complete(cx)
    }

    fn start_seek(mut self: Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        Pin::new(&mut self.file).start_seek(position)
    }
}

impl StoredSnapshot {
    async fn new(
        metadata: &SnapshotMeta<NodeId, Node>,
        directory: &Path,
        targets: Vec<(StorageType, Database, fjall::Snapshot, Vec<String>)>,
    ) -> anyhow::Result<Self> {
        let file_name = format!("diom-{}", metadata.snapshot_id);
        let path = directory.with_file_name(file_name);
        let path_c = path.clone();
        let file = tokio::task::spawn_blocking(move || -> anyhow::Result<std::fs::File> {
            tracing::info!(path=%path_c.display(), "writing snapshot");
            let mut f = std::fs::File::create(path_c)?;
            serialized_state_machine::serialize_to_file(targets, &mut f)?;
            f.seek(SeekFrom::Start(0))?;
            Ok(f)
        })
        .await??;
        let file = tokio::fs::File::from_std(file);
        Ok(Self { file, path })
    }
}

impl RaftSnapshotBuilder<TypeConfig> for StoreHandle {
    async fn build_snapshot(&mut self) -> StorageResult<Snapshot<TypeConfig>> {
        self.0.write().await.build_snapshot_().await
    }
}

impl RaftStateMachine<TypeConfig> for StoreHandle {
    type SnapshotBuilder = Self;

    async fn applied_state(
        &mut self,
    ) -> StorageResult<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>)> {
        let this = self.0.read().await;
        Ok((this.last_applied_log_id, this.last_membership.clone()))
    }

    async fn apply<I>(&mut self, entries: I) -> StorageResult<Vec<Response>>
    where
        I: IntoIterator<Item = <TypeConfig as RaftTypeConfig>::Entry> + openraft::OptionalSend,
        I::IntoIter: openraft::OptionalSend,
    {
        self.0.write().await.apply_(entries).await
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> StorageResult<Box<<TypeConfig as RaftTypeConfig>::SnapshotData>> {
        self.0.write().await.begin_receiving_snapshot_().await
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.0.write().await.prep_snapshot_builder_();
        self.clone()
    }

    #[tracing::instrument(skip(self))]
    async fn get_current_snapshot(&mut self) -> StorageResult<Option<Snapshot<TypeConfig>>> {
        self.0.write().await.get_current_snapshot_().await
    }

    #[tracing::instrument(skip(self, meta))]
    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<NodeId, Node>,
        snapshot: Box<StoredSnapshot>,
    ) -> StorageResult<()> {
        self.0
            .write()
            .await
            .install_snapshot_(meta, snapshot)
            .await
            .map_err(read_snapshot_err)
    }
}

impl StoreHandle {
    pub async fn cluster_id(&self) -> Option<ClusterId> {
        self.0.read().await.cluster_id().copied()
    }
}
