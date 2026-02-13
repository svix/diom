// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::State;
use coyote_derive::aide_annotate;
use coyote_idempotency::IdempotencyStartResult;
use coyote_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::types::EntityKey, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
pub use coyote_idempotency::IdempotencyStore;

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
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyStartIn>,
) -> Result<MsgPackOrJson<IdempotencyStartOut>> {
    let key_str = data.key.to_string();

    let mut idempotency_store = state.get_idempotency_store_by_key(&key_str)?;
    let result = idempotency_store.try_start(&key_str, data.ttl)?;
    Ok(MsgPackOrJson(result.into()))
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCompleteIn>,
) -> Result<MsgPackOrJson<IdempotencyCompleteOut>> {
    let key_str = data.key.to_string();

    let mut idempotency_store = state.get_idempotency_store_by_key(&key_str)?;
    idempotency_store.complete(&key_str, data.response, data.ttl)?;

    Ok(MsgPackOrJson(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abort")]
async fn idempotency_abort(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyAbortIn>,
) -> Result<MsgPackOrJson<IdempotencyAbortOut>> {
    let key_str = data.key.to_string();

    let mut idempotency_store = state.get_idempotency_store_by_key(&key_str)?;
    idempotency_store.abort(&key_str)?;

    Ok(MsgPackOrJson(IdempotencyAbortOut {}))
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
}
