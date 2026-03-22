use std::{
    collections::HashMap,
    io::{Seek, SeekFrom},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::Duration,
};

use crate::{
    cfg::Dir,
    core::metrics::{DbMetrics, DbType},
};
use anyhow::Context;
use diom_core::{Monotime, task::spawn_blocking_in_current_span};
use diom_namespace::entities::StorageType;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, OwnedWriteBatch, PersistMode};
use fjall_utils::{Databases, FjallFixedKey, ReadonlyKeyspace};
use openraft::{
    EntryPayload, LogId, RaftSnapshotBuilder, RaftTypeConfig, Snapshot, SnapshotMeta,
    StorageIOError, StoredMembership, storage::RaftStateMachine,
};
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tap::TapOptional;
use tokio::sync::RwLock as TokioRwLock;
use uuid::Uuid;

use super::{
    Node, NodeId,
    errors::*,
    handle::{Request, RequestWithContext, Response},
    logs::DiomLogs,
    raft::TypeConfig,
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

impl std::fmt::Display for ClusterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.simple().fmt(f)
    }
}

impl ClusterId {
    pub(super) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl JsonSchema for ClusterId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone)]
pub struct StoreHandle {
    inner: Arc<TokioRwLock<Store>>,
    pub time: Monotime,
}

/// The actual meat of the database; a wrapper around fjall state
/// and any number of module states
pub struct Stores {
    pub databases: Databases,
    pub msgs_state: diom_msgs::State,
    pub kv_state: diom_kv::State,
    pub cache_state: diom_cache::State,
    pub idempotency_state: diom_idempotency::State,
    pub rate_limit_state: diom_rate_limit::State,
    pub auth_token_state: diom_auth_token::State,
}

impl From<Store> for StoreHandle {
    fn from(value: Store) -> Self {
        let time = value.time.clone();
        Self {
            inner: Arc::new(TokioRwLock::new(value)),
            time,
        }
    }
}

