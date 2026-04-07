use std::{
    io::{Seek, SeekFrom},
    os::unix::fs::PermissionsExt,
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
use coyote_core::{Monotime, task::spawn_blocking_in_current_span};
use coyote_error::CanFailExt;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode};
use fjall_utils::{Databases, FjallFixedKey, ReadonlyKeyspace, StorageType};
use futures_util::{Stream, StreamExt};
use openraft::{EntryPayload, RaftSnapshotBuilder, storage::RaftStateMachine};
use parking_lot::RwLock;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tap::TapOptional;
use tokio::sync::RwLock as TokioRwLock;
use uuid::Uuid;

use super::{
    LogId, NodeId, handle::Response, logs::CoyoteLogs, raft::TypeConfig, serialized_state_machine,
};
use crate::AppState;

type StorageResult<T> = std::io::Result<T>;
type StoredMembership = openraft::type_config::alias::StoredMembershipOf<TypeConfig>;
type SnapshotMeta = openraft::type_config::alias::SnapshotMetaOf<TypeConfig>;
type Snapshot = openraft::type_config::alias::SnapshotOf<TypeConfig>;
type SnapshotData = openraft::type_config::alias::SnapshotDataOf<TypeConfig>;
type EntryResponder = openraft::storage::EntryResponder<TypeConfig>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LastSnapshot {
    meta: SnapshotMeta,
    path: PathBuf,
}

fn io_err(e: anyhow::Error) -> std::io::Error {
    std::io::Error::other(e)
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
    pub msgs_state: coyote_msgs::State,
    pub kv_state: coyote_kv::State,
    pub cache_state: coyote_cache::State,
    pub idempotency_state: coyote_idempotency::State,
    pub rate_limit_state: coyote_rate_limit::State,
    pub auth_token_state: coyote_auth_token::State,
    pub admin_auth_state: coyote_admin_auth::State,
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
    last_applied_log_id: Option<LogId>,
    last_membership: StoredMembership,
    last_snapshot: Arc<RwLock<Option<LastSnapshot>>>,
    cluster_id: Option<ClusterId>,
    pub(super) time: Monotime,
    pub(super) logs: CoyoteLogs,
    metrics: DbMetrics,
}

trait SnapshotIdx {
    fn snapshot_idx(&self) -> anyhow::Result<u64>;
}

impl SnapshotIdx for SnapshotMeta {
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

static LAST_APPLIED_LOG_ID: FjallFixedKey<LogId> = FjallFixedKey::new("last_applied_log_id");
static LAST_SNAPSHOT: FjallFixedKey<LastSnapshot> = FjallFixedKey::new("last_snapshot");
static LAST_MEMBERSHIP: FjallFixedKey<StoredMembership> = FjallFixedKey::new("last_membership");
static CLUSTER_UUID: FjallFixedKey<ClusterId> = FjallFixedKey::new("cluster_uuid");

impl Store {
    pub async fn new(
        persistent_db: Database,
        ephemeral_db: Database,
        snapshot_directory: Dir,
        app_state: AppState,
        logs: CoyoteLogs,
        node_id: NodeId,
        time: Monotime,
    ) -> anyhow::Result<Self> {
        let meta_keyspace =
            persistent_db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let databases = Databases::new(persistent_db.clone(), ephemeral_db);

        let stores = Stores {
            databases: databases.clone(),
            msgs_state: coyote_msgs::State::init(
                persistent_db.clone(),
                app_state.topic_publish_notifier.clone(),
            )
            .context("initializing msgs state")?,
            kv_state: coyote_kv::State::init(databases.clone()).context("initializing kv state")?,
            cache_state: coyote_cache::State::init(databases.clone())
                .context("initializing cache state")?,
            idempotency_state: coyote_idempotency::State::init(databases.clone())
                .context("initializing idempotency state")?,
            rate_limit_state: coyote_rate_limit::State::init(databases.clone())
                .context("initializing rate limit state")?,
            auth_token_state: coyote_auth_token::State::init(databases.clone())
                .context("initializing auth token state")?,
            admin_auth_state: coyote_admin_auth::State::init(databases.clone())
                .context("initializing admin auth state")?,
        };

        if let Some(timestamp) = logs.get_last_timestamp().await? {
            // if we've ever committed anything, make sure we don't rewind time on restarting
            time.update_from_other(timestamp);
        }

        anyhow::ensure!(
            !logs.is_poisoned().await?,
            "this node was previously removed from a cluster and must be erased before it can be re-added"
        );

        let metrics = DbMetrics::new(&app_state.meter, node_id);
        let mut this = Self {
            stores: Arc::new(RwLock::new(stores)),
            state: app_state,
            snapshot_directory: snapshot_directory.as_path().canonicalize()?,
            readonly_meta_keyspace: ReadonlyKeyspace::from(meta_keyspace.clone()),
            meta_keyspace,
            last_snapshot: Arc::new(RwLock::new(None)),
            snapshot_idx: 0,
            last_applied_log_id: None,
            last_membership: Default::default(),
            cluster_id: None,
            time,
            logs,
            metrics,
        };
        this.load_information().await?;
        this.start_metrics();
        Ok(this)
    }

