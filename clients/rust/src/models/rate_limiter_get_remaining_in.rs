// this file is @generated
use serde::{Deserialize, Serialize};

use super::rate_limiter_config::RateLimiterConfig;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterGetRemainingIn {
    /// Rate limiter configuration
    pub config: RateLimiterConfig,

    pub key: String,
}

impl RateLimiterGetRemainingIn {
    pub fn new(config: RateLimiterConfig, key: String) -> Self {
        Self { config, key }
    }
}
