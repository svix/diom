use std::time::Duration;

use diom_core::Monotime;
use diom_error::Result;
use diom_namespace::{Namespace, entities::KeyValueConfig};
use diom_operations::{BackgroundError, BackgroundResult};
use fjall_utils::Databases;
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
    controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(dbs.persistent, KV_KEYSPACE),
        })
    }

    pub fn controller(&self) -> &KvController {
        &self.controller
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
    cleanup_interval: Duration,
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:kv";

    /// This is a worker function which runs on every node
    ///
    /// It should not mutate the database in any way that could possibly be customer- or
    /// replication-visible; all  mutations should be written through the writer function
    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(self.cleanup_interval);

        let shutting_down = diom_core::shutdown::shutting_down_token();

        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            self.worker_loop(self.time.now()).await?;
        }

        Ok(())
    }
}

impl AllNodesWorker {
    pub fn new(state: State, time: Monotime, cleanup_interval: Duration) -> Self {
        Self {
            state,
            time,
            cleanup_interval,
        }
    }

    #[tracing::instrument(skip_all)]
    async fn worker_loop(&self, now: jiff::Timestamp) -> BackgroundResult<()> {
        let mut tasks = tokio::task::JoinSet::new();
        let state = self.state.clone();
        tasks.spawn_blocking(move || state.controller.clear_expired_in_background(now.into()));
        for result in tasks.join_all().await {
            result.map_err(BackgroundError::Other)?;
        }
        Ok(())
    }
}
