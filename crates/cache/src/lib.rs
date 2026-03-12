// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use coyote_error::Result;
use coyote_kv::kvcontroller::KvController;
use coyote_namespace::{Namespace, entities::CacheConfig};
use coyote_operations::OperationWriter;
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
            persistent_controller: KvController::new(
                StorageType::Persistent,
                dbs.persistent,
                CACHE_KEYSPACE,
            ),
            ephemeral_controller: KvController::new(
                StorageType::Ephemeral,
                dbs.ephemeral,
                CACHE_KEYSPACE,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

/// This is the worker function for this module, it does background cleanup and accounting.
///
/// It should not mutate the database in any way that could possibly be customer- or
/// replication-visible; all  mutations should be written through the writer function
pub async fn worker<F>(state: State, writer: F) -> coyote_operations::BackgroundResult<()>
where
    F: OperationWriter,
    <F as OperationWriter>::Request: From<operations::CacheOperation>,
    operations::Response: TryFrom<<F as OperationWriter>::Response>,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

    let shutting_down = coyote_core::shutdown::shutting_down_token();

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
pub async fn worker_loop<F>(state: &State, writer: &F) -> coyote_operations::BackgroundResult<()>
where
    F: OperationWriter,
    <F as OperationWriter>::Request: From<operations::CacheOperation>,
    operations::Response: TryFrom<<F as OperationWriter>::Response>,
{
    writer
        .write_request(operations::ClearExpiredOperation::new(
            state.persistent_controller.storage_type,
        ))
        .await?;
    writer
        .write_request(operations::ClearExpiredOperation::new(
            state.ephemeral_controller.storage_type,
        ))
        .await?;
    Ok(())
}
