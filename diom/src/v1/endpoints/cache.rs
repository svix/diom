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

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use diom_derive::aide_annotate;
use dashmap::DashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    v1::utils::{openapi_tag, ValidatedJson},
    AppState,
};

#[derive(Clone)]
pub struct CacheStore {
    store: Arc<DashMap<String, CacheModel>>,
}

impl Default for CacheStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    #[validate]
    pub key: EntityKey,

    // FIXME: should be datetime
    /// Time of expiry
    pub expires_at: u64,

    // FIXME: change to Bytes
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheSetIn {
    #[validate]
    pub key: EntityKey,
    // FIXME: validate all fields
    pub expires_at: u64,
    // TODO: add pub expire_in: u64,

    // FIXME: what to do with TTL? Does it get updated on a set, not?

    // FIXME: change to Bytes
    pub value: String,
}

impl From<CacheSetIn> for CacheModel {
    fn from(val: CacheSetIn) -> Self {
        let CacheSetIn {
            key,
            expires_at,
            value,
        } = val;

        CacheModel {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheGetIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheGetOut {
    #[validate]
    pub key: EntityKey,

    // FIXME: should be datetime
    /// Time of expiry
    pub expires_at: u64,

    // FIXME: change to Bytes
    pub value: String,
}

impl From<CacheModel> for CacheGetOut {
    fn from(model: CacheModel) -> Self {
        let CacheModel {
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
pub struct CacheDeleteIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteOut {
    pub deleted: bool,
}

/// Cache Set
#[aide_annotate(op_id = "v1.cache.set")]
async fn cache_set(
    State(AppState { cache_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<CacheSetIn>,
) -> Result<Json<CacheSetOut>> {
    let key_str = data.key.to_string();
    let model: CacheModel = data.into();
    cache_store.store.insert(key_str, model);

    let ret = CacheSetOut {};
    Ok(Json(ret))
}

/// Cache Get
#[aide_annotate(op_id = "v1.cache.get")]
async fn cache_get(
    State(AppState { cache_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<CacheGetIn>,
) -> Result<Json<CacheGetOut>> {
    let key_str = data.key.to_string();

    let model = cache_store
        .store
        .get(&key_str)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let ret: CacheGetOut = model.value().clone().into();
    Ok(Json(ret))
}

/// Cache Delete
#[aide_annotate(op_id = "v1.cache.delete")]
async fn cache_del(
    State(AppState { cache_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<CacheDeleteIn>,
) -> Result<Json<CacheDeleteOut>> {
    let key_str = data.key.to_string();
    let deleted = cache_store.store.remove(&key_str).is_some();
    let ret = CacheDeleteOut { deleted };
    Ok(Json(ret))
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

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Cache");

    ApiRouter::new()
        .api_route_with(
            "/cache/set",
            post_with(cache_set, cache_set_operation),
            &tag,
        )
        .api_route_with(
            "/cache/get",
            post_with(cache_get, cache_get_operation),
            &tag,
        )
        .api_route_with(
            "/cache/delete",
            post_with(cache_del, cache_del_operation),
            &tag,
        )
}
