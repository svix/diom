// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_core::types::EntityKey;
use coyote_derive::aide_annotate;
use coyote_error::{OptionExt as _, ResultExt};
use coyote_idempotency::{
    IdempotencyStartResult,
    operations::{
        AbortOperation, CompleteOperation, CreateIdempotencyOperation, TryStartOperation,
    },
};
use coyote_namespace::{
    Namespace,
    entities::{IdempotencyConfig, StorageType},
};
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

pub type IdempotencyNamespace = Namespace<IdempotencyConfig>;

// ============================================================================
// API Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct IdempotencyStartIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// TTL in seconds for the lock/response
    #[validate(range(min = 1))]
    pub ttl: u64,
}

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
    #[validate(nested)]
    pub key: EntityKey,

    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    #[validate(range(min = 1))]
    pub ttl: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyCompleteOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct IdempotencyAbortIn {
    #[validate(nested)]
    pub key: EntityKey,
}

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
        .fetch_namespace(data.key.namespace())?
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
        .fetch_namespace(data.key.namespace())?
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
        .fetch_namespace(data.key.namespace())?
        .ok_or_not_found()?;

    let operation = AbortOperation::new(namespace, data.key.to_string());
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(IdempotencyAbortOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyCreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyCreateNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create idempotency namespace
#[aide_annotate(op_id = "v1.idempotency.namespace.create")]
async fn idempotency_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCreateNamespaceIn>,
) -> Result<MsgPackOrJson<IdempotencyCreateNamespaceOut>> {
    let operation =
        CreateIdempotencyOperation::new(data.name, data.storage_type, data.max_storage_bytes);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(IdempotencyCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
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
        max_storage_bytes: namespace.max_storage_bytes,
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Idempotency");

    ApiRouter::new()
        .api_route_with(
            "/idempotency/start",
            post_with(idempotency_start, idempotency_start_operation),
            &tag,
        )
        .api_route_with(
            "/idempotency/complete",
            post_with(idempotency_complete, idempotency_complete_operation),
            &tag,
        )
        .api_route_with(
            "/idempotency/abort",
            post_with(idempotency_abort, idempotency_abort_operation),
            &tag,
        )
        .api_route_with(
            "/idempotency/namespace/create",
            post_with(
                idempotency_create_namespace,
                idempotency_create_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/idempotency/namespace/get",
            post_with(
                idempotency_get_namespace,
                idempotency_get_namespace_operation,
            ),
            &tag,
        )
}
