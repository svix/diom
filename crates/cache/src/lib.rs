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
use diom_operations::BackgroundResult;
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
pub struct LeaderWorker<F: diom_operations::OperationWriter<operations::CacheOperation>> {
    state: State,
    time: Monotime,
    cleanup_interval: Duration,
    handle: F,
}

impl<F: diom_operations::OperationWriter<operations::CacheOperation>> LeaderWorker<F> {
    pub fn new(state: State, time: Monotime, cleanup_interval: Duration, handle: F) -> Self {
        Self {
            state,
            time,
            cleanup_interval,
            handle,
        }
    }
}

impl<F: diom_operations::OperationWriter<operations::CacheOperation>>
    diom_operations::workers::BackgroundWorker for LeaderWorker<F>
{
    const NAME: &'static str = "leader-worker:cache";

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