    pub fn cluster_id(&self) -> Option<&ClusterId> {
        self.cluster_id.as_ref()
    }

    pub fn db_handle(&self) -> parking_lot::ArcRwLockReadGuard<parking_lot::RawRwLock, Stores> {
        self.stores.read_arc()
    }

    /// Run a background task to collect metrics from the database
    fn start_metrics(&self) {
        let stores = Arc::clone(&self.stores);
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let shutdown = crate::shutting_down_token();
            let mut ticker = tokio::time::interval(Duration::from_secs(10));
            let mut last_fetched_size = std::time::Instant::now();

            while shutdown.run_until_cancelled(ticker.tick()).await.is_some() {
                let should_fetch_size = last_fetched_size.elapsed() > Duration::from_secs(120);

                spawn_blocking_in_current_span({
                    let stores = Arc::clone(&stores);
                    let metrics = metrics.clone();
                    move || {
                        let guard = stores.read();
                        metrics
                            .record_db(
                                DbType::Persistent,
                                &guard.databases.persistent,
                                should_fetch_size,
                            )
                            .warn_on_fail("error fetching persistent DB metrics");
                        metrics
                            .record_db(
                                DbType::Ephemeral,
                                &guard.databases.ephemeral,
                                should_fetch_size,
                            )
                            .warn_on_fail("error fetching ephemeral DB metrics");
                    }
                })
                .await
                .expect("Failed joining blocking task");

                if should_fetch_size {
                    last_fetched_size = std::time::Instant::now();
                }
            }
        });
    }

    pub(super) async fn set_cluster_id(&mut self, id: ClusterId) -> anyhow::Result<()> {
        let keyspace = self.meta_keyspace.clone();
        let handle = self.stores.clone();
        spawn_blocking_in_current_span(move || -> anyhow::Result<()> {
            let mut tx = handle
                .read()
                .databases
                .persistent
                .batch()
                .durability(Some(PersistMode::SyncAll));
            CLUSTER_UUID.store_tx(&mut tx, &keyspace, &id)?;
            tx.commit()?;
            Ok(())
        })
        .await??;
        self.cluster_id = Some(id);
        Ok(())
    }

    #[tracing::instrument(skip_all)]
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
        meta: SnapshotMeta,
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
            let mut tx = handle
                .read()
                .databases
                .persistent
                .batch()
                .durability(Some(PersistMode::SyncAll));
            LAST_SNAPSHOT.store_tx(&mut tx, &keyspace, &data)?;
            tx.commit()?;
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
        let last_file_name = {
            let handle = self.last_snapshot.read();
            handle
                .as_ref()
                .and_then(|s| s.path.file_name())
                .map(|s| s.to_owned())
        };
        while let Some(dent) = dents.next_entry().await? {
            if let Some(preserve_path) = keep_path.file_name() {
                if dent.file_name() == preserve_path {
                    tracing::trace!(filename=?dent.file_name(), "preserving used snapshot");
                    continue;
                }
            } else {
                tracing::warn!(path=?keep_path, "very weird snapshot path");
            }
            if let Some(preserve_name) = &last_file_name
                && dent.file_name() == *preserve_name
            {
                tracing::trace!(filename=?dent.file_name(), "preserving last_snapshot");
                continue;
            }
            tracing::debug!(filename=?dent.file_name(), "deleting unused snapshot");
            tokio::fs::remove_file(dent.path()).await?;
        }
        Ok(())
    }

    async fn install_snapshot_(
        &mut self,
        meta: &SnapshotMeta,
        mut snapshot: StoredSnapshot,
    ) -> anyhow::Result<()> {
        tracing::debug!("starting snapshot installation");
        if !snapshot.is_final {
            snapshot.persist(meta, &self.snapshot_directory)?;
        }
        let mut f = snapshot.file.try_clone().await?.into_std().await;
        f.seek(SeekFrom::Start(0))?;
        let handle = self.stores.clone();
        spawn_blocking_in_current_span(move || {
            let stores = handle.write();
            serialized_state_machine::load_from_file(&stores.databases, &mut f)
        })
        .await??;
        // load the embedded information from the metadata table in the snapshot
        self.load_information().await?;
        // overwrite any last_log_id and membership in the snapshot with the ones the leader told us
        self.last_applied_log_id = meta.last_log_id;
        self.last_membership = meta.last_membership.clone();
        self.record_ids_().await?;
        // clean up the snapshot directory
        self.set_last_snapshot_(meta.clone(), snapshot.path.clone())
            .await?;
        self.delete_unused_snapshots(snapshot.path.as_path())
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(num_entries))]
    async fn apply_<S>(&mut self, mut entries: S) -> anyhow::Result<()>
    where
        S: Stream<Item = std::io::Result<EntryResponder>> + Unpin + Send,
    {
        let mut changed_log_id = false;
        let mut changed_membership = false;
        let mut num_entries = 0;
        let start = std::time::Instant::now();
        let context = super::applier::ApplyContext::new(self);

        let mut touched_ephemeral = false;
        let mut touched_persistent = false;

        while let Some(entry) = entries.next().await {
            let (item, responder) = entry?;

            self.last_applied_log_id = Some(item.log_id);
            changed_log_id = true;

            let reply = match item.payload {
                EntryPayload::Blank => {
                    tracing::trace!("heartbeat");
                    Response::Blank
                }
                EntryPayload::Normal(req) => {
                    tracing::trace!(log_id=?item.log_id, request=?req, "applying user request");

                    if req.inner.affects_persistent() {
                        touched_persistent = true;
                    }
                    if req.inner.affects_ephemeral() {
                        touched_ephemeral = true;
                    }

                    super::applier::apply_request(&context, req, self, item.log_id)
                        .await
                        .map_err(|err| {
                            tracing::error!(?err, "failed to apply raft log");
                            std::io::Error::other(err)
                        })?
                }
                EntryPayload::Membership(last_membership) => {
                    tracing::trace!("changing cluster membership");
                    self.last_membership =
                        StoredMembership::new(Some(item.log_id), last_membership);
                    changed_membership = true;
                    Response::Blank
                }
            };
            if let Some(responder) = responder {
                responder.send(reply);
            }
            num_entries += 1;
        }
        tracing::Span::current().record("num_entries", num_entries);
        tracing::trace!(num_entries, "applied some entries");
        if changed_log_id || changed_membership {
            self.record_ids_().await.context("recording updated IDs")?;
        }
        if touched_persistent {
            context
                .stores
                .databases
                .persistent
                .persist(self.state.cfg.sync_mode.into())?;
        }
        if touched_ephemeral {
            context
                .stores
                .databases
                .ephemeral
                .persist(self.state.cfg.sync_mode.into())?;
        }
        self.metrics.record_apply(num_entries, start.elapsed());
        Ok(())
    }

    async fn begin_receiving_snapshot_(&mut self) -> anyhow::Result<SnapshotData> {
        let tempfile = tempfile::Builder::new()
            .permissions(std::fs::Permissions::from_mode(0o600))
            .tempfile_in(&self.snapshot_directory)?;
        let (f, path) = tempfile.keep()?;
        self.snapshot_idx += 1;
        let f = tokio::fs::File::from_std(f);
        Ok(StoredSnapshot {
            path,
            file: f,
            is_final: false,
        })
    }

    fn prep_snapshot_builder_(&mut self) {
        self.snapshot_idx += 1;
    }

    #[tracing::instrument(skip_all)]
    async fn get_current_snapshot_(&mut self) -> anyhow::Result<Option<Snapshot>> {
        // clone to avoid holding a lock over an await point
        let last_snapshot = self.last_snapshot.read().clone();
        if let Some(last_snapshot) = last_snapshot {
            tracing::trace!(?last_snapshot, "found last_snapshot");
            let f = tokio::fs::File::open(&last_snapshot.path)
                .await
                .context("failed to open snapshot")?;
            Ok(Some(Snapshot {
                meta: last_snapshot.meta.clone(),
                snapshot: StoredSnapshot {
                    file: f,
                    path: last_snapshot.path.clone(),
                    is_final: true,
                },
            }))
        } else {
            tracing::trace!("found no last_snapshot");
            Ok(None)
        }
    }

    async fn build_snapshot_(&self) -> anyhow::Result<Snapshot> {
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

        let start = std::time::Instant::now();

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
        .context("failed to generate snapshot targets")?;

        let snapshot = StoredSnapshot::new(&meta, &self.snapshot_directory, targets)
            .await
            .context("failed to build snapshot")?;

        let path = snapshot.path.clone();

        let size =
            spawn_blocking_in_current_span(move || std::fs::metadata(&path).map(|m| m.len()))
                .await?
                .context("getting size of snapshot")?;

        self.metrics.record_snapshot(size, start.elapsed());

        Ok(Snapshot { meta, snapshot })
    }
}

