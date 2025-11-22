// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post, ApiRouter};
use axum::{extract::State, Json};
use diom_derive::aide_annotate;
use dashmap::DashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    v1::utils::{openapi_tag, ValidatedJson},
    AppState,
};

/// Get current time in milliseconds since Unix epoch.
/// Uses system time which is consistent across distributed nodes.
/// Note: Subject to NTP adjustments and leap seconds, but rate limiters
/// are designed to tolerate small time discrepancies.
fn now_millis() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

// ============================================================================
// Token Bucket Rate Limiter
// ============================================================================

#[derive(Clone)]
pub struct TokenBucketRateLimiter {
    store: Arc<DashMap<String, TokenBucketState>>,
}

#[derive(Clone, Debug)]
struct TokenBucketState {
    tokens: u64,
    last_refill_millis: u64,
    // Configuration
    capacity: u64,
    refill_amount: u64,
    refill_interval_millis: u64,
}

impl Default for TokenBucketRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenBucketRateLimiter {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    fn configure(
        &self,
        key: &str,
        capacity: u64,
        refill_amount: u64,
        refill_interval_seconds: u64,
    ) {
        let now = now_millis();
        let refill_interval_millis = refill_interval_seconds * 1000;

        self.store.insert(
            key.to_string(),
            TokenBucketState {
                tokens: capacity,
                last_refill_millis: now,
                capacity,
                refill_amount,
                refill_interval_millis,
            },
        );
    }

    fn check_and_consume(
        &self,
        key: &str,
        tokens_requested: u64,
    ) -> Result<(bool, u64, Option<u64>)> {
        let now = now_millis();

        let mut entry = self.store.get_mut(key).ok_or_else(|| {
            Error::http(HttpError::not_found(
                Some("Rate limiter not configured for this key".into()),
                None,
            ))
        })?;

        let capacity = entry.capacity;
        let refill_amount = entry.refill_amount;
        let refill_interval_millis = entry.refill_interval_millis;

        // Refill tokens based on intervals elapsed
        let elapsed_millis = now.saturating_sub(entry.last_refill_millis);
        let intervals_elapsed = elapsed_millis / refill_interval_millis;

        if intervals_elapsed > 0 {
            let new_tokens = intervals_elapsed.saturating_mul(refill_amount);
            entry.tokens = entry.tokens.saturating_add(new_tokens).min(capacity);
            entry.last_refill_millis = now;
        }

        // Check if enough tokens available
        if entry.tokens >= tokens_requested {
            entry.tokens -= tokens_requested;
            Ok((true, entry.tokens, None))
        } else {
            // Calculate how long until we have enough tokens (in seconds)
            let tokens_needed = tokens_requested - entry.tokens;
            let intervals_needed = if refill_amount > 0 {
                tokens_needed.div_ceil(refill_amount) // Ceiling division
            } else {
                u64::MAX
            };
            let retry_after_millis = intervals_needed.saturating_mul(refill_interval_millis);
            let retry_after_seconds = retry_after_millis.div_ceil(1000); // Ceiling division to seconds
            Ok((false, entry.tokens, Some(retry_after_seconds)))
        }
    }

    fn get_remaining(&self, key: &str) -> Result<(u64, Option<u64>)> {
        let now = now_millis();

        match self.store.get(key) {
            Some(entry) => {
                let capacity = entry.capacity;
                let refill_amount = entry.refill_amount;
                let refill_interval_millis = entry.refill_interval_millis;

                let elapsed_millis = now.saturating_sub(entry.last_refill_millis);
                let intervals_elapsed = elapsed_millis / refill_interval_millis;
                let new_tokens = intervals_elapsed.saturating_mul(refill_amount);
                let current_tokens = entry.tokens.saturating_add(new_tokens).min(capacity);

                if current_tokens == 0 {
                    // Calculate retry_after for at least 1 token (in seconds)
                    let retry_after_seconds = refill_interval_millis.div_ceil(1000); // Ceiling division
                    Ok((0, Some(retry_after_seconds)))
                } else {
                    Ok((current_tokens, None))
                }
            }
            None => Err(Error::http(HttpError::not_found(
                Some("Rate limiter not configured for this key".into()),
                None,
            ))),
        }
    }
}

// ============================================================================
// Combined Rate Limiter Store
// ============================================================================

#[derive(Clone)]
pub struct RateLimiterStore {
    limiter: TokenBucketRateLimiter,
}

impl Default for RateLimiterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiterStore {
    pub fn new() -> Self {
        Self {
            limiter: TokenBucketRateLimiter::new(),
        }
    }
}

// ============================================================================
// API Types - Configuration
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterConfigureIn {
    #[validate]
    pub key: EntityKey,

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterConfigureOut {}

// ============================================================================
// API Types - Limit
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterCheckIn {
    #[validate]
    pub key: EntityKey,

    /// Number of tokens to consume (default: 1)
    #[serde(default = "default_tokens_requested")]
    pub tokens_requested: u64,
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

// ============================================================================
// API Types - Get Remaining
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimiterGetRemainingIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimiterGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until at least one token is available (only present when remaining is 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

// ============================================================================
// API Endpoints - Configuration
// ============================================================================

/// Configure Rate Limiter
#[aide_annotate(op_id = "v1.rate_limiter.configure")]
async fn rate_limiter_configure(
    State(AppState {
        rate_limiter_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterConfigureIn>,
) -> Result<Json<RateLimiterConfigureOut>> {
    let key_str = data.key.to_string();

    rate_limiter_store.limiter.configure(
        &key_str,
        data.capacity,
        data.refill_amount,
        data.refill_interval_seconds,
    );

    Ok(Json(RateLimiterConfigureOut {}))
}

// ============================================================================
// API Endpoints - Limit
// ============================================================================

/// Rate Limiter Check and Consume
#[aide_annotate(op_id = "v1.rate_limiter.limit")]
async fn rate_limiter_limit(
    State(AppState {
        rate_limiter_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<RateLimiterCheckIn>,
) -> Result<Json<RateLimiterCheckOut>> {
    let key_str = data.key.to_string();

    let (allowed, remaining, retry_after) = rate_limiter_store
        .limiter
        .check_and_consume(&key_str, data.tokens_requested)?;

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

    let (remaining, retry_after) = rate_limiter_store.limiter.get_remaining(&key_str)?;

    Ok(Json(RateLimiterGetRemainingOut {
        remaining,
        retry_after,
    }))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Rate Limiter");

    ApiRouter::new()
        .api_route("/rate-limiter/configure", post(rate_limiter_configure))
        .api_route("/rate-limiter/limit", post(rate_limiter_limit))
        .api_route(
            "/rate-limiter/get-remaining",
            post(rate_limiter_get_remaining),
        )
}
