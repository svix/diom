// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # KV Store Module
//!
//! This module implements a key-value store with automatic expiration.
//!
//! ## Data Structure Design
//!
//! The KV store uses two separate data structures
//!
//! 1. Main Store (DashMap) - Primary storage for key-value pairs with their expiration timestamps.
//! 2. Expiry Heap (BinaryHeap) - Maintains keys sorted by expiration time for efficient cleanup.
//!
//! ## How It Works
//!
//! - On Write: Keys are inserted into both the store and expiry heap
//! - On Update: New expiry entries are added (old ones remain but are ignored during cleanup)
//! - On Read: Expiration is checked; expired keys are removed and return not found
//! - Background Worker: Periodically scans the heap and removes expired keys from the store
//!
//! The expiry heap may contain stale entries (from updates/deletes), but these are safely
//! ignored during cleanup since the worker checks if the key exists and if the expiration
//! matches before removal.
//!
//! ## TODO FIXME
//! - The lock unwrap()s. I didn't bother with removing them because we'll change the data
//!   structure before going to prod.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    AppState,
};

struct KvStoreState {
    store: HashMap<Arc<EntityKey>, KvModel>,
    expiry: expiry::ExpiryHeap,
}

#[derive(Clone)]
pub struct KvStore {
    state: Arc<RwLock<KvStoreState>>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(KvStoreState {
                store: HashMap::new(),
                expiry: expiry::ExpiryHeap::new(),
            })),
        }
    }

    pub fn set(
        &self,
        key: Arc<EntityKey>,
        model: KvModel,
        behavior: OperationBehavior,
    ) -> Result<()> {
        let expires_at = model.expires_at;

        match behavior {
            OperationBehavior::Insert => {
                // Atomically insert only if key doesn't exist
                let mut state = self.state.write().unwrap();
                if state.store.contains_key(&key) {
                    return Err(Error::http(HttpError::conflict(None, None)));
                }
                state.store.insert(Arc::clone(&key), model);
                state.expiry.insert(expires_at, key);
            }
            OperationBehavior::Update => {
                // Atomically update only if key exists
                let mut state = self.state.write().unwrap();
                if !state.store.contains_key(&key) {
                    return Err(Error::http(HttpError::not_found(None, None)));
                }
                state.store.insert(Arc::clone(&key), model);
                state.expiry.insert(expires_at, key);
            }
            OperationBehavior::Upsert => {
                let mut state = self.state.write().unwrap();
                state.expiry.insert(expires_at, Arc::clone(&key));
                state.store.insert(key, model);
            }
        }

        Ok(())
    }

    pub fn get(&self, key: &EntityKey) -> Result<KvModel> {
        let now = Utc::now();

        // First, check if the key exists and is not expired
        {
            let state = self.state.read().unwrap();
            if let Some(model) = state.store.get(key) {
                if model.expires_at > now {
                    return Ok(model.clone());
                }
            }
        }

        // If we get here, either the key doesn't exist or it's expired
        // If expired, remove it
        {
            let mut state = self.state.write().unwrap();
            if let Some(model) = state.store.get(key) {
                if model.expires_at <= now {
                    state.store.remove(key);
                }
            }
        }

        Err(Error::http(HttpError::not_found(None, None)))
    }

    pub fn delete(&self, key: &Arc<EntityKey>) -> bool {
        self.state.write().unwrap().store.remove(key).is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvModel {
    #[validate(nested)]
    pub key: Arc<EntityKey>,

    /// Time of expiry
    pub expires_at: DateTime<Utc>,

    // FIXME: change to Bytes
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OperationBehavior {
    #[default]
    Upsert,
    Insert,
    Update,
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        {
            let mut kv_state = state.kv_store.state.write().unwrap();
            while kv_state
                .expiry
                .peek()
                .is_some_and(|x| x.expired(Utc::now()))
            {
                if let Some(expiry_item) = kv_state.expiry.pop() {
                    if let Some(value) = kv_state.store.get(&expiry_item.key) {
                        // If the expiry is the same or older than what we expect, we should expire the item.
                        if value.expires_at <= expiry_item.expires_at {
                            kv_state.store.remove(&expiry_item.key);
                        }
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}

mod expiry {
    use chrono::{DateTime, Utc};
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::Arc;

    use crate::core::types::EntityKey;

    pub(super) struct ExpiryHeap {
        heap: BinaryHeap<ExpiryState>,
    }

    impl ExpiryHeap {
        pub(super) fn new() -> Self {
            Self {
                heap: BinaryHeap::new(),
            }
        }

        pub(super) fn insert(&mut self, expires_at: DateTime<Utc>, key: Arc<EntityKey>) {
            self.heap.push(ExpiryState { expires_at, key });
        }

        pub(super) fn peek(&self) -> Option<&ExpiryState> {
            self.heap.peek()
        }

        pub(super) fn pop(&mut self) -> Option<ExpiryState> {
            self.heap.pop()
        }
    }

    #[derive(Eq, PartialEq)]
    pub(super) struct ExpiryState {
        /// The timestamp of when to expire.
        pub(super) expires_at: DateTime<Utc>,
        pub(super) key: Arc<EntityKey>,
    }

    impl ExpiryState {
        pub(super) fn expired(&self, now: DateTime<Utc>) -> bool {
            self.expires_at <= now
        }
    }

    impl Ord for ExpiryState {
        fn cmp(&self, other: &Self) -> Ordering {
            // Comparing in reverse so that we get a min heap
            other.expires_at.cmp(&self.expires_at)
        }
    }

    impl PartialOrd for ExpiryState {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}
