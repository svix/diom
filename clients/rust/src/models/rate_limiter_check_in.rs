// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limiter_config::RateLimiterConfig;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterCheckIn {
    /// Rate limiter configuration
    pub config: RateLimiterConfig,

    pub key: String,

    /// Number of units to consume (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<u64>,
}

impl RateLimiterCheckIn {
    pub fn new(config: RateLimiterConfig, key: String) -> Self {
        Self {
            config,
            key,
            units: None,
        }
    }
}
