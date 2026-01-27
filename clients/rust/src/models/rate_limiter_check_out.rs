// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limit_result::RateLimitResult;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterCheckOut {
    /// Number of tokens remaining
    pub remaining: u64,

    /// Whether the request is allowed
    pub result: RateLimitResult,

    /// Seconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl RateLimiterCheckOut {
    pub fn new(remaining: u64, result: RateLimitResult) -> Self {
        Self {
            remaining,
            result,
            retry_after: None,
        }
    }
}
