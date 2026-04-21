use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_authorization::RequestedOperation;
use diom_core::types::{ByteString, Consistency, EntityKey, NonZeroDurationMs, UnixTimestampMs};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::Module;
use diom_kv::{
    KvNamespace,
    kvcontroller::{KvModel, OperationBehavior},
    operations::{ConfigureKvOperation, DeleteOperation, SetOperation, SetResponseData},
};
use diom_namespace::entities::NamespaceName;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

fn kv_metadata<'a>(
    ns: Option<&'a NamespaceName>,
    key: &'a EntityKey,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::Kv,
        namespace: ns.map(|n| n.as_str()),
        key: Some(key.as_str()),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                kv_metadata(self.namespace.as_ref(), &self.key, $action)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key", "value"]))]
pub struct KvSetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    pub value: ByteString,

    /// Time to live in milliseconds
    #[serde(rename = "ttl_ms")]
    pub ttl: Option<NonZeroDurationMs>,

    #[serde(default)]
    pub behavior: OperationBehavior,

    /// If set, the write only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    pub version: Option<u64>,
}

request_input!(KvSetIn, "set");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct KvSetOut {
    pub version: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct KvGetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub key: EntityKey,
    #[serde(default = "Consistency::strong")]
    pub consistency: Consistency,
}

request_input!(KvGetIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct KvGetOut {
    /// Time of expiry
    pub expiry: Option<UnixTimestampMs>,

    pub value: Option<ByteString>,

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

    pub key: EntityKey,

    /// If set, the delete only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    pub version: Option<u64>,
}

request_input!(KvDeleteIn, "delete");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = SetOperation::new(
        namespace,
        data.key,
        data.value,
        data.ttl.map(NonZeroDurationMs::get),
        data.behavior,
        data.version,
    );
    let SetResponseData { version } = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(KvSetOut { version }))
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
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    if data.consistency.linearizable() {
        repl.wait_linearizable().await.or_internal_error()?;
    }

    let kv_state = repl.state_machine.kv_store().await;
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
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let key = data.key;
    let operation = DeleteOperation::new(namespace, key, data.version);
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct KvGetNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct KvConfigureNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(KvConfigureNamespaceIn, "configure");

impl From<KvConfigureNamespaceIn> for ConfigureKvOperation {
    fn from(v: KvConfigureNamespaceIn) -> Self {
        ConfigureKvOperation::new(v.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct KvConfigureNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Configure KV namespace
#[aide_annotate(op_id = "v1.kv.namespace.configure")]
async fn kv_configure_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvConfigureNamespaceIn>,
) -> Result<MsgPackOrJson<KvConfigureNamespaceOut>> {
    let operation = ConfigureKvOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(KvConfigureNamespaceOut {
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
            kv_configure_namespace_path,
            post_with(kv_configure_namespace, kv_configure_namespace_operation),
            &tag,
        )
        .api_route_with(
            kv_get_namespace_path,
            post_with(kv_get_namespace, kv_get_namespace_operation),
            &tag,
        )
        .api_route_with(kv_del_path, post_with(kv_del, kv_del_operation), &tag)
}
