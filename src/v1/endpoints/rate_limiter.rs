// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::State;
use coyote_derive::aide_annotate;
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::types::EntityKey, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
use coyote_rate_limiter::{FixedWindow, RateLimitConfig, RateLimitResult, TokenBucket};

// FIXME(@svix-lucho): Not fully convinced about 'method' and 'config'
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method", content = "config", rename_all = "snake_case")]
pub enum RateLimiterMethod {
    TokenBucket(RateLimiterTokenBucketConfig),
    FixedWindow(RateLimiterFixedWindowConfig),
}

// FIXME(@svix-lucho): This can be done with postprocessing of the OpenAPI spec or with some magic macro that I couldn't write
impl JsonSchema for RateLimiterMethod {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RateLimiterMethod".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        #[derive(JsonSchema)]
        #[serde(tag = "method", content = "config", rename_all = "snake_case")]
        #[allow(unused)]
        enum RateLimiterMethod {
            TokenBucket(RateLimiterTokenBucketConfig),
            FixedWindow(RateLimiterFixedWindowConfig),
        }

        let mut schema = RateLimiterMethod::json_schema(schema_gen);
        if let Some(obj) = schema.as_object_mut() {
            obj.insert("type".to_string(), serde_json::json!("object"));
            obj.insert(
                "discriminator".to_string(),
                serde_json::json!({"propertyName": "method"}),
            );
        }
        schema
    }
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

impl Into<RateLimitConfig> for RateLimiterMethod {
    fn into(self) -> RateLimitConfig {
        match self {
            RateLimiterMethod::TokenBucket(config) => RateLimitConfig::TokenBucket(config.into()),
            RateLimiterMethod::FixedWindow(config) => RateLimitConfig::FixedWindow(config.into()),
        }
    }
}

impl Into<TokenBucket> for RateLimiterTokenBucketConfig {
    fn into(self) -> TokenBucket {
        TokenBucket {
            bucket_size: self.capacity,
            refill_rate: self.refill_amount,
            refill_interval: Duration::from_secs(self.refill_interval_seconds),
        }
    }
}

impl Into<FixedWindow> for RateLimiterFixedWindowConfig {
    fn into(self) -> FixedWindow {
        FixedWindow {
            size: Duration::from_secs(self.window_size_seconds),
            tokens: self.max_requests,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterFixedWindowConfig {
    /// Window size in seconds
    #[validate(range(min = 1))]
    pub window_size_seconds: u64,

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
    #[serde(flatten)]
    pub method: RateLimiterMethod,
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
    State(AppState { rate_limiter, .. }): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterCheckIn>,
) -> Result<MsgPackOrJson<RateLimiterCheckOut>> {
    let now = Timestamp::now(); // FIXME(@svix-lucho): should come from consensus?
    let (result, remaining, retry_after) =
        rate_limiter.limit(now, &data.key, data.units, data.method.into())?;

    Ok(MsgPackOrJson(RateLimiterCheckOut {
        result,
        remaining,
        retry_after: retry_after.map(|t| t.as_millis() as u64),
    }))
}

// FIXME: should this essentially just be a "dry-run" option on limit?
/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limiter.get_remaining")]
async fn rate_limiter_get_remaining(
    State(AppState { rate_limiter, .. }): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterGetRemainingIn>,
) -> Result<MsgPackOrJson<RateLimiterGetRemainingOut>> {
    let now = Timestamp::now(); // FIXME(@svix-lucho): should come from consensus?
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
