// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_derive::aide_annotate;
use coyote_error::{Error, HttpError, ResultExt};
use coyote_idempotency::{
    IdempotencyStartResult,
    operations::{AbortOperation, CompleteOperation, TryStartOperation},
};
use coyote_namespace::{Namespace, entities::IdempotencyConfig};
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

// Re-export types that are used in AppState
pub use coyote_idempotency::IdempotencyStore;

pub type IdempotencyNamespace = Namespace<IdempotencyConfig>;

// ============================================================================
// API Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyStartIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// TTL in seconds for the lock/response
    #[validate(range(min = 1))]
    pub ttl: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartOut {
    /// Lock acquired, request should proceed
    Started,
    /// Request is already in progress (locked)
    Locked,
    /// Request was already completed, cached response returned
    Completed { response: Vec<u8> },
}

impl From<IdempotencyStartResult> for IdempotencyStartOut {
    fn from(result: IdempotencyStartResult) -> Self {
        match result {
            IdempotencyStartResult::Started => IdempotencyStartOut::Started,
            IdempotencyStartResult::Locked => IdempotencyStartOut::Locked,
            IdempotencyStartResult::Completed { response } => {
                IdempotencyStartOut::Completed { response }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyStartIn>,
) -> Result<MsgPackOrJson<IdempotencyStartOut>> {
    let key_str = data.key.to_string();
    let operation = TryStartOperation::new(key_str, data.ttl);
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(response.result.into()))
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCompleteIn>,
) -> Result<MsgPackOrJson<IdempotencyCompleteOut>> {
    let key_str = data.key.to_string();
    let operation = CompleteOperation::new(key_str, data.response, data.ttl);
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abort")]
async fn idempotency_abort(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyAbortIn>,
) -> Result<MsgPackOrJson<IdempotencyAbortOut>> {
    let key_str = data.key.to_string();
    let operation = AbortOperation::new(key_str);
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(IdempotencyAbortOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct IdempotencyGetNamespaceOut {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Get idempotency namespace
#[aide_annotate(op_id = "v1.idempotency.get_namespace")]
async fn idempotency_get_namespace(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyGetNamespaceIn>,
) -> Result<MsgPackOrJson<IdempotencyGetNamespaceOut>> {
    let namespace: IdempotencyNamespace = state
        .namespace_state
        .fetch_namespace(Some(&data.name))?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    Ok(MsgPackOrJson(IdempotencyGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        created_at: namespace.created_at,
        updated_at: namespace.updated_at,
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
            "/idempotency/get-namespace",
            post_with(
                idempotency_get_namespace,
                idempotency_get_namespace_operation,
            ),
            &tag,
        )
}
