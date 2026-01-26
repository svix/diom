use openraft::BasicNode;
use serde::{Deserialize, Serialize};

use super::state_machine::StoredSnapshot;

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

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
        Node = Node,
        SnapshotData = StoredSnapshot
);

#[cfg(test)]
mod tests {
    use fjall::Database;
    use openraft::testing::StoreBuilder;
    use openraft::{RaftTypeConfig, StorageIOError};
    use tempfile::TempDir;

    use super::super::logs::DiomLogs;
    use super::super::state_machine::Store;
    use super::TypeConfig;

    type NodeId = <TypeConfig as RaftTypeConfig>::NodeId;

    struct DiomStoreBuilder;

    impl DiomStoreBuilder {
        async fn setup() -> anyhow::Result<(TempDir, DiomLogs, Store)> {
            let workdir = tempfile::tempdir()?;
            let mut log_path = workdir.path().to_path_buf();
            log_path.push("logs");
            let logs = DiomLogs::new(log_path).await?;

            let mut data_path = workdir.path().to_path_buf();
            data_path.push("data");
            let mut snapshot_path = workdir.path().to_path_buf();
            snapshot_path.push("snapshots");
            let db = Database::builder(data_path).open()?;
            let store = Store::new(db, snapshot_path).await?;

            Ok((workdir, logs, store))
        }
    }

    impl StoreBuilder<TypeConfig, DiomLogs, Store, TempDir> for DiomStoreBuilder {
        async fn build(
            &self,
        ) -> Result<(TempDir, DiomLogs, Store), openraft::StorageError<NodeId>> {
            Self::setup().await.map_err(|e| openraft::StorageError::IO {
                source: StorageIOError::write(e),
            })
        }
    }

    #[test]
    fn test_storage() -> anyhow::Result<()> {
        openraft::testing::Suite::test_all(DiomStoreBuilder)?;
        Ok(())
    }
}
