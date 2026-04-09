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
        metrics::{ClusterMetrics, LogMetrics, OpenraftMetrics},
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
    let voters = cluster.keys().copied().collect::<Vec<_>>();
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
        .voter_ids(voters, "waiting for cluster to bootstrap")
        .await?;
    let new_id = ClusterId::generate();
    tracing::info!(cluster_id=%new_id, "cluster initialized, setting cluster_id");
    #[allow(clippy::disallowed_methods)]
    raft.client_write(Arc::new(RequestWithContext::new(
        Request::ClusterInternal(SetClusterUuidOperation(new_id).into()),
        jiff::Timestamp::now(),
        None,
    )))
    .await
    .tap_err(|err| tracing::error!(?err, "failed to set initial cluster id"))?;
    tracing::debug!(elapsed=?start.elapsed(), "initialization finished");
    Ok(new_id)
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
        cfg.cluster.log_ack_immediately,
    )
    .context("setting up log store")?;
    let id: NodeId = logs
        .get_node_id()
        .await
        .context("reading node ID from logs")?;
    logs.enable_metrics(LogMetrics::new(&app_state.meter, id));
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
    let network = super::network::NetworkFactory::new(cfg)?;

    let db = app_state.namespace_state.both_dbs.persistent.clone();
    let edb = app_state.namespace_state.both_dbs.ephemeral.clone();

    let metrics = ClusterMetrics::new(&app_state.meter, id);

    let state_machine = super::state_machine::Store::new(
        db,
        edb,
        cfg.cluster.snapshot_path(cfg)?,
        app_state.clone(),
        logs.clone(),
        id,
        time.clone(),
    )
    .await?;
    let state_machine: StoreHandle = state_machine.into();

    let raft = Raft::new(id, config, network.clone(), logs, state_machine.clone())
        .await
        .context("initializing openraft")?;

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

    use diom_proto::InternalClient;
    use fjall::Database;
    use openraft::testing::log::StoreBuilder;
    use tempfile::TempDir;

    use crate::{AppState, cfg::ConfigurationInner};

    use super::{
        super::{
            logs::DiomLogs,
            state_machine::{Store, StoreHandle},
        },
        TypeConfig,
    };
    use crate::cfg::Dir;

    struct DiomStoreBuilder;

    impl DiomStoreBuilder {
        async fn setup() -> anyhow::Result<(TempDir, DiomLogs, StoreHandle)> {
            let workdir = tempfile::tempdir()?;
            let log_path = workdir.path().to_path_buf().join("logs");
            let log_path = Dir::new(log_path)?;
            let logs = DiomLogs::new(log_path, 1, Duration::from_secs(10), false)?;

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
            )
            .await?;

            Ok((workdir, logs, store.into()))
        }
    }

    impl StoreBuilder<TypeConfig, DiomLogs, StoreHandle, TempDir> for DiomStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(TempDir, DiomLogs, StoreHandle), openraft::StorageError<TypeConfig>>
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
