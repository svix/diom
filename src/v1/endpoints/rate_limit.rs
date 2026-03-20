// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_core::types::{DurationMs, EntityKey};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_namespace::entities::{NamespaceName, StorageType};
use diom_proto::MsgPackOrJson;
use diom_rate_limit::operations::{CreateRateLimitOperation, LimitOperation, ResetOperation};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
pub use diom_rate_limit::TokenBucket;

pub use diom_rate_limit::RateLimitNamespace;

impl From<RateLimitTokenBucketConfig> for TokenBucket {
    fn from(val: RateLimitTokenBucketConfig) -> Self {
        TokenBucket {
            bucket_size: val.capacity,
            refill_rate: val.refill_amount,
            refill_interval: val.refill_interval_millis,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitTokenBucketConfig {
    /// Maximum capacity of the bucket
    #[validate(range(min = 1))]
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    #[validate(range(min = 1))]
    pub refill_amount: u64,

    /// Interval in milliseconds between refills (minimum 1 millisecond)
    #[validate(range(min = 1))]
    #[serde(default = "default_interval_millis")]
    pub refill_interval_millis: DurationMs,
}

fn default_interval_millis() -> DurationMs {
    1000.into()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitCheckIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    /// Number of tokens to consume (default: 1)
    #[serde(default = "default_tokens")]
    pub tokens: u64,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitTokenBucketConfig,
}

fn default_tokens() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimitCheckOut {
    /// Whether the request is allowed
    pub allowed: bool,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_millis: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitGetRemainingIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitTokenBucketConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimitGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until at least one token is available (only present when remaining is 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after_millis: Option<u64>,
}

/// Rate Limiter Check and Consume
#[aide_annotate(op_id = "v1.rate_limit.limit")]
async fn rate_limit_limit(
    Extension(repl): Extension<RaftState>,
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitCheckIn>,
) -> Result<MsgPackOrJson<RateLimitCheckOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let key = data.key.0.clone();
    let units = data.tokens;
    let method = data.config.into();

    let operation = LimitOperation::new(namespace, key, units, method);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(RateLimitCheckOut {
        allowed: response.allowed,
        remaining: response.remaining,
        retry_after_millis: response
            .retry_after
            .map(|t: std::time::Duration| t.as_millis() as u64),
    }))
}

/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate_limit.get_remaining")]
async fn rate_limit_get_remaining(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitGetRemainingIn>,
) -> Result<MsgPackOrJson<RateLimitGetRemainingOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    repl.wait_linearizable().await.or_internal_error()?;

    let now = repl.time.last();
    // FIXME: this state should be passed, not created every time.
    let rate_limit_state = diom_rate_limit::State::init(state.do_not_use_dbs.clone())?;
    let controller = rate_limit_state.controller(namespace.storage_type);
    let (remaining, retry_after) = controller
        .get_remaining(now, namespace.id, data.key, data.config.into())
        .await?;

    Ok(MsgPackOrJson(RateLimitGetRemainingOut {
        remaining,
        retry_after_millis: retry_after.map(|t| t.as_millis() as u64),
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitResetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    #[validate(nested)]
    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitTokenBucketConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct RateLimitResetOut {}

/// Rate Limiter Reset
#[aide_annotate(op_id = "v1.rate_limit.reset")]
async fn rate_limit_reset(
    Extension(repl): Extension<RaftState>,
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitResetIn>,
) -> Result<MsgPackOrJson<RateLimitResetOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let key = data.key.0.clone();
    let method = data.config.into();

    let operation = ResetOperation::new(namespace, key, method);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(RateLimitResetOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct RateLimitCreateNamespaceIn {
    pub name: NamespaceName,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
}

impl From<RateLimitCreateNamespaceIn> for CreateRateLimitOperation {
    fn from(v: RateLimitCreateNamespaceIn) -> Self {
        CreateRateLimitOperation::new(v.name, v.storage_type, v.max_storage_bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimitCreateNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimitGetNamespaceIn {
    pub name: NamespaceName,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimitGetNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create rate limiter namespace
#[aide_annotate(op_id = "v1.rate_limit.namespace.create")]
async fn rate_limit_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitCreateNamespaceIn>,
) -> Result<MsgPackOrJson<RateLimitCreateNamespaceOut>> {
    let operation = CreateRateLimitOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(RateLimitCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get rate limiter namespace
#[aide_annotate(op_id = "v1.rate_limit.namespace.get")]
async fn rate_limit_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitGetNamespaceIn>,
) -> Result<MsgPackOrJson<RateLimitGetNamespaceOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(RateLimitGetNamespaceOut {
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
            post_with(rate_limit_limit, rate_limit_limit_operation),
            &tag,
        )
        .api_route_with(
            "/rate-limit/get-remaining",
            post_with(rate_limit_get_remaining, rate_limit_get_remaining_operation),
            &tag,
        )
        .api_route_with(
            "/rate-limit/reset",
            post_with(rate_limit_reset, rate_limit_reset_operation),
            &tag,
        )
        .api_route_with(
            "/rate-limit/namespace/create",
            post_with(
                rate_limit_create_namespace,
                rate_limit_create_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/rate-limit/namespace/get",
            post_with(rate_limit_get_namespace, rate_limit_get_namespace_operation),
            &tag,
        )
}
