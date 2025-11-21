// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::{post}};
use axum::{Json, extract::State};
use coyote_derive::aide_annotate;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use validator::Validate;
use std::sync::Arc;
use dashmap::DashMap;

use crate::{
    AppState, core::types::EntityKey, v1::utils::{ValidatedJson, openapi_tag},
    error::{Result, Error, HttpError},

};

#[derive(Clone)]
pub struct KvStore {
    store: Arc<DashMap<String, KvModel>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvModel {
    #[validate]
    pub key: EntityKey,

    // FIXME: should be datetime
    /// Time of expiry
    pub expires_at: u64,

    // FIXME: change to Bytes
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OperationBehavior {
    Upsert,
    Insert,
    Update
}

impl Default for OperationBehavior {
    fn default() -> Self {
        Self::Upsert
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetIn {
    #[validate]
    pub key: EntityKey,
    // FIXME: validate all fields
    pub expires_at: u64,
    // TODO: add pub expire_in: u64,

    // FIXME: do we want it here? I think we probably want separate commands for insert, upsert,
    // and update? Or does it get weird?
    #[serde(default)]
    pub behavior: OperationBehavior,

    // FIXME: what to do with TTL? Does it get updated on a set, not?

    // FIXME: change to Bytes
    pub value: String,
}

impl Into<KvModel> for KvSetIn {
    fn into(self) -> KvModel {
        let Self { key, expires_at, value, behavior: _ } = self;

        KvModel {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    #[validate]
    pub key: EntityKey,

    // FIXME: should be datetime
    /// Time of expiry
    pub expires_at: u64,

    // FIXME: change to Bytes
    pub value: String,
}

impl From<KvModel> for KvGetOut {
    fn from(model: KvModel) -> Self {
        let KvModel { key, expires_at, value } = model;

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
    pub key: EntityKey,
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
    let key_str = data.key.to_string();
    let behavior = data.behavior.clone();
    let model: KvModel = data.into();

    match behavior {
        OperationBehavior::Insert => {
            // Atomically insert only if key doesn't exist
            use dashmap::mapref::entry::Entry;
            match kv_store.store.entry(key_str) {
                Entry::Vacant(entry) => {
                    entry.insert(model);
                }
                Entry::Occupied(_) => {
                    return Err(Error::http(HttpError::conflict(None, None)));
                }
            }
        }
        OperationBehavior::Update => {
            // Atomically update only if key exists
            match kv_store.store.get_mut(&key_str) {
                Some(mut entry) => {
                    *entry = model;
                }
                None => {
                    return Err(Error::http(HttpError::not_found(None, None)));
                }
            }
        }
        OperationBehavior::Upsert => {
            kv_store.store.insert(key_str, model);
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
    let key_str = data.key.to_string();

    let model = kv_store
        .store
        .get(&key_str)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let ret: KvGetOut = model.value().clone().into();
    Ok(Json(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvDeleteIn>,
) -> Result<Json<KvDeleteOut>> {
    let key_str = data.key.to_string();
    let deleted = kv_store.store.remove(&key_str).is_some();
    let ret = KvDeleteOut { deleted };
    Ok(Json(ret))
}

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route("/kv/set", post(kv_set))
        .api_route("/kv/get", post(kv_get))
        .api_route("/kv/delete", post(kv_del))
}
