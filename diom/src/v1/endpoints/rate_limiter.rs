// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use diom_derive::aide_annotate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::Result,
    v1::utils::{openapi_tag, ValidatedJson},
    AppState,
};

// Re-export types that are used in AppState
pub use crate::v1::modules::rate_limiter::worker;
pub use crate::v1::modules::rate_limiter::RateLimiterStore;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterConfig {
    /// Maximum capacity of the bucket
    #[validate(range(min = 1))]
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    #[validate(range(min = 1))]
    pub refill_amount: u64,

    /// Interval in seconds between refills (minimum 1 second)
    #[validate(range(min = 1))]
    pub refill_interval_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterCheckIn {
    #[validate]
    pub key: EntityKey,

    /// Number of tokens to consume (default: 1)
    #[serde(default = "default_tokens_requested")]
    pub tokens_requested: u64,

    /// Rate limiter configuration
    #[validate]
    pub config: RateLimiterConfig,
}

fn default_tokens_requested() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterCheckOut {
    /// Whether the request is allowed
    pub allowed: bool,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterGetRemainingIn {
    #[validate]
    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate]
    pub config: RateLimiterConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until at least one token is available (only present when remaining is 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

/// Rate Limiter Check and Consume
#[aide_annotate(op_id = "v1.rate_limiter.limit")]
async fn rate_limiter_limit(
    State(AppState {
        rate_limiter_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterCheckIn>,
) -> Result<Json<RateLimiterCheckOut>> {
    let key_str = data.key.to_string();

    let (allowed, remaining, retry_after) = rate_limiter_store.limiter.check_and_consume(
        &key_str,
        data.tokens_requested,
        data.config.capacity,
        data.config.refill_amount,
        data.config.refill_interval_seconds,
    )?;

    Ok(Json(RateLimiterCheckOut {
        allowed,
        remaining,
        retry_after,
    }))
}

/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limiter.get_remaining")]
async fn rate_limiter_get_remaining(
    State(AppState {
        rate_limiter_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterGetRemainingIn>,
) -> Result<Json<RateLimiterGetRemainingOut>> {
    let key_str = data.key.to_string();

    let (remaining, retry_after) = rate_limiter_store.limiter.get_remaining(
        &key_str,
        data.config.capacity,
        data.config.refill_amount,
        data.config.refill_interval_seconds,
    )?;

    Ok(Json(RateLimiterGetRemainingOut {
        remaining,
        retry_after,
    }))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Rate Limiter");

    ApiRouter::new()
        .api_route_with(
            "/rate-limiter/limit",
            post_with(rate_limiter_limit, rate_limiter_limit_operation),
            &tag,
        )
        .api_route_with(
            "/rate-limiter/get-remaining",
            post_with(
                rate_limiter_get_remaining,
                rate_limiter_get_remaining_operation,
            ),
            &tag,
        )
}
