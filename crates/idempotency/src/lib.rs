// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Idempotency module.
//!
//! This module implements idempotency, so people can use it to implement idempotency in their web
//! services.
//!
//! ## TODO FIXME
//! - The API probably needs changing.

pub mod operations;

use std::time::Duration;

use diom_core::{Monotime, types::Metadata};
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
pub(crate) enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress,
    /// Request completed successfully with a response
    Completed {
        response: Vec<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context: Option<Metadata>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartResult {
    Started,
    Locked,
    Completed {
        response: Vec<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context: Option<Metadata>,
    },
}

impl From<IdempotencyState> for Vec<u8> {
    fn from(state: IdempotencyState) -> Self {
        rmp_serde::to_vec_named(&state).expect("Failed to serialize IdempotencyState")
    }
}

impl From<Vec<u8>> for IdempotencyState {
    fn from(value: Vec<u8>) -> Self {
        rmp_serde::from_slice(&value).expect("Failed to deserialize IdempotencyState")
    }
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
            while self.state.controller.has_expired(self.time.now()).await {
                self.handle
                    .write_request(operations::ClearExpiredOperation::new())
                    .await?;
            }
        }

        Ok(())
    }
}
