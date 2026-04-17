use std::time::Duration;

use diom_core::Monotime;
use diom_error::Result;
use diom_namespace::{Namespace, entities::KeyValueConfig};
use diom_operations::BackgroundResult;
use fjall_utils::Databases;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod kvcontroller;
pub mod operations;
pub mod storage;

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
pub struct LeaderWorker<F: diom_operations::OperationWriter<operations::KvOperation>> {
    state: State,
    time: Monotime,
    cleanup_interval: Duration,
    handle: F,
}

impl<F: diom_operations::OperationWriter<operations::KvOperation>> LeaderWorker<F> {
    pub fn new(state: State, time: Monotime, cleanup_interval: Duration, handle: F) -> Self {
        Self {
            state,
            time,
            cleanup_interval,
            handle,
        }
    }
}

impl<F: diom_operations::OperationWriter<operations::KvOperation>>
    diom_operations::workers::BackgroundWorker for LeaderWorker<F>
{
    const NAME: &'static str = "leader-worker:kv";

    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(self.cleanup_interval);

        let shutting_down = diom_core::shutdown::shutting_down_token();

        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            let handle = self.handle.clone();
            self.state
                .controller
                .clear_expired_in_raft_until_done(self.time.now_utm(), async move || {
                    handle
                        .write_request(operations::ClearExpiredOperation::new())
                        .await
                })
                .await?;
        }

        Ok(())
    }
}
