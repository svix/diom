// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_authorization::RequestedOperation;
use coyote_core::types::{Consistency, DurationMs, EntityKey};
use coyote_derive::aide_annotate;
use coyote_error::{OptionExt, ResultExt};
use coyote_id::Module;
use coyote_kv::{
    KvNamespace,
    kvcontroller::{KvModel, OperationBehavior},
    operations::{CreateKvOperation, DeleteOperation, SetOperation, SetResponseData},
};
use coyote_namespace::entities::NamespaceName;
use coyote_proto::{MsgPackOrJson, RequestInput};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

fn kv_metadata<'a>(
    ns: Option<&'a str>,
    key: &'a EntityKey,
    action: &'static str,
) -> RequestedOperation<'a> {
    RequestedOperation {
        module: Module::Kv,
        namespace: ns,
        key: Some(key.as_str()),
        action,
    }
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn operation(&self) -> RequestedOperation<'_> {
                kv_metadata(self.namespace.as_deref(), &self.key, $action)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvSetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[serde(rename = "ttl_ms")]
    #[validate(range(min = 1))]
    pub ttl: Option<DurationMs>,

    #[serde(default)]
    pub behavior: OperationBehavior,

    /// If set, the write only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    pub version: Option<u64>,
}

request_input!(KvSetIn, "set");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {
    /// Whether the operation succeeded or was a noop due to pre-conditions.
    pub success: bool,
    pub version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvGetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,
    #[serde(default = "Consistency::strong")]
    pub consistency: Consistency,
}

request_input!(KvGetIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Option<Vec<u8>>,

    /// Opaque version token for optimistic concurrency control.
    /// Pass as `version` in a subsequent `set` to perform a conditional write.
    pub version: u64,
}

impl KvGetOut {
    fn from_model(model: KvModel) -> Self {
        Self {
            expiry: model.expiry,
            value: Some(model.value),
            version: model.version,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvDeleteIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,
}

request_input!(KvDeleteIn, "delete");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteOut {
    /// Whether the operation succeeded or was a noop due to pre-conditions.
    pub success: bool,
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
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = SetOperation::new(
        namespace,
        data.key,
        data.value,
        data.ttl,
        data.behavior,
        data.version,
    );
    let SetResponseData { version, success } =
        repl.client_write(operation).await.or_internal_error()?.0?;

    let ret = KvSetOut { version, success };
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
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    if data.consistency.linearizable() {
        repl.wait_linearizable().await.or_internal_error()?;
    }

    // FIXME: this state should be passed, not created every time.
    let kv_state = coyote_kv::State::init(state.do_not_use_dbs.clone())?;
    let controller = kv_state.controller();

    let model = controller
        .fetch(namespace.id, data.key, repl.time.now())
        .await?;

    let ret = match model {
        Some(m) => KvGetOut::from_model(m),
        None => KvGetOut {
            expiry: None,
            value: None,
            version: 0,
        },
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
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let key = data.key;
    let operation = DeleteOperation::new(namespace, key);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;

    let ret = KvDeleteOut {
        success: resp.success,
    };
    Ok(MsgPackOrJson(ret))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvGetNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(KvGetNamespaceIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvGetNamespaceOut {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct KvCreateNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(KvCreateNamespaceIn, "create");

impl From<KvCreateNamespaceIn> for CreateKvOperation {
    fn from(v: KvCreateNamespaceIn) -> Self {
        CreateKvOperation::new(v.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct KvCreateNamespaceOut {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create KV namespace
#[aide_annotate(op_id = "v1.kv.namespace.create")]
async fn kv_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvCreateNamespaceIn>,
) -> Result<MsgPackOrJson<KvCreateNamespaceOut>> {
    let operation = CreateKvOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(KvCreateNamespaceOut {
        name: resp.name,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get KV namespace
#[aide_annotate(op_id = "v1.kv.namespace.get")]
async fn kv_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvGetNamespaceIn>,
) -> Result<MsgPackOrJson<KvGetNamespaceOut>> {
    // Ensure we have the latest version of namespace
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: KvNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(KvGetNamespaceOut {
        name: namespace.name,
        created: namespace.created,
        updated: namespace.updated,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route_with(kv_set_path, post_with(kv_set, kv_set_operation), &tag)
        .api_route_with(kv_get_path, post_with(kv_get, kv_get_operation), &tag)
        .api_route_with(
            kv_create_namespace_path,
            post_with(kv_create_namespace, kv_create_namespace_operation),
            &tag,
        )
        .api_route_with(
            kv_get_namespace_path,
            post_with(kv_get_namespace, kv_get_namespace_operation),
            &tag,
        )
        .api_route_with(kv_del_path, post_with(kv_del, kv_del_operation), &tag)
}
