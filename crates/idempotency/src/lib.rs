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

use diom_core::Monotime;
use diom_error::Result;
use diom_kv::kvcontroller::KvController;
use diom_namespace::{Namespace, entities::IdempotencyConfig};
use diom_operations::{BackgroundError, BackgroundResult};
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
    persistent_controller: KvController,
    ephemeral_controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            persistent_controller: KvController::new(dbs.persistent, IDEMPOTENCY_KEYSPACE),
            ephemeral_controller: KvController::new(dbs.ephemeral, IDEMPOTENCY_KEYSPACE),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &KvController {
        match storage_type {
            StorageType::Persistent => &self.persistent_controller,
            StorageType::Ephemeral => &self.ephemeral_controller,
        }
    }
}

#[derive(Clone)]
pub struct AllNodesWorker {
    state: State,
    time: Monotime,
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:idempotency";

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
            self.worker_loop(self.time.now()).await?;
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
