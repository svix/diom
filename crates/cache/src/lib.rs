// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use diom_error::Result;
use diom_kv::kvcontroller::KvController;
use diom_namespace::{Namespace, entities::CacheConfig};
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

/// This is the worker function for this module, it does background cleanup and accounting.
/// It deletes expired entries from the database and evicts entries if the Cache is configured to do so.
pub async fn worker<F>(dbs: Databases, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
    let controller = KvController::new(dbs.persistent, CACHE_KEYSPACE);

    loop {
        if is_shutting_down() {
            break;
        }

        timer.tick().await;

        let now = Timestamp::now();
        match controller.clear_expired(now) {
            Ok(()) => {}
            Err(e) => {
                tracing::error!(error = ?e, "Failed to clean.");
            }
        };

        // FIXME: also do cache eviction once that's implemented
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}
