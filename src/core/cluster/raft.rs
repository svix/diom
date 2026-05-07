use std::{collections::BTreeMap, sync::Arc, time::Instant};

use anyhow::Context;
use diom_core::Monotime;
use openraft::error::{InitializeError, RaftError};
use tap::TapFallible;

use super::{
    handle::{RaftState, Request, RequestWithContext, Response},
    node::{Node, NodeId},
    state_machine::StoredSnapshot,
};
use crate::{
    AppState,
    cfg::Configuration,
    core::{
        cluster::{
            operations::SetClusterUuidOperation,
            state_machine::{ClusterId, StoreHandle},
        },
        metrics::{ClusterMetrics, ClusterNetworkMetrics, LogMetrics, OpenraftMetrics},
    },
};

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Arc<RequestWithContext>,
        R = Response,
        Node = Node,
        NodeId = NodeId,
        SnapshotData = StoredSnapshot
);

pub type Raft = openraft::Raft<TypeConfig, StoreHandle>;

pub(crate) async fn initialize_cluster(
    raft: &Raft,
    cluster: BTreeMap<NodeId, Node>,
) -> anyhow::Result<ClusterId> {
    let start = Instant::now();
    match raft.initialize(cluster).await {
        Ok(_) => {}
        Err(RaftError::APIError(InitializeError::NotAllowed(_))) => {
            anyhow::bail!("cluster already initialized");
        }
        Err(err) => {
            tracing::error!(?err, "error initializing cluster");
            return Err(err.into());
        }
    };
    raft.wait(None)
        .log_index_at_least(Some(1), "waiting for someone to become the leader")
        .await?;
    let new_id = ClusterId::generate();
    tracing::info!(cluster_id = %new_id, "cluster initialized, setting cluster_id");
    #[allow(clippy::disallowed_methods)]
    raft.client_write(Arc::new(RequestWithContext::new(
        Request::ClusterInternal(SetClusterUuidOperation(new_id).into()),
        jiff::Timestamp::now().into(),
        None,
    )))
    .await
    .tap_err(|err| tracing::error!(?err, "failed to set initial cluster id"))?;
    tracing::debug!(elapsed = ?start.elapsed(), "initialization finished");
    Ok(new_id)
}

struct RaftStateWatcherInner {
    ready: bool,
    has_applied_log: bool,
    is_single_node: bool,
    leader: Option<(NodeId, u64)>,
    tx: tokio::sync::watch::Sender<bool>,
    handle: Option<openraft::raft::WatchChangeHandle<TypeConfig>>,
}

impl RaftStateWatcherInner {
    fn recompute_ready(&mut self) {
        let new_ready = self.leader.is_some() && (self.is_single_node || self.has_applied_log);
        if new_ready != self.ready {
            self.ready = new_ready;
            self.tx.send_replace(new_ready);
        }
    }
}

#[derive(Clone)]
pub struct RaftStateWatcher {
    inner: Arc<parking_lot::RwLock<RaftStateWatcherInner>>,
    rx: tokio::sync::watch::Receiver<bool>,
}

impl RaftStateWatcher {
    fn new() -> Self {
        let (tx, rx) = tokio::sync::watch::channel(false);
        let inner = Arc::new(parking_lot::RwLock::new(RaftStateWatcherInner {
            ready: false,
            has_applied_log: false,
            is_single_node: false,
            leader: None,
            handle: None,
            tx,
        }));
        Self { inner, rx }
    }

    pub(crate) fn record_first_applied_log(&self) {
        let mut inner = self.inner.write();
        inner.has_applied_log = true;
        inner.recompute_ready();
    }

    async fn connect_raft(&mut self, raft: &Raft) {
        let inner_h = Arc::clone(&self.inner);
        let handle = raft.on_cluster_leader_change(move |_, (new_leader_id, _)| {
            let inner_h = Arc::clone(&inner_h);
            async move {
                let mut guard = inner_h.write();
                guard.leader = Some((new_leader_id.node_id, new_leader_id.term));
                guard.recompute_ready();
            }
        });
        let is_single_node = raft
            .with_raft_state(|s| s.membership_state.effective().nodes().count() == 1)
            .await
            .inspect_err(|err| {
                tracing::warn!(?err, "error determining if raft is in single-node mode")
            })
            .unwrap_or(false);
        let mut guard = self.inner.write();
        guard.handle = Some(handle);
        guard.is_single_node = is_single_node;
        guard.recompute_ready();
    }

