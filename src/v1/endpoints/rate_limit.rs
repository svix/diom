// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_core::types::EntityKey;
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_namespace::entities::StorageType;
use diom_proto::MsgPackOrJson;
use diom_rate_limit::{State as RateLimiter, operations::CreateRateLimitOperation};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
pub use diom_rate_limit::{RateLimitStatus, TokenBucket};

pub use diom_rate_limit::RateLimitNamespace;

impl From<RateLimiterTokenBucketConfig> for TokenBucket {
    fn from(val: RateLimiterTokenBucketConfig) -> Self {
        TokenBucket {
            bucket_size: val.capacity,
            refill_rate: val.refill_amount,
            refill_interval: std::time::Duration::from_secs(val.refill_interval),
        }
    }
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
    pub config: RateLimiterTokenBucketConfig,
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
    pub config: RateLimiterTokenBucketConfig,
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
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterCheckIn>,
) -> Result<MsgPackOrJson<RateLimiterCheckOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.key.namespace())?
        .ok_or_not_found()?;

    let key = data.key.0.clone();
    let units = data.tokens;
    let method = data.config.into();

    let operation = RateLimiter::limit_operation(namespace, key, units, method);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(RateLimiterCheckOut {
        status: response.status,
        remaining: response.remaining,
        retry_after: response
            .retry_after
            .map(|t: std::time::Duration| t.as_millis() as u64),
    }))
}

/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limiter.get_remaining")]
async fn rate_limiter_get_remaining(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterGetRemainingIn>,
) -> Result<MsgPackOrJson<RateLimiterGetRemainingOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.key.namespace())?
        .ok_or_not_found()?;

    let now = Timestamp::now();
    let (remaining, retry_after) = state.rate_limiter.get_remaining(
        now,
        namespace.id,
        namespace.storage_type,
        &data.key,
        data.config.into(),
    )?;

    Ok(MsgPackOrJson(RateLimiterGetRemainingOut {
        remaining,
        retry_after: retry_after.map(|t| t.as_millis() as u64),
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimiterCreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimiterCreateNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimiterGetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimiterGetNamespaceOut {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create rate limiter namespace
#[aide_annotate(op_id = "v1.rate_limiter.namespace.create")]
async fn rate_limiter_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterCreateNamespaceIn>,
) -> Result<MsgPackOrJson<RateLimiterCreateNamespaceOut>> {
    let operation =
        CreateRateLimitOperation::new(data.name, data.storage_type, data.max_storage_bytes);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(RateLimiterCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get rate limiter namespace
#[aide_annotate(op_id = "v1.rate_limiter.namespace.get")]
async fn rate_limiter_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimiterGetNamespaceIn>,
) -> Result<MsgPackOrJson<RateLimiterGetNamespaceOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(RateLimiterGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Rate Limiter");

    ApiRouter::new()
        .api_route_with(
            "/rate-limit/limit",
            post_with(rate_limiter_limit, rate_limiter_limit_operation),
            &tag,
        )
        .api_route_with(
            "/rate-limit/get-remaining",
            post_with(
                rate_limiter_get_remaining,
                rate_limiter_get_remaining_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/rate-limit/namespace/create",
            post_with(
                rate_limiter_create_namespace,
                rate_limiter_create_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/rate-limit/namespace/get",
            post_with(
                rate_limiter_get_namespace,
                rate_limiter_get_namespace_operation,
            ),
            &tag,
        )
}