// Wrapper around a snapshot that has been written to disk and has both a filename
// and a concrete File
pub struct StoredSnapshot {
    file: tokio::fs::File,
    path: PathBuf,
    is_final: bool,
}

impl Drop for StoredSnapshot {
    fn drop(&mut self) {
        if !self.is_final
            && let Err(err) = std::fs::remove_file(&self.path)
        {
            tracing::warn!(?err, "error unlinking in-progress snapshot");
        }
    }
}

impl std::fmt::Debug for StoredSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StoredSnapshot {{ path: {:?}, final: {:?} }}",
            self.path, self.is_final
        )
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
        metadata: &SnapshotMeta,
        directory: &Path,
        targets: Vec<(StorageType, Database, fjall::Snapshot, Vec<String>)>,
    ) -> anyhow::Result<Self> {
        let file_name = format!("coyote-{}", metadata.snapshot_id);
        let path = directory.join(file_name);
        let directory = directory.to_owned();
        let path_c = path.clone();
        let file = spawn_blocking_in_current_span(move || -> anyhow::Result<std::fs::File> {
            let mut tf = tempfile::Builder::new()
                .permissions(std::fs::Permissions::from_mode(0o600))
                .tempfile_in(directory)?;
            tracing::debug!(final_path=%path_c.display(), temp_path = %tf.path().display(), "writing snapshot");
            serialized_state_machine::serialize_to_file(targets, tf.as_file_mut())?;
            tf.as_file_mut().sync_all()?;
            let mut f = tf.persist_noclobber(path_c)?;
            f.seek(SeekFrom::Start(0))?;
            Ok(f)
        })
        .await??;
        let file = tokio::fs::File::from_std(file);
        Ok(Self {
            file,
            path,
            is_final: true,
        })
    }

    fn persist(&mut self, metadata: &SnapshotMeta, directory: &Path) -> anyhow::Result<()> {
        if self.is_final {
            anyhow::bail!("cannot persist a final snapshot");
        }
        let file_name = format!("coyote-{}", metadata.snapshot_id);
        let path = directory.join(file_name);
        tracing::debug!(
            current_path = %self.path.display(),
            target_path = %path.display(),
            "persisting incoming snapshot"
        );
        std::fs::rename(&self.path, &path).context("renaming snapshot into place")?;
        self.path = path;
        self.is_final = true;
        Ok(())
    }
}

