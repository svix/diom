// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Cache module.
//!
//! The idea of having a separate cache module is that we can aggressively evict from this one when
//! under memory pressure, which we don't want to do with kv store (which can't be lost!). So cache
//! is really for caching things, and not a kv store. That's why they should maybe be different.
//! So for example we can configure eviction policies like: swap, drop, and behaviors like lru,
//! whatever.
//!
//! FIXME:
//! * Potentially we could merge it with KV and just with the "group configuration" behavior we can
//!   define the cache behavior. So we don't actually need a different backend?
//!   * Though even if we do that, maybe cache should be an alias for kv with a default base
//!     configuration?
//! * If we end up making them separate: this can potentially reuse code from kv-store?

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    AppState,
};

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) store: Arc<RwLock<HashMap<String, CacheModel>>>,
}

impl Default for CacheStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, model: CacheModel) {
        self.store.write().unwrap().insert(key, model);
    }

    pub fn get(&self, key: &str) -> Result<CacheModel> {
        self.store
            .read()
            .unwrap()
            .get(key)
            .cloned()
            .ok_or_else(|| Error::http(HttpError::not_found(None, None)))
    }

    pub fn delete(&self, key: &str) -> bool {
        self.store.write().unwrap().remove(key).is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    #[validate(nested)]
    pub key: EntityKey,

    // FIXME: should be datetime
    /// Time of expiry
    pub expires_at: u64,

    // FIXME: change to Bytes
    pub value: String,
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(_state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
