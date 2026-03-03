// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{num::NonZeroU64, time::Duration};

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_cache::{
    CacheModel,
    operations::{CreateCacheOperation, DeleteOperation, SetOperation},
};
use coyote_derive::aide_annotate;
use coyote_error::{Error, HttpError, ResultExt};
use coyote_namespace::{
    Namespace,
    entities::{CacheConfig, EvictionPolicy, StorageType},
};
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::{cluster::RaftState, types::EntityKey},
    error::Result,
    v1::utils::openapi_tag,
};

pub type CacheNamespace = Namespace<CacheConfig>;

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheSetIn {
    #[validate(nested)]
    pub key: EntityKey,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl: u64,
}

impl CacheSetIn {
    fn into_model(self) -> CacheModel {
        let expiry = Timestamp::now() + Duration::from_millis(self.ttl);

        CacheModel {
            expiry: Some(expiry),
            value: self.value,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheSetOut {}

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheGetIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheGetOut {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl CacheGetOut {
    fn from_model(key: EntityKey, model: CacheModel) -> Self {
        Self {
            key,
            expiry: model.expiry,
            value: model.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheDeleteIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteOut {
    pub deleted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheCreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
    #[serde(default)]
    pub eviction_policy: EvictionPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheCreateNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Cache Set
#[aide_annotate(op_id = "v1.cache.set")]
async fn cache_set(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheSetIn>,
) -> Result<MsgPackOrJson<CacheSetOut>> {
    let key_str = data.key.to_string();
    // TODO: Presumably this should only need to happen in
    // the consensus layer, but currently raft seems to
    // break if an operation with a non-existent namespace is attempted,
    // so do this here for now as a quick check that the namespace
    // exists:
    let _cache_store = state.get_cache_store_by_key(&key_str).await?;

    let operation = SetOperation::new(key_str, data.into_model());
    repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(CacheSetOut {}))
}

/// Cache Get
#[aide_annotate(op_id = "v1.cache.get")]
async fn cache_get(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetIn>,
) -> Result<MsgPackOrJson<CacheGetOut>> {
    let mut cache_store = state.get_cache_store_by_key(&data.key.0).await?;
    let model = cache_store
        .get(&data.key)?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(MsgPackOrJson(CacheGetOut::from_model(data.key, model)))
}

/// Cache Delete
#[aide_annotate(op_id = "v1.cache.delete")]
async fn cache_del(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheDeleteIn>,
) -> Result<MsgPackOrJson<CacheDeleteOut>> {
    let key_str = data.key.to_string();
    let operation = DeleteOperation::new(key_str);
    repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(CacheDeleteOut { deleted: true }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetNamespaceIn {
    pub name: String,
}

/// Create cache namespace
#[aide_annotate(op_id = "v1.cache.namespace.create")]
async fn cache_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheCreateNamespaceIn>,
) -> Result<MsgPackOrJson<CacheCreateNamespaceOut>> {
    let operation = CreateCacheOperation::new(
        data.name,
        data.eviction_policy,
        data.storage_type,
        data.max_storage_bytes,
    );
    let resp = repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(CacheCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
        eviction_policy: resp.eviction_policy,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get cache namespace
#[aide_annotate(op_id = "v1.cache.namespace.get")]
async fn cache_get_namespace(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetNamespaceIn>,
) -> Result<MsgPackOrJson<CacheGetNamespaceOut>> {
    let namespace: CacheNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    Ok(MsgPackOrJson(CacheGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        storage_type: namespace.storage_type,
        eviction_policy: namespace.config.eviction_policy,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
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
            "/cache/namespace/create",
            post_with(cache_create_namespace, cache_create_namespace_operation),
            &tag,
        )
        .api_route_with(
            "/cache/namespace/get",
            post_with(cache_get_namespace, cache_get_namespace_operation),
            &tag,
        )
        .api_route_with(
            "/cache/delete",
            post_with(cache_del, cache_del_operation),
            &tag,
        )
}