    pub(crate) async fn wait_for_up(&self) -> bool {
        {
            let guard = self.inner.read();
            if guard.ready {
                tracing::debug!("node is already up");
                return true;
            }
        }
        tracing::debug!("waiting for node to come up");
        let mut rx = self.rx.clone();
        if rx.wait_for(|v| *v).await.is_err() {
            tracing::warn!("state watcher was shut down while waiting for up");
            return false;
        }
        tracing::debug!("node has come up");
        true
    }
}

pub async fn initialize_raft(
    cfg: &Configuration,
    app_state: AppState,
    time: Monotime,
    initialized: crate::Initialized,
) -> anyhow::Result<RaftState> {
    let mut logs = super::DiomLogs::new(
        cfg.cluster.log_path(cfg)?,
        cfg.cluster.log_sync_interval_commits,
        cfg.cluster.log_sync_interval_duration.into(),
        cfg.cluster.log_sync_interval_auto,
        cfg.cluster.log_sync_mode,
        cfg.fsync_mode,
        crate::shutting_down_token(),
    )
    .context("setting up log store")?;
    let id: NodeId = logs
        .get_node_id()
        .await
        .context("reading node ID from logs")?;
    logs.enable_metrics(LogMetrics::new(&app_state.meter, id))
        .await?;
    let config = openraft::Config {
        heartbeat_interval: cfg.cluster.heartbeat_interval.as_millis(),
        election_timeout_min: cfg.cluster.election_timeout_min.as_millis(),
        election_timeout_max: cfg.cluster.election_timeout_max.as_millis(),
        cluster_name: cfg.cluster.name.clone(),

        replication_lag_threshold: cfg.cluster.replication_lag_threshold,

        snapshot_policy: openraft::SnapshotPolicy::Never,

        // we're using the v1 version of snapshot sending for now
        #[allow(deprecated)]
        send_snapshot_timeout: cfg.cluster.send_snapshot_timeout.as_millis(),
        install_snapshot_timeout: cfg.cluster.send_snapshot_timeout.as_millis(),
        ..Default::default()
    };
    let config = Arc::new(config.validate().context("configuring openraft")?);
    let network_metrics = ClusterNetworkMetrics::new(&app_state.meter, id);
    let network = super::network::NetworkFactory::new(cfg, network_metrics)?;

    let db = app_state.namespace_state.both_dbs.persistent.clone();
    let edb = app_state.namespace_state.both_dbs.ephemeral.clone();

    let metrics = ClusterMetrics::new(&app_state.meter, id);

    let mut state_watcher = RaftStateWatcher::new();

    let state_machine = super::state_machine::Store::new(
        db,
        edb,
        cfg.cluster.snapshot_path(cfg)?,
        app_state.clone(),
        logs.clone(),
        id,
        time.clone(),
        crate::shutting_down_token(),
        state_watcher.clone(),
    )
    .await?;
    let state_machine: StoreHandle = state_machine.into();

    let raft = Raft::new(id, config, network.clone(), logs, state_machine.clone())
        .await
        .context("initializing openraft")?;

    state_watcher.connect_raft(&raft).await;

    let openraft_metrics = OpenraftMetrics::new(&app_state.meter, id);
    raft.set_metrics_recorder(Some(Arc::new(openraft_metrics)))
        .await?;

    let (bgtx, bgrx) = tokio::sync::mpsc::channel(10);
    let handle = RaftState {
        raft,
        node_id: id,
        state_machine,
        network,
        background_channel: bgtx,
        time,
        cfg: cfg.clone(),
        metrics: metrics.clone(),
        state_watcher,
    };

    #[cfg(feature = "raft-runtime-stats")]
    tokio::spawn({
        let handle = handle.clone();
        async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
            let shutdown = diom_core::shutdown::shutting_down_token();
            while shutdown
                .run_until_cancelled(interval.tick())
                .await
                .is_some()
            {
                let stats = match handle.raft.runtime_stats().await {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::warn!(?err, "unable to get runtime stats");
                        continue;
                    }
                };
                println!("{}", stats.display().human_readable());
            }
        }
    });
    tokio::spawn({
        let handle = handle.clone();
        let cfg = cfg.clone();
        let initialized = initialized.clone();
        async move {
            if initialized.wait().await.is_err() {
                return;
            }
            if let Err(err) =
                super::background::run_background_jobs_on_leader(cfg.clone(), handle.clone()).await
            {
                tracing::error!(
                    ?err,
                    "raft administrative process died; shutting everything down"
                );
                crate::start_shut_down()
            }
        }
    });
    tokio::spawn({
        let handle = handle.clone();
        let cfg = cfg.clone();
        let initialized = initialized.clone();
        async move {
            if initialized.wait().await.is_err() {
                return;
            }
            if let Err(err) = super::background::run_background_jobs_on_all_nodes(
                cfg.clone(),
                handle.clone(),
                bgrx,
            )
            .await
            {
                tracing::error!(
                    ?err,
                    "raft administrative process died; shutting everything down"
                );
                crate::start_shut_down()
            }
        }
    });
    Ok(handle)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use diom_error::CanFailExt;
    use diom_proto::InternalClient;
    use fjall::Database;
    use openraft::testing::log::StoreBuilder;
    use tempfile::TempDir;
    use tokio_util::sync::CancellationToken;

    use crate::{AppState, cfg::ConfigurationInner};

    use super::{
        super::{
            logs::DiomLogs,
            state_machine::{Store, StoreHandle},
        },
        TypeConfig,
    };
    use crate::cfg::{Dir, FsyncMode};

    struct DiomStoreGuard {
        tempdir: Option<TempDir>,
        cancel: CancellationToken,
    }

    impl Drop for DiomStoreGuard {
        fn drop(&mut self) {
            self.cancel.cancel();
            // wait a beat for it to catch up since cancellation is async and we're sync
            std::thread::sleep(Duration::from_millis(10));
            if let Some(tempdir) = self.tempdir.take() {
                tempdir.close().can_fail("error cleaning up tempdir");
            }
        }
    }

    struct DiomStoreBuilder;

    impl DiomStoreBuilder {
        async fn setup() -> anyhow::Result<(DiomStoreGuard, DiomLogs, StoreHandle)> {
            let workdir = tempfile::tempdir()?;
            let log_path = workdir.path().to_path_buf().join("logs");
            let log_path = Dir::new(log_path)?;
            let token = CancellationToken::new();

            let logs = DiomLogs::new(
                log_path,
                1,
                Duration::from_secs(10),
                false,
                crate::cfg::SyncMode::Buffer,
                FsyncMode::default(),
                token.clone(),
            )?;

            let data_path = workdir.path().join("data");
            let e_data_path = workdir.path().join("edata");

            let snapshot_path = workdir.path().join("snapshots");
            let snapshot_path = Dir::new(snapshot_path)?;

            let mut cfg = ConfigurationInner::default();
            cfg.ephemeral_db.path = e_data_path.clone();
            cfg.persistent_db.path = data_path.clone();
            let cfg = cfg.into();

            let db = Database::builder(data_path).open()?;
            let edb = Database::builder(e_data_path).open()?;

            let time = diom_core::Monotime::initial();
            let _ = time.update_now();

            // FIXME: Should we be spawning an internal API server task here?
            let internal_client = InternalClient::useless_instance_for_tests();
            let app_state = AppState::new(cfg, time.clone(), internal_client);

            let store = Store::new(
                db,
                edb,
                snapshot_path,
                app_state,
                logs.clone(),
                1.into(),
                time,
                token.clone(),
                super::RaftStateWatcher::new(),
            )
            .await?;

            let guard = DiomStoreGuard {
                tempdir: Some(workdir),
                cancel: token,
            };

            Ok((guard, logs, store.into()))
        }
    }

    impl StoreBuilder<TypeConfig, DiomLogs, StoreHandle, DiomStoreGuard> for DiomStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(DiomStoreGuard, DiomLogs, StoreHandle), openraft::StorageError<TypeConfig>>
        {
            Ok(Self::setup().await.unwrap())
        }
    }

    #[tokio::test]
    async fn test_storage_openraft_slow() -> anyhow::Result<()> {
        openraft::testing::log::Suite::test_all(DiomStoreBuilder).await?;
        Ok(())
    }
}
