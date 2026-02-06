use std::{collections::BTreeMap, sync::Arc, time::Instant};

use openraft::{
    BasicNode,
    error::{InitializeError, RaftError},
};
use serde::{Deserialize, Serialize};
use tap::TapFallible;
use uuid::Uuid;

use super::{
    handle::{RaftState, Request, Response},
    state_machine::StoredSnapshot,
};
use crate::{
    AppState,
    cfg::Configuration,
    core::cluster::{
        operations::InternalRequest,
        state_machine::{ClusterId, StoreHandle},
    },
};

// TODO: is BasicNode enough for us?
pub(super) type Node = BasicNode;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct NodeId {
    #[serde(with = "uuid::serde::simple")]
    inner: Uuid,
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl NodeId {
    pub(super) fn generate() -> Self {
        Self {
            inner: Uuid::new_v4(),
        }
    }
}

#[cfg(test)]
impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        let inner = Uuid::from_u64_pair(value, value);
        Self { inner }
    }
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
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
    raft.client_write(Request::ClusterInternal(InternalRequest::SetClusterId(
        new_id,
    )))
    .await
    .tap_err(|err| tracing::error!(?err, "failed to set initial cluster id"))?;
    tracing::debug!(elapsed=?start.elapsed(), "initialization finished");
    Ok(())
}

pub async fn initialize_raft(
    cfg: &Configuration,
    app_state: AppState,
) -> anyhow::Result<RaftState> {
    let mut logs = super::DiomLogs::new(&cfg.cluster.log_path).await?;
    let id = logs.get_node_id().await?;
    let config = openraft::Config {
        heartbeat_interval: cfg.cluster.heartbeat_interval_ms,
        election_timeout_min: cfg.cluster.election_timeout_min_ms,
        election_timeout_max: cfg.cluster.election_timeout_max_ms,
        cluster_name: cfg.cluster.name.clone(),
        ..Default::default()
    };
    let config = Arc::new(config.validate()?);
    let network = super::network::NetworkFactory::new(cfg);

    // TODO: handle ephemeral DB
    let db = app_state.configgroup_state.db().clone();

    let state_machine =
        super::state_machine::Store::new(db, cfg.cluster.snapshot_path.clone(), app_state).await?;
    let state_machine: StoreHandle = state_machine.into();
    let raft = Raft::new(id, config, network, logs, state_machine.clone()).await?;
    Ok(RaftState {
        raft,
        node_id: id,
        state_machine,
    })
}

#[cfg(test)]
mod tests {
    use fjall::Database;
    use openraft::{StorageIOError, testing::StoreBuilder};
    use tempfile::TempDir;

    use crate::{AppState, cfg::ConfigurationInner};

    use super::{
        super::{
            logs::DiomLogs,
            state_machine::{Store, StoreHandle},
        },
        NodeId, TypeConfig,
    };

    struct DiomStoreBuilder;

    impl DiomStoreBuilder {
        async fn setup() -> anyhow::Result<(TempDir, DiomLogs, StoreHandle)> {
            let workdir = tempfile::tempdir()?;
            let mut log_path = workdir.path().to_path_buf();
            log_path.push("logs");
            let logs = DiomLogs::new(log_path).await?;

            let mut data_path = workdir.path().to_path_buf();
            data_path.push("data");
            let mut e_data_path = workdir.path().to_path_buf();
            e_data_path.push("edata");

            let mut snapshot_path = workdir.path().to_path_buf();
            snapshot_path.push("snapshots");

            let mut cfg = ConfigurationInner::default();
            cfg.ephemeral_db.path = e_data_path.clone();
            cfg.persistent_db.path = data_path.clone();
            let cfg = cfg.into();

            let db = Database::builder(data_path).open()?;

            let app_state: AppState = AppState::new(cfg);

            let store = Store::new(db, snapshot_path, app_state).await?;

            Ok((workdir, logs, store.into()))
        }
    }

    impl StoreBuilder<TypeConfig, DiomLogs, StoreHandle, TempDir> for DiomStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(TempDir, DiomLogs, StoreHandle), openraft::StorageError<NodeId>> {
            Self::setup().await.map_err(|e| openraft::StorageError::IO {
                source: StorageIOError::write(e),
            })
        }
    }

    #[test]
    fn test_storage_openraft_slow() -> anyhow::Result<()> {
        openraft::testing::Suite::test_all(DiomStoreBuilder)?;
        Ok(())
    }
}
