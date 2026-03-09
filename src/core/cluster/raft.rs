use std::{collections::BTreeMap, sync::Arc, time::Instant};

use anyhow::Context;
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
        metrics::LogMetrics,
    },
};

openraft::declare_raft_types!(
    pub TypeConfig:
        D = RequestWithContext,
        R = Response,
        Node = Node,
        NodeId = NodeId,
        SnapshotData = StoredSnapshot
);

pub type Raft = openraft::Raft<TypeConfig>;

pub(super) async fn initialize_cluster(
    raft: &Raft,
    cluster: BTreeMap<NodeId, Node>,
) -> anyhow::Result<()> {
    let start = Instant::now();
    let voters = cluster.keys().copied().collect::<Vec<_>>();
    match raft.initialize(cluster).await {
        Ok(_) => {}
        Err(RaftError::APIError(InitializeError::NotAllowed(_))) => {
            tracing::debug!("cluster already initialized, ignoring");
            return Ok(());
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
    tracing::debug!(cluster_id=?new_id, "cluster initialized, setting cluster_id");
    raft.client_write(RequestWithContext::new(
        Request::ClusterInternal(SetClusterUuidOperation(new_id).into()),
        jiff::Timestamp::now(),
        None,
    ))
    .await
    .tap_err(|err| tracing::error!(?err, "failed to set initial cluster id"))?;
    tracing::debug!(elapsed=?start.elapsed(), "initialization finished");
    Ok(())
}

pub async fn initialize_raft(
    cfg: &Configuration,
    app_state: AppState,
) -> anyhow::Result<RaftState> {
    let mut logs = super::CoyoteLogs::new(
        cfg.cluster.log_path(cfg)?,
        cfg.cluster.log_sync_interval_commits,
        cfg.cluster.log_sync_interval_duration,
        cfg.cluster.log_ack_immediately,
    )
    .context("setting up log store")?;
    let id = logs
        .get_node_id()
        .await
        .context("reading node ID from logs")?;
    let config = openraft::Config {
        heartbeat_interval: cfg.cluster.heartbeat_interval.as_millis() as u64,
        election_timeout_min: cfg.cluster.election_timeout_min.as_millis() as u64,
        election_timeout_max: cfg.cluster.election_timeout_max.as_millis() as u64,
        cluster_name: cfg.cluster.name.clone(),
        snapshot_policy: openraft::SnapshotPolicy::Never,
        ..Default::default()
    };
    let config = Arc::new(config.validate().context("configuring openraft")?);
    let network = super::network::NetworkFactory::new(cfg)?;

    let db = app_state.namespace_state.both_dbs.persistent.clone();
    let edb = app_state.namespace_state.both_dbs.ephemeral.clone();

    let state_machine = super::state_machine::Store::new(
        db,
        edb,
        cfg.cluster.snapshot_path(cfg)?,
        app_state.clone(),
        logs.clone(),
        id,
    )
    .await?;
    let state_machine: StoreHandle = state_machine.into();

    logs.start_metrics(LogMetrics::new(&app_state.meter, id));

    let raft = Raft::new(id, config, network.clone(), logs, state_machine.clone())
        .await
        .context("initializing openraft")?;
    let (bgtx, bgrx) = tokio::sync::mpsc::channel(10);
    let handle = RaftState {
        raft,
        node_id: id,
        state_machine,
        network,
        background_channel: bgtx,
    };
    tokio::spawn({
        let handle = handle.clone();
        let cfg = cfg.clone();
        async move {
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
        async move {
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

    use fjall::Database;
    use openraft::{StorageIOError, testing::StoreBuilder};
    use tempfile::TempDir;

    use crate::{AppState, cfg::ConfigurationInner};

    use super::{
        super::{
            logs::CoyoteLogs,
            state_machine::{Store, StoreHandle},
        },
        NodeId, TypeConfig,
    };
    use crate::cfg::Dir;

    struct CoyoteStoreBuilder;

    impl CoyoteStoreBuilder {
        async fn setup() -> anyhow::Result<(TempDir, CoyoteLogs, StoreHandle)> {
            let workdir = tempfile::tempdir()?;
            let log_path = workdir.path().to_path_buf().join("logs");
            let log_path = Dir::new(log_path)?;
            let logs = CoyoteLogs::new(log_path, 1, Duration::from_secs(10), false)?;

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

            let app_state: AppState = AppState::new(cfg);

            let store =
                Store::new(db, edb, snapshot_path, app_state, logs.clone(), 1.into()).await?;

            Ok((workdir, logs, store.into()))
        }
    }

    impl StoreBuilder<TypeConfig, CoyoteLogs, StoreHandle, TempDir> for CoyoteStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(TempDir, CoyoteLogs, StoreHandle), openraft::StorageError<NodeId>> {
            Self::setup().await.map_err(|e| openraft::StorageError::IO {
                source: StorageIOError::write(e),
            })
        }
    }

    #[test]
    fn test_storage_openraft_slow() -> anyhow::Result<()> {
        openraft::testing::Suite::test_all(CoyoteStoreBuilder)?;
        Ok(())
    }
}
