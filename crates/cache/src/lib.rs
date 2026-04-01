// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use std::time::Duration;

use diom_core::Monotime;
use diom_error::Result;
use diom_kv::kvcontroller::KvController;
use diom_namespace::{Namespace, entities::CacheConfig};
use diom_operations::{BackgroundError, BackgroundResult};
use fjall_utils::Databases;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type CacheNamespace = Namespace<CacheConfig>;

const CACHE_KEYSPACE: &str = "mod_cache";

#[derive(Clone)]
pub struct State {
    controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(dbs.ephemeral, CACHE_KEYSPACE),
        })
    }

    pub fn controller(&self) -> &KvController {
        &self.controller
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
    cleanup_interval: Duration,
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:cache";

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
    async fn worker_loop(&self, now: Timestamp) -> BackgroundResult<()> {
        let mut tasks = tokio::task::JoinSet::new();
        let state = self.state.clone();
        tasks.spawn_blocking(move || state.controller.clear_expired_in_background(now));
        for result in tasks.join_all().await {
            result.map_err(BackgroundError::Other)?;
        }
        Ok(())
    }
}
