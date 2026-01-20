// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Json, extract::State};
use chrono::Duration;
use diom_derive::aide_annotate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::types::EntityKey,
    error::Result,
    v1::{
        modules::rate_limiter::{RateLimitConfig, RateLimitResult, TokenBucket},
        utils::{ValidatedJson, openapi_tag},
    },
};

// Re-export types that are used in AppState
pub use crate::v1::modules::rate_limiter::RateLimiterStore;
pub use crate::v1::modules::rate_limiter::worker;

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
    #[validate(nested)]
    pub key: EntityKey,

    /// Number of units to consume (default: 1)
    #[serde(default = "default_units")]
    pub units: u64,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimiterConfig,
}

fn default_units() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterCheckOut {
    /// Whether the request is allowed
    pub result: RateLimitResult,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterGetRemainingIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate(nested)]
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
    State(AppState { rate_limiter, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterCheckIn>,
) -> Result<Json<RateLimiterCheckOut>> {
    let key_str = data.key.to_string();

    let (result, remaining, retry_after) = rate_limiter
        .limit(
            &EntityKey(key_str),
            data.units,
            RateLimitConfig::TokenBucket(TokenBucket {
                bucket_size: data.config.capacity,
                refill_rate: data.config.refill_amount,
                refill_interval: Duration::seconds(data.config.refill_interval_seconds as i64),
            }),
        )
        .map_err(|e| crate::error::Error::generic(e))?;

    Ok(Json(RateLimiterCheckOut {
        result,
        remaining,
        retry_after: retry_after.map(|d| d.num_seconds() as u64),
    }))
}

// FIXME: should this essentially just be a "dry-run" option on limit?
/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limiter.get_remaining")]
async fn rate_limiter_get_remaining(
    State(AppState { rate_limiter, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterGetRemainingIn>,
) -> Result<Json<RateLimiterGetRemainingOut>> {
    let key_str = data.key.to_string();

    let remaining = rate_limiter
        .get_remaining(
            &EntityKey(key_str),
            RateLimitConfig::TokenBucket(TokenBucket {
                bucket_size: data.config.capacity,
                refill_rate: data.config.refill_amount,
                refill_interval: Duration::seconds(data.config.refill_interval_seconds as i64),
            }),
        )
        .map_err(|e| crate::error::Error::generic(e))?;

    Ok(Json(RateLimiterGetRemainingOut {
        remaining,
        retry_after: None, // FIXME: calculate retry after
    }))
}

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
