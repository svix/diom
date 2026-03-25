// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_cache::{
    CacheModel,
    operations::{CreateCacheOperation, DeleteOperation, SetOperation},
};
use diom_core::types::{Consistency, DurationMs, EntityKey};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_kv::kvcontroller::KvModel;
use diom_namespace::{
    Namespace,
    entities::{CacheConfig, EvictionPolicy, NamespaceName},
};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

pub type CacheNamespace = Namespace<CacheConfig>;

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheSetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl: DurationMs,
}

impl CacheSetIn {
    fn into_model(self, when: Timestamp) -> CacheModel {
        let expiry = when + self.ttl;
        debug_assert!(expiry > Timestamp::UNIX_EPOCH);

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
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,
    #[serde(default = "Consistency::weak")]
    pub consistency: Consistency,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheGetOut {
    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Option<Vec<u8>>,
}

impl CacheGetOut {
    fn from_model(model: KvModel) -> Self {
        Self {
            expiry: model.expiry,
            value: Some(model.value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheDeleteIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteOut {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct CacheCreateNamespaceIn {
    pub name: NamespaceName,
    pub max_storage_bytes: Option<NonZeroU64>,
    #[serde(default)]
    pub eviction_policy: EvictionPolicy,
}

impl From<CacheCreateNamespaceIn> for CreateCacheOperation {
    fn from(v: CacheCreateNamespaceIn) -> Self {
        CreateCacheOperation::new(v.name, v.eviction_policy, v.max_storage_bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheCreateNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
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
    let namespace: CacheNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let now = repl.time.now();

    let operation = SetOperation::new(namespace, data.key.to_string(), data.into_model(now));
    repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(CacheSetOut {}))
}

/// Cache Get
#[aide_annotate(op_id = "v1.cache.get")]
async fn cache_get(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetIn>,
) -> Result<MsgPackOrJson<CacheGetOut>> {
    let namespace: CacheNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    if data.consistency.linearizable() {
        repl.wait_linearizable().await.or_internal_error()?;
    }

    let cache_state = diom_cache::State::init(state.do_not_use_dbs.clone())?;
    let controller = cache_state.controller();

    let model = controller
        .fetch(namespace.id, data.key, repl.time.now())
        .await?;

    let ret = match model {
        Some(m) => CacheGetOut::from_model(m),
        None => CacheGetOut {
            expiry: None,
            value: None,
        },
    };
    Ok(MsgPackOrJson(ret))
}

/// Cache Delete
#[aide_annotate(op_id = "v1.cache.delete")]
async fn cache_del(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheDeleteIn>,
) -> Result<MsgPackOrJson<CacheDeleteOut>> {
    let namespace: CacheNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = DeleteOperation::new(namespace, data.key.to_string());
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(CacheDeleteOut {
        success: resp.success,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetNamespaceIn {
    pub name: NamespaceName,
}

/// Create cache namespace
#[aide_annotate(op_id = "v1.cache.namespace.create")]
async fn cache_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheCreateNamespaceIn>,
) -> Result<MsgPackOrJson<CacheCreateNamespaceOut>> {
    let operation = CreateCacheOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(CacheCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        eviction_policy: resp.eviction_policy,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get cache namespace
#[aide_annotate(op_id = "v1.cache.namespace.get")]
async fn cache_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetNamespaceIn>,
) -> Result<MsgPackOrJson<CacheGetNamespaceOut>> {
    // Ensure we have the latest version of namespace
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: CacheNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(CacheGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        eviction_policy: namespace.config.eviction_policy,
        created: namespace.created,
        updated: namespace.updated,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Cache");

    ApiRouter::new()
        .api_route_with(
            cache_set_path,
            post_with(cache_set, cache_set_operation),
            &tag,
        )
        .api_route_with(
            cache_get_path,
            post_with(cache_get, cache_get_operation),
            &tag,
        )
        .api_route_with(
            cache_create_namespace_path,
            post_with(cache_create_namespace, cache_create_namespace_operation),
            &tag,
        )
        .api_route_with(
            cache_get_namespace_path,
            post_with(cache_get_namespace, cache_get_namespace_operation),
            &tag,
        )
        .api_route_with(
            cache_del_path,
            post_with(cache_del, cache_del_operation),
            &tag,
        )
}
