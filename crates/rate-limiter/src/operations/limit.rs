use std::time::Duration;

use super::{LimitResponse, RateLimiterRequest};
use crate::{RateLimitConfig, RateLimitResult};
use coyote_operations::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOperation {
    pub(crate) key: String,
    pub(crate) now: Timestamp,
    pub(crate) units: u64,
    pub(crate) method: RateLimitConfig,
}

impl LimitOperation {
    pub fn new(key: String, now: Timestamp, units: u64, method: RateLimitConfig) -> Self {
        Self {
            key,
            now,
            units,
            method,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitResponseData {
    pub result: RateLimitResult,
    pub remaining: u64,
    pub retry_after: Option<Duration>,
}

impl LimitOperation {
    fn apply_real(self, state: &crate::RateLimiter) -> Result<LimitResponseData> {
        let (result, remaining, retry_after) =
            state.limit(self.now, &self.key, self.units, self.method)?;
        Ok(LimitResponseData {
            result,
            remaining,
            retry_after,
        })
    }
}

impl RateLimiterRequest for LimitOperation {
    fn apply(self, state: &crate::RateLimiter) -> LimitResponse {
        LimitResponse(self.apply_real(state))
    }
}
