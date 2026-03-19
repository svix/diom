use diom_core::Monotime;
use diom_error::Result;
use diom_namespace::{Namespace, entities::KeyValueConfig};
use diom_operations::{BackgroundError, BackgroundResult, OperationWriter};
use fjall_utils::{Databases, StorageType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod kvcontroller;
pub mod operations;
pub mod tables;

use crate::kvcontroller::KvController;

pub type KvNamespace = Namespace<KeyValueConfig>;
const KV_KEYSPACE: &str = "mod_kv";

#[derive(Clone)]
pub struct State {
    persistent_controller: KvController,
    ephemeral_controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            persistent_controller: KvController::new(dbs.persistent, KV_KEYSPACE),
            ephemeral_controller: KvController::new(dbs.ephemeral, KV_KEYSPACE),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &KvController {
        match storage_type {
            StorageType::Persistent => &self.persistent_controller,
            StorageType::Ephemeral => &self.ephemeral_controller,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OperationBehavior {
    #[default]
    Upsert,
    Insert,
    Update,
}

#[derive(Clone)]
pub struct AllNodesWorker {
    state: State,
    time: Monotime,
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:kv";

    /// This is a worker function which runs on every node
    ///
    /// It should not mutate the database in any way that could possibly be customer- or
    /// replication-visible; all  mutations should be written through the writer function
    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

        let shutting_down = diom_core::shutdown::shutting_down_token();

        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            self.worker_loop(self.time.last()).await?;
        }

        Ok(())
    }
}

impl AllNodesWorker {
    pub fn new(state: State, time: Monotime) -> Self {
        Self { state, time }
    }

    #[tracing::instrument(skip_all)]
    async fn worker_loop(&self, now: jiff::Timestamp) -> BackgroundResult<()> {
        let mut tasks = tokio::task::JoinSet::new();
        let state = self.state.clone();
        tasks.spawn_blocking(move || {
            state
                .persistent_controller
                .clear_expired_in_background(now, StorageType::Persistent)
        });
        let state = self.state.clone();
        tasks.spawn_blocking(move || {
            state
                .ephemeral_controller
                .clear_expired_in_background(now, StorageType::Ephemeral)
        });
        for result in tasks.join_all().await {
            result.map_err(BackgroundError::Other)?;
        }
        Ok(())
    }
}

/// This is a worker function for this module which runs only on the leader
///
/// It should not mutate the database in any way that could possibly be customer- or
/// replication-visible; all  mutations should be written through the writer function
pub async fn leader_worker<F>(_state: State, _writer: F, _time: Monotime) -> BackgroundResult<()>
where
    F: OperationWriter<operations::KvOperation>,
{
    let shutting_down = diom_core::shutdown::shutting_down_token();

    shutting_down.cancelled().await;

    Ok(())
}
