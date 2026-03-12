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

use diom_error::Result;
use diom_kv::kvcontroller::KvController;
use diom_namespace::{Namespace, entities::IdempotencyConfig};
use fjall_utils::{Databases, StorageType};
use serde::{Deserialize, Serialize};

pub type IdempotencyNamespace = Namespace<IdempotencyConfig>;

const IDEMPOTENCY_KEYSPACE: &str = "mod_idempotency";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(crate) enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress,
    /// Request completed successfully with a response
    Completed { response: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartResult {
    Started,
    Locked,
    Completed { response: Vec<u8> },
}

impl From<IdempotencyState> for Vec<u8> {
    fn from(state: IdempotencyState) -> Self {
        rmp_serde::to_vec(&state).expect("Failed to serialize IdempotencyState")
    }
}

impl From<Vec<u8>> for IdempotencyState {
    fn from(value: Vec<u8>) -> Self {
        rmp_serde::from_slice(&value).expect("Failed to deserialize IdempotencyState")
    }
}

#[derive(Clone)]
pub struct State {
    persistent_controller: KvController,
    ephemeral_controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            persistent_controller: KvController::new(
                StorageType::Persistent,
                dbs.persistent,
                IDEMPOTENCY_KEYSPACE,
            ),
            ephemeral_controller: KvController::new(
                StorageType::Ephemeral,
                dbs.ephemeral,
                IDEMPOTENCY_KEYSPACE,
            ),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &KvController {
        match storage_type {
            StorageType::Persistent => &self.persistent_controller,
            StorageType::Ephemeral => &self.ephemeral_controller,
        }
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
///
/// It should not mutate the database in any way that could possibly be customer- or
/// replication-visible; all  mutations should be written through the writer function
pub async fn worker<F>(state: State, writer: F) -> diom_operations::BackgroundResult<()>
where
    F: AsyncFn(
        operations::IdempotencyOperation,
    ) -> diom_operations::BackgroundResult<operations::Response>,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

    let shutting_down = diom_core::shutdown::shutting_down_token();

    while shutting_down
        .run_until_cancelled(timer.tick())
        .await
        .is_some()
    {
        worker_loop(&state, &writer).await?;
    }

    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn worker_loop<F>(state: &State, writer: &F) -> diom_operations::BackgroundResult<()>
where
    F: AsyncFn(
        operations::IdempotencyOperation,
    ) -> diom_operations::BackgroundResult<operations::Response>,
{
    writer(operations::ClearExpiredOperation::new(state.persistent_controller.storage_type).into())
        .await?;
    writer(operations::ClearExpiredOperation::new(state.ephemeral_controller.storage_type).into())
        .await?;
    Ok(())
}
