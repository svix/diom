use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_authorization::RequestedOperation;
use diom_core::types::{DurationMs, EntityKey, UnixTimestampMs};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::Module;
use diom_namespace::entities::NamespaceName;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use diom_rate_limit::operations::{ConfigureRateLimitOperation, LimitOperation, ResetOperation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, error::Result, v1::utils::openapi_tag};

// Re-export types that are used in AppState
pub use diom_rate_limit::TokenBucket;

pub use diom_rate_limit::RateLimitNamespace;

fn rate_limit_metadata<'a>(
    ns: Option<&'a NamespaceName>,
    key: &'a EntityKey,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::RateLimit,
        namespace: ns.map(|n| n.as_str()),
        key: Some(key.as_str()),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                rate_limit_metadata(self.namespace.as_ref(), &self.key, $action)
            }
        }
    };
}

impl From<RateLimitConfig> for TokenBucket {
    fn from(val: RateLimitConfig) -> Self {
        TokenBucket {
            bucket_size: val.capacity,
            refill_rate: val.refill_amount,
            refill_interval: val.refill_interval,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitConfig {
    /// Maximum capacity of the bucket
    #[validate(range(min = 1))]
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    #[validate(range(min = 1))]
    pub refill_amount: u64,

    /// Interval in milliseconds between refills (minimum 1 millisecond)
    #[serde(rename = "refill_interval_ms", default = "default_interval_ms")]
    #[validate(range(min = 1))]
    pub refill_interval: DurationMs,
}

fn default_interval_ms() -> DurationMs {
    1000.into()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitCheckIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    /// Number of tokens to consume (default: 1)
    #[serde(default = "default_tokens")]
    pub tokens: u64,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitConfig,
}

request_input!(RateLimitCheckIn, "limit");

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
    #[serde(rename = "retry_after_ms", skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<DurationMs>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitGetRemainingIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitConfig,
}

request_input!(RateLimitGetRemainingIn, "get-remaining");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RateLimitGetRemainingOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Milliseconds until at least one token is available (only present when remaining is 0)
    #[serde(rename = "retry_after_ms", skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<DurationMs>,
}

/// Rate Limiter Check and Consume
#[aide_annotate(op_id = "v1.rate-limit.limit")]
async fn rate_limit_limit(
    Extension(repl): Extension<RaftState>,
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitCheckIn>,
) -> Result<MsgPackOrJson<RateLimitCheckOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let key = data.key.0;
    let tokens = data.tokens;
    let config: TokenBucket = data.config.into();

    // Fast path: if local state already shows exhaustion, no need to go to raft.
    let now = repl.time.now();
    let rate_limit_state = repl.state_machine.rate_limit_store().await;
    let (remaining, retry_after) = rate_limit_state
        .controller()
        .get_remaining(now, namespace.id, key.clone(), tokens, config.clone())
        .await?;
    if let Some(retry_after) = retry_after {
        return Ok(MsgPackOrJson(RateLimitCheckOut {
            allowed: false,
            remaining,
            retry_after: Some(retry_after),
        }));
    }

    let operation = LimitOperation::new(namespace, key, tokens, config);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(RateLimitCheckOut {
        allowed: response.allowed,
        remaining: response.remaining,
        retry_after: response.retry_after,
    }))
}

/// Rate Limiter Get Remaining
#[aide_annotate(op_id = "v1.rate-limit.get-remaining")]
async fn rate_limit_get_remaining(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitGetRemainingIn>,
) -> Result<MsgPackOrJson<RateLimitGetRemainingOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    repl.wait_linearizable().await.or_internal_error()?;

    let now = repl.time.now();
    let rate_limit_state = repl.state_machine.rate_limit_store().await;
    let controller = rate_limit_state.controller();
    let (remaining, retry_after) = controller
        .get_remaining(now, namespace.id, data.key, 1, data.config.into())
        .await?;

    Ok(MsgPackOrJson(RateLimitGetRemainingOut {
        remaining,
        retry_after,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct RateLimitResetIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,

    pub key: EntityKey,

    /// Rate limiter configuration
    #[validate(nested)]
    pub config: RateLimitConfig,
}

request_input!(RateLimitResetIn, "reset");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
pub struct RateLimitResetOut {}

/// Rate Limiter Reset
#[aide_annotate(op_id = "v1.rate-limit.reset")]
async fn rate_limit_reset(
    Extension(repl): Extension<RaftState>,
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitResetIn>,
) -> Result<MsgPackOrJson<RateLimitResetOut>> {
    let namespace: RateLimitNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let key = data.key.0.clone();
    let method = data.config.into();

    let operation = ResetOperation::new(namespace, key, method);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(RateLimitResetOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct RateLimitConfigureNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(RateLimitConfigureNamespaceIn, "configure");

impl From<RateLimitConfigureNamespaceIn> for ConfigureRateLimitOperation {
    fn from(v: RateLimitConfigureNamespaceIn) -> Self {
        ConfigureRateLimitOperation::new(v.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct RateLimitConfigureNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RateLimitGetNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(RateLimitGetNamespaceIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct RateLimitGetNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Configure rate limiter namespace
#[aide_annotate(op_id = "v1.rate-limit.namespace.configure")]
async fn rate_limit_configure_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RateLimitConfigureNamespaceIn>,
) -> Result<MsgPackOrJson<RateLimitConfigureNamespaceOut>> {
    let operation = ConfigureRateLimitOperation::from(data);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(RateLimitConfigureNamespaceOut {
        name: resp.name,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get rate limiter namespace
#[aide_annotate(op_id = "v1.rate-limit.namespace.get")]
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
        created: namespace.created,
        updated: namespace.updated,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Rate Limiter");

    ApiRouter::new()
        .api_route_with(
            rate_limit_limit_path,
            post_with(rate_limit_limit, rate_limit_limit_operation),
            &tag,
        )
        .api_route_with(
            rate_limit_get_remaining_path,
            post_with(rate_limit_get_remaining, rate_limit_get_remaining_operation),
            &tag,
        )
        .api_route_with(
            rate_limit_reset_path,
            post_with(rate_limit_reset, rate_limit_reset_operation),
            &tag,
        )
        .api_route_with(
            rate_limit_configure_namespace_path,
            post_with(
                rate_limit_configure_namespace,
                rate_limit_configure_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            rate_limit_get_namespace_path,
            post_with(rate_limit_get_namespace, rate_limit_get_namespace_operation),
            &tag,
        )
}
