// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::Extension;
use coyote_derive::aide_annotate;
use coyote_error::ResultExt;
use coyote_proto::MsgPackOrJson;
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyStartIn>,
) -> Result<MsgPackOrJson<IdempotencyStartOut>> {
    let key = data.key.to_string();
    let operation =
        coyote_idempotency::IdempotencyStore::start_operation(key.clone(), data.ttl_seconds);
    let response = repl.client_write(operation).await.map_err_generic()?.0?;
    match response {
        None => Ok(MsgPackOrJson(IdempotencyStartOut::Locked)),
        Some(response) => Ok(MsgPackOrJson(IdempotencyStartOut::Completed { response })),
    }
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyCompleteIn>,
) -> Result<MsgPackOrJson<IdempotencyCompleteOut>> {
    let key = data.key.to_string();
    let operation = coyote_idempotency::IdempotencyStore::complete_operation(
        key,
        data.response,
        data.ttl_seconds,
    );
    repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abandon")]
async fn idempotency_abandon(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<IdempotencyAbandonIn>,
) -> Result<MsgPackOrJson<IdempotencyAbandonOut>> {
    let key = data.key.to_string();
    let operation = coyote_idempotency::IdempotencyStore::abandon_operation(key);
    repl.client_write(operation).await.map_err_generic()?.0?;
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
