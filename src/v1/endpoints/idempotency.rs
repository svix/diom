// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::State;
use coyote_derive::aide_annotate;
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
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartOut {
    /// Lock acquired, request should proceed
    Locked,
    /// Request was already completed, cached response returned
    Completed { response: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyCompleteIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// The response to cache
    pub response: Vec<u8>,

    /// TTL in seconds for the cached response
    #[validate(range(min = 1))]
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyCompleteOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyAbandonIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyAbandonOut {}

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
    match idempotency_store.try_start(&key_str, data.ttl_seconds)? {
        None => Ok(MsgPackOrJson(IdempotencyStartOut::Locked)),
        Some(response) => Ok(MsgPackOrJson(IdempotencyStartOut::Completed { response })),
    }
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCompleteIn>,
) -> Result<MsgPackOrJson<IdempotencyCompleteOut>> {
    let key_str = data.key.to_string();

    let mut idempotency_store = state.get_idempotency_store_by_key(&key_str)?;
    idempotency_store.complete(&key_str, data.response, data.ttl_seconds)?;

    Ok(MsgPackOrJson(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abandon")]
async fn idempotency_abandon(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyAbandonIn>,
) -> Result<MsgPackOrJson<IdempotencyAbandonOut>> {
    let key_str = data.key.to_string();

    let mut idempotency_store = state.get_idempotency_store_by_key(&key_str)?;
    idempotency_store.abandon(&key_str)?;

    Ok(MsgPackOrJson(IdempotencyAbandonOut {}))
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
            "/idempotency/abandon",
            post_with(idempotency_abandon, idempotency_abandon_operation),
            &tag,
        )
}
