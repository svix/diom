use std::time::Duration;

use super::{LimitResponse, RateLimiterRequest};
use crate::{RateLimitConfig, RateLimitStatus};
use coyote_operations::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOperation {
    pub(crate) key: String,
    pub(crate) tokens: u64,
    pub(crate) method: RateLimitConfig,
}

impl LimitOperation {
    pub fn new(key: String, tokens: u64, method: RateLimitConfig) -> Self {
        Self {
            key,
            tokens,
            method,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitResponseData {
    pub status: RateLimitStatus,
    pub remaining: u64,
    pub retry_after: Option<Duration>,
}

impl LimitOperation {
    fn apply_real(self, state: &crate::RateLimiter, now: Timestamp) -> Result<LimitResponseData> {
        let (status, remaining, retry_after) =
            state.limit(now, &self.key, self.tokens, self.method)?;
        Ok(LimitResponseData {
            status,
            remaining,
            retry_after,
        })
    }
}

impl RateLimiterRequest for LimitOperation {
    fn apply(self, state: &crate::RateLimiter, now: Timestamp) -> LimitResponse {
        LimitResponse(self.apply_real(state, now))
    }
}
