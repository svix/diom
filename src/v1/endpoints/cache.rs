// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_authorization::RequestedOperation;
use coyote_cache::operations::{CreateCacheOperation, DeleteOperation, SetOperation};
use coyote_core::types::{ByteString, Consistency, DurationMs, EntityKey};
use coyote_derive::aide_annotate;
use coyote_error::{OptionExt, ResultExt};
use coyote_id::Module;
use coyote_kv::kvcontroller::KvModel;
use coyote_namespace::{
    Namespace,
    entities::{CacheConfig, EvictionPolicy, NamespaceName},
};
use coyote_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

fn cache_access_metadata<'a>(
    ns: Option<&'a str>,
    key: &'a EntityKey,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::Cache,
        namespace: ns,
        key: Some(key.as_str()),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                cache_access_metadata(self.namespace.as_deref(), &self.key, $action)
            }
        }
    };
}

pub type CacheNamespace = Namespace<CacheConfig>;

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key", "value"]))]
pub struct CacheSetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    pub value: ByteString,

    /// Time to live in milliseconds
    #[serde(rename = "ttl_ms")]
    pub ttl: DurationMs,
}

request_input!(CacheSetIn, "set");

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

request_input!(CacheGetIn, "get");

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheGetOut {
    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Option<ByteString>,
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

request_input!(CacheDeleteIn, "delete");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteOut {
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetNamespaceOut {
    pub name: NamespaceName,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct CacheCreateNamespaceIn {
    pub name: NamespaceName,
    #[serde(default)]
    pub eviction_policy: EvictionPolicy,
}

namespace_request_input!(CacheCreateNamespaceIn, "create");

impl From<CacheCreateNamespaceIn> for CreateCacheOperation {
    fn from(v: CacheCreateNamespaceIn) -> Self {
        CreateCacheOperation::new(v.name, v.eviction_policy)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheCreateNamespaceOut {
    pub name: NamespaceName,
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

    let operation = SetOperation::new(namespace, data.key.to_string(), Some(data.ttl), data.value);
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

    let cache_state = coyote_cache::State::init(state.do_not_use_dbs.clone())?;
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

namespace_request_input!(CacheGetNamespaceIn, "get");

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
