// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{num::NonZeroU64, sync::Arc, time::Duration};

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_derive::aide_annotate;
use diom_error::{Error, HttpError, ResultExt};
use diom_kv::{KvStore, operations::CreateKvOperation};
use diom_namespace::{
    Namespace,
    entities::{KeyValueConfig, StorageType},
};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::{cluster::RaftState, types::EntityKey},
    error::Result,
    v1::{
        modules::kv::{KvModel, OperationBehavior},
        utils::openapi_tag,
    },
};

// Re-export types that are used in AppState
pub use crate::v1::modules::kv::{KvStore as KvStoreType, worker};

pub type KvNamespace = Namespace<KeyValueConfig>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvSetIn {
    #[validate(nested)]
    pub key: Arc<EntityKey>,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[validate(range(min = 1))]
    pub ttl: Option<u64>,

    #[serde(default)]
    pub behavior: OperationBehavior,
}

impl KvSetIn {
    fn into_model(self) -> KvModel {
        let KvSetIn {
            key: _,
            ttl: expire_in,
            value,
            behavior: _,
        } = self;

        let expiry = expire_in.map(|expire_in| Timestamp::now() + Duration::from_millis(expire_in));

        KvModel { expiry, value }
    }
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
    pub key: Arc<EntityKey>,

    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl KvGetOut {
    fn from_model(key: Arc<EntityKey>, model: KvModel) -> Self {
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
    pub key: Arc<EntityKey>,
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
    let key = data.key.0.clone();

    // TODO: Presumably this should only need to happen in
    // the consensus layer, but currently raft seems to
    // break if an operation with a non-existent namespace is attempted,
    // so do this here for now as a quick check that the namespace
    // exists:
    let _kv_store = state.get_kv_store_by_key(&key).await?;

    let behavior = data.behavior.clone();
    let model = data.into_model();

    let operation = KvStore::set_operation(key, model, behavior);
    repl.client_write(operation).await.map_err_generic()?.0?;

    let ret = KvSetOut {};
    Ok(MsgPackOrJson(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<KvGetIn>,
) -> Result<MsgPackOrJson<KvGetOut>> {
    let mut kv_store = state.get_kv_store_by_key(&data.key.0).await?;

    let model = kv_store.get(&data.key.0).map_err(|e| Error::generic(e))?;
    let ret = match model {
        Some(m) => KvGetOut::from_model(Arc::new(data.key), m),
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvDeleteIn>,
) -> Result<MsgPackOrJson<KvDeleteOut>> {
    let key = data.key.0.clone();
    let operation = KvStore::delete_operation(key);
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
