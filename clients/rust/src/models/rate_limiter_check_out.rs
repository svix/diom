// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterCheckOut {
    /// Whether the request is allowed
    pub allowed: bool,

    /// Number of tokens remaining
    pub remaining: u64,

    /// Seconds until enough tokens are available (only present when allowed is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl RateLimiterCheckOut {
    pub fn new(allowed: bool, remaining: u64) -> Self {
        Self {
            allowed,
            remaining,
            retry_after: None,
        }
    }
}