pub struct StoreSnapshotHandle {
    inner: Arc<TokioRwLock<Store>>,
}

impl RaftSnapshotBuilder<TypeConfig> for StoreSnapshotHandle {
    async fn build_snapshot(&mut self) -> StorageResult<Snapshot> {
        // build the snapshot with a read lock so we don't block writes
        let snapshot = self
            .inner
            .read()
            .await
            .build_snapshot_()
            .await
            .map_err(io_err)?;
        // but set the last_snapshot_ field with a write lock to serialize
        self.inner
            .write()
            .await
            .set_last_snapshot_(snapshot.meta.clone(), snapshot.snapshot.path.clone())
            .await
            .context("failed to set last snapshot")
            .map_err(io_err)?;
        Ok(snapshot)
    }
}

impl RaftStateMachine<TypeConfig> for StoreHandle {
    type SnapshotBuilder = StoreSnapshotHandle;

    async fn applied_state(&mut self) -> StorageResult<(Option<LogId>, StoredMembership)> {
        let this = self.inner.read().await;
        Ok((this.last_applied_log_id, this.last_membership.clone()))
    }

    async fn apply<S>(&mut self, entries: S) -> std::io::Result<()>
    where
        S: Stream<Item = std::io::Result<EntryResponder>> + Unpin + Send,
    {
        self.inner
            .write()
            .await
            .apply_(entries)
            .await
            .map_err(io_err)
    }

