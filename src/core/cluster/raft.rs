use std::sync::Arc;

use fjall::Database;
use openraft::BasicNode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::discovery::Discovery;
use super::state_machine::StoredSnapshot;
use crate::cfg::Configuration;

// TODO: this should actually be our Operation trait
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    key: String,
}

// TODO: the value here needs to actually be the response from Operation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    value: Option<String>,
}

impl Response {
    pub(crate) fn blank() -> Self {
        Self { value: None }
    }
}

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

pub async fn initialize_raft(cfg: &Configuration, db: Database) -> anyhow::Result<(Raft, NodeId)> {
    let mut logs = super::CoyoteLogs::new(&cfg.cluster.log_path).await?;
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
    let state_machine =
        super::state_machine::Store::new(db, cfg.cluster.snapshot_path.clone()).await?;
    let raft = Raft::new(id, config, network, logs, state_machine).await?;
    let has_cluster = raft
        .with_raft_state(|s| {
            s.committed.is_some() || s.membership_state.effective().nodes().count() > 0
        })
        .await?;
    if !has_cluster {
        let disco = Discovery::new(cfg.clone(), raft.clone(), id)?;
        tokio::spawn(async move {
            if let Err(err) = disco.discover_cluster().await {
                tracing::error!(
                    ?err,
                    "discovery failed; this node must be manually initialized"
                );
            }
        });
    }
    Ok((raft, id))
}

#[cfg(test)]
mod tests {
    use fjall::Database;
    use openraft::{StorageIOError, testing::StoreBuilder};
    use tempfile::TempDir;

    use super::{NodeId, TypeConfig};
    use crate::core::cluster::{logs::CoyoteLogs, state_machine::Store};

    struct CoyoteStoreBuilder;

    impl CoyoteStoreBuilder {
        async fn setup() -> anyhow::Result<(TempDir, CoyoteLogs, Store)> {
            let workdir = tempfile::tempdir()?;
            let mut log_path = workdir.path().to_path_buf();
            log_path.push("logs");
            let logs = CoyoteLogs::new(log_path).await?;

            let mut data_path = workdir.path().to_path_buf();
            data_path.push("data");
            let mut snapshot_path = workdir.path().to_path_buf();
            snapshot_path.push("snapshots");
            let db = Database::builder(data_path).open()?;
            let store = Store::new(db, snapshot_path).await?;

            Ok((workdir, logs, store))
        }
    }

    impl StoreBuilder<TypeConfig, CoyoteLogs, Store, TempDir> for CoyoteStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(TempDir, CoyoteLogs, Store), openraft::StorageError<NodeId>> {
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
