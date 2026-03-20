// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use coyote_core::Monotime;
use coyote_error::Result;
use coyote_kv::kvcontroller::KvController;
use coyote_namespace::{Namespace, entities::CacheConfig};
use coyote_operations::{BackgroundError, BackgroundResult};
use fjall_utils::{Databases, StorageType};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type CacheNamespace = Namespace<CacheConfig>;

const CACHE_KEYSPACE: &str = "mod_cache";

#[derive(Clone)]
pub struct State {
    persistent_controller: KvController,
    ephemeral_controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            persistent_controller: KvController::new(dbs.persistent, CACHE_KEYSPACE),
            ephemeral_controller: KvController::new(dbs.ephemeral, CACHE_KEYSPACE),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &KvController {
        match storage_type {
            StorageType::Persistent => &self.persistent_controller,
            StorageType::Ephemeral => &self.ephemeral_controller,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

#[derive(Clone)]
pub struct AllNodesWorker {
    state: State,
    time: Monotime,
}

impl coyote_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:cache";

    /// This is a worker function which runs on every node
    ///
    /// It should not mutate the database in any way that could possibly be customer- or
    /// replication-visible; all  mutations should be written through the writer function
    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

        let shutting_down = coyote_core::shutdown::shutting_down_token();

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
    async fn worker_loop(&self, now: Timestamp) -> BackgroundResult<()> {
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