    async fn begin_receiving_snapshot(&mut self) -> StorageResult<SnapshotData> {
        self.inner
            .write()
            .await
            .begin_receiving_snapshot_()
            .await
            .map_err(io_err)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.inner.write().await.prep_snapshot_builder_();
        StoreSnapshotHandle {
            inner: self.inner.clone(),
        }
    }

    #[tracing::instrument(skip_all)]
    async fn get_current_snapshot(&mut self) -> StorageResult<Option<Snapshot>> {
        self.inner
            .write()
            .await
            .get_current_snapshot_()
            .await
            .map_err(io_err)
    }

    #[tracing::instrument(skip_all, fields(snapshot_id = meta.snapshot_id))]
    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta,
        snapshot: StoredSnapshot,
    ) -> StorageResult<()> {
        let mut this = self.inner.write().await;
        this.install_snapshot_(meta, snapshot).await.map_err(io_err)
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

    pub(crate) async fn msgs_store(&self) -> coyote_msgs::State {
        self.inner.read().await.stores.read().msgs_state.clone()
    }

    pub(crate) async fn kv_store(&self) -> coyote_kv::State {
        self.inner.read().await.stores.read().kv_state.clone()
    }

    pub(crate) async fn cache_store(&self) -> coyote_cache::State {
        self.inner.read().await.stores.read().cache_state.clone()
    }

    pub(crate) async fn idempotency_store(&self) -> coyote_idempotency::State {
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

    pub(crate) async fn last_snapshot_id(&self) -> Option<String> {
        let handle = self.inner.read().await;
        let snap = handle.last_snapshot.read();
        snap.as_ref().map(|s| s.meta.snapshot_id.clone())
    }
}
