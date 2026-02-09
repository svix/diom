use super::{RateLimiterRequest, ResetResponse};
use crate::RateLimitConfig;
use coyote_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetOperation {
    pub(crate) key: String,
    pub(crate) method: RateLimitConfig,
}

impl ResetOperation {
    pub fn new(key: String, method: RateLimitConfig) -> Self {
        Self { key, method }
    }
}

impl ResetOperation {
    fn apply_real(self, state: &crate::RateLimiter) -> Result<()> {
        state.reset(&self.key, self.method)?;
        Ok(())
    }
}

impl RateLimiterRequest for ResetOperation {
    fn apply(self, state: &crate::RateLimiter) -> ResetResponse {
        ResetResponse(self.apply_real(state))
    }
}
