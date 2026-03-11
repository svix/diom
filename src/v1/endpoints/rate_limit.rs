// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_core::types::EntityKey;
use diom_derive::aide_annotate;
use diom_error::ResultExt;
use diom_namespace::Namespace;
use diom_proto::MsgPackOrJson;
use diom_rate_limit::RateLimiter;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
use diom_rate_limit::{FixedWindow, RateLimitConfig, RateLimitStatus, TokenBucket};

pub type RateLimitNamespace = Namespace<RateLimitConfig>;

// FIXME(@svix-lucho): Not fully convinced about 'method' and 'config'
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "method", content = "config", rename_all = "snake_case")]
pub enum RateLimiterMethod {
    TokenBucket(RateLimiterTokenBucketConfig),
    FixedWindow(RateLimiterFixedWindowConfig),
}

// FIXME(@svix-lucho): Is this right?
impl Validate for RateLimiterMethod {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            RateLimiterMethod::TokenBucket(config) => config.validate(),
            RateLimiterMethod::FixedWindow(config) => config.validate(),
        }
    }
}

impl From<RateLimiterMethod> for RateLimitConfig {
    fn from(val: RateLimiterMethod) -> Self {
        match val {
            RateLimiterMethod::TokenBucket(config) => RateLimitConfig::TokenBucket(config.into()),
            RateLimiterMethod::FixedWindow(config) => RateLimitConfig::FixedWindow(config.into()),
        }
    }
}

impl From<RateLimiterTokenBucketConfig> for TokenBucket {
    fn from(val: RateLimiterTokenBucketConfig) -> Self {
        TokenBucket {
            bucket_size: val.capacity,
            refill_rate: val.refill_amount,
            refill_interval: Duration::from_secs(val.refill_interval),
        }
    }
}

impl From<RateLimiterFixedWindowConfig> for FixedWindow {
    fn from(val: RateLimiterFixedWindowConfig) -> Self {
        FixedWindow {
            size: Duration::from_secs(val.window_size),
            tokens: val.max_requests,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterFixedWindowConfig {
    /// Window size in seconds
    #[validate(range(min = 1))]
    pub window_size: u64,

    /// Maximum number of requests allowed within the window
    #[validate(range(min = 1))]
    pub max_requests: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterTokenBucketConfig {
    /// Maximum capacity of the bucket
    #[validate(range(min = 1))]
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    #[validate(range(min = 1))]
    pub refill_amount: u64,

    /// Interval in seconds between refills (minimum 1 second)
    #[validate(range(min = 1))]
    #[serde(default = "default_interval")]
    pub refill_interval: u64,
}

fn default_interval() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterCheckIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// Number of tokens to consume (default: 1)
    #[serde(default = "default_tokens")]
    pub tokens: u64,

    /// Rate limiter configuration
    #[validate(nested)]
    #[serde(flatten)]
    pub method: RateLimiterMethod,
}

fn default_tokens() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterCheckOut {
    /// Whether the request is allowed
    pub status: RateLimitStatus,

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
    #[serde(flatten)]
    pub method: RateLimiterMethod,
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterCheckIn>,
) -> Result<MsgPackOrJson<RateLimiterCheckOut>> {
    let key = data.key.0.clone();
    let units = data.tokens;
    let method = data.method.into();

    let operation = RateLimiter::limit_operation(key, units, method);
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(RateLimiterCheckOut {
        status: response.status,
        remaining: response.remaining,
        retry_after: response.retry_after.map(|t: Duration| t.as_millis() as u64),
    }))
}

/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limiter.get_remaining")]
async fn rate_limiter_get_remaining(
    State(AppState { rate_limiter, .. }): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterGetRemainingIn>,
) -> Result<MsgPackOrJson<RateLimiterGetRemainingOut>> {
    let now = Timestamp::now();
    let (remaining, retry_after) =
        rate_limiter.get_remaining(now, &data.key, data.method.into())?;

    Ok(MsgPackOrJson(RateLimiterGetRemainingOut {
        remaining,
        retry_after: retry_after.map(|t| t.as_millis() as u64),
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
