// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_authorization::RequestedOperation;
use diom_core::types::{DurationMs, EntityKey};
use diom_derive::aide_annotate;
use diom_error::{OptionExt as _, ResultExt};
use diom_id::Module;
use diom_idempotency::{
    IdempotencyStartResult,
    operations::{
        AbortOperation, CompleteOperation, CreateIdempotencyOperation, TryStartOperation,
    },
};
use diom_namespace::{
    Namespace,
    entities::{IdempotencyConfig, NamespaceName},
};
use diom_proto::{MsgPackOrJson, RequestInput};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

fn idempotency_metadata<'a>(
    ns: Option<&'a str>,
    key: &'a EntityKey,
    action: &'static str,
) -> RequestedOperation<'a> {
    RequestedOperation {
        module: Module::Idempotency,
        namespace: ns,
        key: Some(key.as_str()),
        action,
    }
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn operation(&self) -> RequestedOperation<'_> {
                idempotency_metadata(self.namespace.as_deref(), &self.key, $action)
            }
        }
    };
}

pub type IdempotencyNamespace = Namespace<IdempotencyConfig>;

// ============================================================================
// API Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct IdempotencyStartIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    /// TTL in milliseconds for the lock/response
    #[serde(rename = "ttl_ms")]
    #[validate(range(min = 1))]
    pub ttl: DurationMs,
}

request_input!(IdempotencyStartIn, "start");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", content = "data", rename_all = "snake_case")]
pub enum IdempotencyStartOut {
    /// Lock acquired, request should proceed
    Started,
    /// Request is already in progress (locked)
    Locked,
    /// Request was already completed, cached response returned
    Completed(IdempotencyCompleted),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyCompleted {
    pub response: Vec<u8>,
}

impl From<IdempotencyStartResult> for IdempotencyStartOut {
    fn from(result: IdempotencyStartResult) -> Self {
        match result {
            IdempotencyStartResult::Started => IdempotencyStartOut::Started,
            IdempotencyStartResult::Locked => IdempotencyStartOut::Locked,
            IdempotencyStartResult::Completed { response } => {
                IdempotencyStartOut::Completed(IdempotencyCompleted { response })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct IdempotencyCompleteIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in milliseconds for the cached response
    #[serde(rename = "ttl_ms")]
    #[validate(range(min = 1))]
    pub ttl: DurationMs,
}

request_input!(IdempotencyCompleteIn, "complete");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyCompleteOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct IdempotencyAbortIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,
}

request_input!(IdempotencyAbortIn, "abort");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyAbortOut {}

// ============================================================================
// API Endpoints
// ============================================================================

/// Start an idempotent request
#[aide_annotate(op_id = "v1.idempotency.start")]
async fn idempotency_start(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyStartIn>,
) -> Result<MsgPackOrJson<IdempotencyStartOut>> {
    let namespace: IdempotencyNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = TryStartOperation::new(namespace, data.key.to_string(), data.ttl);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(response.result.into()))
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCompleteIn>,
) -> Result<MsgPackOrJson<IdempotencyCompleteOut>> {
    let namespace: IdempotencyNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation =
        CompleteOperation::new(namespace, data.key.to_string(), data.response, data.ttl);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abort")]
async fn idempotency_abort(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyAbortIn>,
) -> Result<MsgPackOrJson<IdempotencyAbortOut>> {
    let namespace: IdempotencyNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = AbortOperation::new(namespace, data.key.to_string());
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(IdempotencyAbortOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(IdempotencyGetNamespaceIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceOut {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct IdempotencyCreateNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(IdempotencyCreateNamespaceIn, "create");

impl From<IdempotencyCreateNamespaceIn> for CreateIdempotencyOperation {
    fn from(v: IdempotencyCreateNamespaceIn) -> Self {
        CreateIdempotencyOperation::new(v.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyCreateNamespaceOut {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create idempotency namespace
#[aide_annotate(op_id = "v1.idempotency.namespace.create")]
async fn idempotency_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCreateNamespaceIn>,
) -> Result<MsgPackOrJson<IdempotencyCreateNamespaceOut>> {
    let operation = CreateIdempotencyOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(IdempotencyCreateNamespaceOut {
        name: resp.name,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get idempotency namespace
#[aide_annotate(op_id = "v1.idempotency.namespace.get")]
async fn idempotency_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyGetNamespaceIn>,
) -> Result<MsgPackOrJson<IdempotencyGetNamespaceOut>> {
    // Ensure we have the latest version of namespace
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: IdempotencyNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(IdempotencyGetNamespaceOut {
        name: namespace.name,
        created: namespace.created,
        updated: namespace.updated,
    }))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Idempotency");

    ApiRouter::new()
        .api_route_with(
            idempotency_start_path,
            post_with(idempotency_start, idempotency_start_operation),
            &tag,
        )
        .api_route_with(
            idempotency_complete_path,
            post_with(idempotency_complete, idempotency_complete_operation),
            &tag,
        )
        .api_route_with(
            idempotency_abort_path,
            post_with(idempotency_abort, idempotency_abort_operation),
            &tag,
        )
        .api_route_with(
            idempotency_create_namespace_path,
            post_with(
                idempotency_create_namespace,
                idempotency_create_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            idempotency_get_namespace_path,
            post_with(
                idempotency_get_namespace,
                idempotency_get_namespace_operation,
            ),
            &tag,
        )
}
