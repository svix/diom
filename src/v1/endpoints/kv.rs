// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_core::types::EntityKey;
use coyote_derive::aide_annotate;
use coyote_error::{Error, HttpError, ResultExt};
use coyote_kv::{
    KvNamespace,
    kvcontroller::{KvModel, OperationBehavior},
    operations::{CreateKvOperation, DeleteOperation, SetOperation},
};
use coyote_namespace::entities::StorageType;
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
pub use crate::v1::modules::kv::worker;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvSetIn {
    #[validate(nested)]
    pub key: EntityKey,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[validate(range(min = 1))]
    pub ttl: Option<u64>,

    #[serde(default)]
    pub behavior: OperationBehavior,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvGetIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl KvGetOut {
    fn from_model(key: EntityKey, model: KvModel) -> Self {
        Self {
            key,
            expiry: model.expiry,
            value: model.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvDeleteIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteOut {
    pub deleted: bool,
}

/// KV Set
#[aide_annotate(op_id = "v1.kv.set")]
async fn kv_set(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvSetIn>,
) -> Result<MsgPackOrJson<KvSetOut>> {
    let namespace: KvNamespace = state
        .namespace_state
        .fetch_namespace(data.key.namespace())?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = SetOperation::new(namespace, data.key, data.value, data.ttl, data.behavior);
    repl.client_write(operation).await.map_err_generic()?.0?;

    let ret = KvSetOut {};
    Ok(MsgPackOrJson(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvGetIn>,
) -> Result<MsgPackOrJson<KvGetOut>> {
    let namespace: KvNamespace = state
        .namespace_state
        .fetch_namespace(data.key.namespace())?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    repl.raft.ensure_linearizable().await.map_err_generic()?;

    // FIXME: this state should be passed, not created every time.
    let kv_state = coyote_kv::State::init(state.do_not_use_dbs.clone())?;
    let controller = kv_state.controller(namespace.storage_type);

    let model = controller.fetch(namespace.id, &data.key, Timestamp::now())?;

    let ret = match model {
        Some(m) => KvGetOut::from_model(data.key, m),
        None => {
            return Err(Error::http(HttpError::not_found(
                None,
                Some("Key not found".to_string()),
            )));
        }
    };
    Ok(MsgPackOrJson(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvDeleteIn>,
) -> Result<MsgPackOrJson<KvDeleteOut>> {
    let namespace: KvNamespace = state
        .namespace_state
        .fetch_namespace(data.key.namespace())?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let key = data.key;
    let operation = DeleteOperation::new(namespace, key);
    repl.client_write(operation).await.map_err_generic()?.0?;

    let ret = KvDeleteOut { deleted: true };
    Ok(MsgPackOrJson(ret))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvGetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvGetNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvCreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvCreateNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create KV namespace
#[aide_annotate(op_id = "v1.kv.namespace.create")]
async fn kv_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvCreateNamespaceIn>,
) -> Result<MsgPackOrJson<KvCreateNamespaceOut>> {
    let operation = CreateKvOperation::new(data.name, data.storage_type, data.max_storage_bytes);
    let resp = repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(KvCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get KV namespace
#[aide_annotate(op_id = "v1.kv.namespace.get")]
async fn kv_get_namespace(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<KvGetNamespaceIn>,
) -> Result<MsgPackOrJson<KvGetNamespaceOut>> {
    let namespace: KvNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    Ok(MsgPackOrJson(KvGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route_with("/kv/set", post_with(kv_set, kv_set_operation), &tag)
        .api_route_with("/kv/get", post_with(kv_get, kv_get_operation), &tag)
        .api_route_with(
            "/kv/namespace/create",
            post_with(kv_create_namespace, kv_create_namespace_operation),
            &tag,
        )
        .api_route_with(
            "/kv/namespace/get",
            post_with(kv_get_namespace, kv_get_namespace_operation),
            &tag,
        )
        .api_route_with("/kv/delete", post_with(kv_del, kv_del_operation), &tag)
}
