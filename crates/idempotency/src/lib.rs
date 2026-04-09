//! # Idempotency module.
//!
//! This module implements idempotency, so people can use it to implement idempotency in their web
//! services.
//!
//! ## TODO FIXME
//! - The API probably needs changing.

pub mod operations;
pub(crate) mod storage;

use std::time::Duration;

use diom_core::{
    Monotime,
    types::{ByteString, Metadata},
};
use diom_error::Result;
use diom_kv::kvcontroller::KvController;
use diom_namespace::{Namespace, entities::IdempotencyConfig};
use diom_operations::BackgroundResult;
use fjall_utils::Databases;
use serde::{Deserialize, Serialize};

pub type IdempotencyNamespace = Namespace<IdempotencyConfig>;

const IDEMPOTENCY_KEYSPACE: &str = "mod_idempotency";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartResult {
    Started,
    Locked,
    Completed {
        response: ByteString,
        context: Option<Metadata>,
    },
}

#[derive(Clone)]
pub struct State {
    controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(dbs.persistent, IDEMPOTENCY_KEYSPACE),
        })
    }

    pub fn controller(&self) -> &KvController {
        &self.controller
    }
}

#[derive(Clone)]
pub struct LeaderWorker<F: diom_operations::OperationWriter<operations::IdempotencyOperation>> {
    state: State,
    time: Monotime,
    cleanup_interval: Duration,
    handle: F,
}

impl<F: diom_operations::OperationWriter<operations::IdempotencyOperation>> LeaderWorker<F> {
    pub fn new(state: State, time: Monotime, cleanup_interval: Duration, handle: F) -> Self {
        Self {
            state,
            time,
            cleanup_interval,
            handle,
        }
    }
}

impl<F: diom_operations::OperationWriter<operations::IdempotencyOperation>>
    diom_operations::workers::BackgroundWorker for LeaderWorker<F>
{
    const NAME: &'static str = "leader-worker:idempotency";

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
