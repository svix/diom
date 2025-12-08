// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use coyote_derive::aide_annotate;
use dashmap::DashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    v1::utils::{openapi_tag, ValidatedJson},
    AppState,
};

// TODO FIXME:
// * The lock unwrap()s. I didn't bother with removing them because we'll change the data
// structure before going to prod.

#[derive(Clone)]
pub struct KvStore {
    store: Arc<DashMap<Arc<EntityKey>, KvModel>>,
    expiry: Arc<Mutex<expiry::ExpiryHeap>>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
            expiry: Arc::new(Mutex::new(expiry::ExpiryHeap::new())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvModel {
    #[validate]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetIn {
    #[validate]
    pub key: Arc<EntityKey>,
    // FIXME: validate all fields
    /// Time to live in milliseconds
    pub expire_in: u64,

    // FIXME: do we want it here? I think we probably want separate commands for insert, upsert,
    // and update? Or does it get weird?
    #[serde(default)]
    pub behavior: OperationBehavior,

    // FIXME: what to do with TTL? Does it get updated on a set, not?

    // FIXME: change to Bytes
    pub value: String,
}

impl KvSetIn {
    fn into_model(self) -> KvModel {
        let KvSetIn {
            key,
            expire_in,
            value,
            behavior: _,
        } = self;

        let expires_at = Utc::now() + chrono::Duration::milliseconds(expire_in as i64);

        KvModel {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    #[validate]
    pub key: Arc<EntityKey>,

    /// Time of expiry
    pub expires_at: DateTime<Utc>,

    // FIXME: change to Bytes
    pub value: String,
}

impl From<KvModel> for KvGetOut {
    fn from(model: KvModel) -> Self {
        let KvModel {
            key,
            expires_at,
            value,
        } = model;

        Self {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteIn {
    #[validate]
    pub key: Arc<EntityKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteOut {
    pub deleted: bool,
}

/// KV Set
#[aide_annotate(op_id = "v1.kv.set")]
async fn kv_set(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvSetIn>,
) -> Result<Json<KvSetOut>> {
    let key = data.key.clone();
    let behavior = data.behavior.clone();
    let model = data.into_model();

    let expires_at = model.expires_at;

    match behavior {
        OperationBehavior::Insert => {
            // Atomically insert only if key doesn't exist
            use dashmap::mapref::entry::Entry;
            match kv_store.store.entry(Arc::clone(&key)) {
                Entry::Vacant(entry) => {
                    entry.insert(model);
                    // Insert into expiry heap
                    kv_store.expiry.lock().unwrap().insert(expires_at, key);
                }
                Entry::Occupied(_) => {
                    return Err(Error::http(HttpError::conflict(None, None)));
                }
            }
        }
        OperationBehavior::Update => {
            // Atomically update only if key exists
            match kv_store.store.get_mut(&key) {
                Some(mut entry) => {
                    *entry = model;
                    // Add new expiry entry (don't need to remove old one, as it will be ignored)
                    kv_store.expiry.lock().unwrap().insert(expires_at, key);
                }
                None => {
                    return Err(Error::http(HttpError::not_found(None, None)));
                }
            }
        }
        OperationBehavior::Upsert => {
            kv_store
                .expiry
                .lock()
                .unwrap()
                .insert(expires_at, Arc::clone(&key));
            kv_store.store.insert(key, model);
        }
    }

    let ret = KvSetOut {};
    Ok(Json(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvGetIn>,
) -> Result<Json<KvGetOut>> {
    let key = data.key;

    let model = kv_store
        .store
        .get(&key)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    // Check if the key has expired
    let now = Utc::now();
    if model.expires_at <= now {
        // Key has expired, remove it and return not found
        drop(model); // Release the read lock before removing
        kv_store.store.remove(&key);
        return Err(Error::http(HttpError::not_found(None, None)));
    }

    let ret: KvGetOut = model.value().clone().into();
    Ok(Json(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvDeleteIn>,
) -> Result<Json<KvDeleteOut>> {
    let key = data.key;
    let deleted = kv_store.store.remove(&key).is_some();
    let ret = KvDeleteOut { deleted };
    Ok(Json(ret))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route_with("/kv/set", post_with(kv_set, kv_set_operation), &tag)
        .api_route_with("/kv/get", post_with(kv_get, kv_get_operation), &tag)
        .api_route_with("/kv/delete", post_with(kv_del, kv_del_operation), &tag)
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    let expiry = state.kv_store.expiry;
    loop {
        // FIXME: this is not good to lock for such a long time, but we don't care as we'll change
        // the data structure anyway.
        let mut expiry = expiry.lock().unwrap();
        while expiry.peek().is_some_and(|x| x.expired(Utc::now())) {
            expiry.pop();
        }
        drop(expiry);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

mod expiry {
    use chrono::{DateTime, Utc};
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::Arc;

    use crate::core::types::EntityKey;

    // Can have two data structures.
    // Hashmap with TTL (separate to the main one)
    // Binary heap for the scanning - has the key.
    //  If the expiry timestamp is not the same, assume there's another entry.

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
        expires_at: DateTime<Utc>,
        key: Arc<EntityKey>,
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