/// The raft store; has to encapsulate all stored state
pub struct Store {
    pub(super) state: AppState,
    // This is wrapped an an RwLock (even though the StoreHandle has its own RwLock)
    // because it gets sent to `spawn_blocking` invocations all over the place,
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
    pub(super) time: Monotime,
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
        node_id: NodeId,
        time: Monotime,
    ) -> anyhow::Result<Self> {
        let meta_keyspace =
            persistent_db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let databases = Databases::new(persistent_db.clone(), ephemeral_db);

        let stores = Stores {
            databases: databases.clone(),
            msgs_state: diom_msgs::State::init(persistent_db.clone())
                .context("initializing msgs state")?,
            kv_state: diom_kv::State::init(databases.clone()).context("initializing kv state")?,
            cache_state: diom_cache::State::init(databases.clone())
                .context("initializing cache state")?,
            idempotency_state: diom_idempotency::State::init(databases.clone())
                .context("initializing idempotency state")?,
            rate_limit_state: diom_rate_limit::State::init(databases.clone())
                .context("initializing rate limit state")?,
            auth_token_state: diom_auth_token::State::init(databases.clone())
                .context("initializing auth token state")?,
        };

        if let Some(timestamp) = logs.get_last_timestamp().await? {
            // if we've ever committed anything, make sure we don't rewind time on restarting
            time.update_from_other(timestamp);
        }

        anyhow::ensure!(
            !logs.is_poisoned().await?,
            "this node was previously removed from a cluster and must be erased before it can be re-added"
        );

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
            time,
            logs,
        };
        this.load_information().await?;
        this.start_metrics(DbMetrics::new(&this.state.meter, node_id));
        Ok(this)
    }

    pub fn cluster_id(&self) -> Option<&ClusterId> {
        self.cluster_id.as_ref()
    }

    pub fn db_handle(&self) -> impl std::ops::Deref<Target = Stores> + Send {
        self.stores.read_arc()
    }

    pub fn start_metrics(&self, metrics: DbMetrics) {
        let stores = Arc::clone(&self.stores);
        let shutdown = crate::shutting_down_token();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            loop {
                tokio::select! {
                    _ = ticker.tick() => {}
                    _ = shutdown.cancelled() => break,
                }

                match spawn_blocking_in_current_span({
                    let stores = Arc::clone(&stores);
                    move || {
                        let guard = stores.read();
                        let persistent_bytes = guard.databases.persistent.disk_space()?;
                        let ephemeral_bytes = guard.databases.ephemeral.disk_space()?;
                        Ok::<_, fjall::Error>((persistent_bytes, ephemeral_bytes))
                    }
                })
                .await
                .expect("Failed joining blocking task")
                {
                    Ok((persistent_bytes, ephemeral_bytes)) => {
                        metrics.bytes_used(persistent_bytes, DbType::Persistent);
                        metrics.bytes_used(ephemeral_bytes, DbType::Ephemeral);
                    }
                    Err(err) => tracing::info!(?err, "failed to read db disk space"),
                }
            }
        });
    }

    pub(super) async fn set_cluster_id(&mut self, id: ClusterId) -> anyhow::Result<()> {
        let keyspace = self.meta_keyspace.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
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
            spawn_blocking_in_current_span(move || -> anyhow::Result<_> {
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
        if let Some(cluster_id) = &cluster_id {
            tracing::info!(%cluster_id, "starting up with existing cluster membership");
        }
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
            path: snapshot_path.clone(),
        };
        tracing::trace!(last_snapshot=?data, "setting last_snapshot");
        let data = spawn_blocking_in_current_span(move || -> anyhow::Result<LastSnapshot> {
            let db = &handle.read().databases.persistent;
            LAST_SNAPSHOT.store(&keyspace, &data)?;
            db.persist(PersistMode::SyncAll)?;
            Ok(data)
        })
        .await??;
        *self.last_snapshot.write() = Some(data);
        self.delete_unused_snapshots(&snapshot_path)
            .await
            .context("cleaning up snapshots after setting new snapshot")?;
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
        spawn_blocking_in_current_span(move || {
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
        spawn_blocking_in_current_span(move || {
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
        // FIXME: I'm sure there's a way to do it without the allocation. E.g. by creating the
        // closures as well in one loop.
        let entries: Vec<_> = entries.into_iter().collect();
        let n = entries.len();
        let mut responses: Vec<Option<Response>> = vec![None; n];
        let mut changed_log_id = false;
        let mut changed_membership = false;

        // Step 1: Update all in-memory state upfront, though see readme below, I'm not sure it's
        // even needed necessarily.
        for entry in &entries {
            self.last_applied_log_id = Some(entry.log_id);
            changed_log_id = true;
            match &entry.payload {
                // FIXME: I updated the time in advance. Felt safe because we pass the request time
                // to each module anyway. Probably just need to do it at the end?
                EntryPayload::Normal(req) => self.time.update_from_other(req.timestamp),
                EntryPayload::Membership(m) => {
                    self.last_membership = StoredMembership::new(Some(entry.log_id), m.clone());
                    changed_membership = true;
                }
                EntryPayload::Blank => {}
            }
        }

        // Execute in parallel based on a conflict key (reported by operation).
        // I also used ClusterInternal as a barrier, though I don't think it should be? I think we
        // can just parallelize around that as well?
        let stores = Arc::clone(&self.stores);
        // Why is this separate here? I think can also just be an Arc?
        let namespace_state = self.state.namespace_state.clone();

        // Segment: (original_idx, wave, request, log_id)
        let mut current_segment: Vec<(usize, usize, RequestWithContext, LogId<NodeId>)> = vec![];
        let mut key_to_wave: HashMap<String, usize> = HashMap::new();

        let persistent_batch = Arc::new(RwLock::new(self.state.do_not_use_dbs.persistent.batch()));

        for (idx, entry) in entries.iter().enumerate() {
            match &entry.payload {
                EntryPayload::Blank => {
                    tracing::trace!("heartbeat");
                    responses[idx] = Some(Response::Blank);
                }
                EntryPayload::Membership(_) => {
                    tracing::trace!("changing cluster membership");
                    responses[idx] = Some(Response::Blank);
                }
                EntryPayload::Normal(req) => {
                    if matches!(req.inner, Request::ClusterInternal(_)) {
                        // ClusterInternal is a barrier: flush pending parallel work first.
                        // FIXME: though should it be a barrier? I don't know.
                        execute_parallel_segment(
                            &current_segment,
                            &stores,
                            persistent_batch.clone(),
                            &namespace_state,
                            &mut responses,
                        )
                        .await?;
                        current_segment.clear();
                        key_to_wave.clear();

                        tracing::trace!(log_id=?entry.log_id, "applying cluster-internal request");
                        let reply = match super::applier::apply_cluster_internal(
                            req,
                            self,
                            persistent_batch.clone(),
                            entry.log_id,
                        )
                        .await
                        {
                            Ok(r) => r,
                            Err(e) => {
                                tracing::error!("failed to apply raft log");
                                return Err(StorageError::IO {
                                    source: StorageIOError::apply(entry.log_id, e),
                                });
                            }
                        };
                        responses[idx] = Some(reply);
                    } else {
                        tracing::trace!(log_id=?entry.log_id, request=?req, "scheduling user request");
                        let key = apply_conflict_key(req);
                        let wave = key_to_wave.get(&key).map(|&w| w + 1).unwrap_or(0);
                        key_to_wave.insert(key, wave);
                        current_segment.push((idx, wave, req.clone(), entry.log_id));
                    }
                }
            }
        }

        // Flush any remaining parallel segment.
        execute_parallel_segment(
            &current_segment,
            &stores,
            persistent_batch.clone(),
            &namespace_state,
            &mut responses,
        )
        .await?;

        // FIXME to not unwrap... It's very ugly, but the idea is to confirm we are the only ones
        // here, which should be the case.
        Arc::into_inner(persistent_batch)
            .unwrap()
            .into_inner()
            .commit()
            .unwrap();

        if changed_log_id || changed_membership {
            self.record_ids_().await.map_err(write_err)?;
        }

        Ok(responses.into_iter().map(|r| r.unwrap()).collect())
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

        let targets = spawn_blocking_in_current_span(move || {
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
        let path = directory.join(file_name);
        let path_c = path.clone();
        let file = spawn_blocking_in_current_span(move || -> anyhow::Result<std::fs::File> {
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
        self.inner.write().await.build_snapshot_().await
    }
}

/// Compute the conflict key for a request, used to determine which entries
/// can execute in parallel (different keys) vs must be sequential (same key).
///
/// Two entries with the same conflict key are placed in successive waves and
/// run sequentially. Entries with different keys can run in the same wave.
fn apply_conflict_key(req: &RequestWithContext) -> String {
    let module = req.module();
    match req.hashed_key() {
        Some(hash) => format!("{module}:{hash}"),
        // Operations with no key (e.g. AuthToken) are serialized within their module.
        None => format!("{module}:__sequential__"),
    }
}

/// Execute one parallel segment of log entries using wave scheduling.
///
/// Entries in the same wave have no key conflicts and run concurrently via
/// `join_all`. Waves are executed in order so that entries sharing a conflict
/// key still apply in log order overall.
async fn execute_parallel_segment(
    segment: &[(usize, usize, RequestWithContext, LogId<NodeId>)],
    stores: &Arc<RwLock<Stores>>,
    batch: Arc<RwLock<OwnedWriteBatch>>,
    namespace_state: &diom_namespace::State,
    responses: &mut Vec<Option<Response>>,
) -> StorageResult<()> {
    if segment.is_empty() {
        return Ok(());
    }

    let max_wave = segment.iter().map(|(_, w, _, _)| *w).max().unwrap_or(0);

    for wave_idx in 0..=max_wave {
        let wave_futures: Vec<_> = segment
            .iter()
            .filter(|(_, w, _, _)| *w == wave_idx)
            .map(|(original_idx, _, req, log_id)| {
                let stores = Arc::clone(stores);
                let namespace_state = namespace_state.clone();
                let req = req.clone();
                let log_id = *log_id;
                let original_idx = *original_idx;
                let batch = batch.clone();
                async move {
                    let result = super::applier::apply_module_request(
                        req,
                        stores,
                        batch,
                        namespace_state,
                        log_id,
                    )
                    .await;
                    (original_idx, log_id, result)
                }
            })
            .collect();

        let results = futures_util::future::join_all(wave_futures).await;

        for (original_idx, log_id, result) in results {
            match result {
                Ok(response) => responses[original_idx] = Some(response),
                Err(e) => {
                    tracing::error!("failed to apply raft log");
                    return Err(StorageError::IO {
                        source: StorageIOError::apply(log_id, e),
                    });
                }
            }
        }
    }

    Ok(())
}

impl RaftStateMachine<TypeConfig> for StoreHandle {
    type SnapshotBuilder = Self;

    async fn applied_state(
        &mut self,
    ) -> StorageResult<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>)> {
        let this = self.inner.read().await;
        Ok((this.last_applied_log_id, this.last_membership.clone()))
    }

    async fn apply<I>(&mut self, entries: I) -> StorageResult<Vec<Response>>
    where
        I: IntoIterator<Item = <TypeConfig as RaftTypeConfig>::Entry> + openraft::OptionalSend,
        I::IntoIter: openraft::OptionalSend,
    {
        self.inner.write().await.apply_(entries).await
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> StorageResult<Box<<TypeConfig as RaftTypeConfig>::SnapshotData>> {
        self.inner.write().await.begin_receiving_snapshot_().await
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.inner.write().await.prep_snapshot_builder_();
        self.clone()
    }

    #[tracing::instrument(skip(self))]
    async fn get_current_snapshot(&mut self) -> StorageResult<Option<Snapshot<TypeConfig>>> {
        self.inner.write().await.get_current_snapshot_().await
    }

    #[tracing::instrument(skip(self, meta))]
    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<NodeId, Node>,
        snapshot: Box<StoredSnapshot>,
    ) -> StorageResult<()> {
        self.inner
            .write()
            .await
            .install_snapshot_(meta, snapshot)
            .await
            .map_err(read_snapshot_err)
    }
}

impl StoreHandle {
    pub async fn cluster_id(&self) -> Option<ClusterId> {
        self.inner.read().await.cluster_id().copied()
    }

    pub async fn log_id_before_time(
        &self,
        timestamp: jiff::Timestamp,
    ) -> anyhow::Result<Option<u64>> {
        self.inner
            .read()
            .await
            .logs
            .log_index_before(timestamp)
            .await
    }

    pub fn now(&self) -> jiff::Timestamp {
        self.time.update_now()
    }

    pub(crate) async fn kv_store(&self) -> diom_kv::State {
        self.inner.read().await.stores.read().kv_state.clone()
    }

    pub(super) async fn cache_store(&self) -> diom_cache::State {
        self.inner.read().await.stores.read().cache_state.clone()
    }

    pub(super) async fn idempotency_store(&self) -> diom_idempotency::State {
        self.inner
            .read()
            .await
            .stores
            .read()
            .idempotency_state
            .clone()
    }

    /// Mark this node as removed from a cluster and ineligible to continue
    pub(super) async fn poison(&self, cluster_id: ClusterId) -> anyhow::Result<()> {
        let handle = self.inner.write().await;
        handle.logs.poison(cluster_id).await?;
        Ok(())
    }
}
