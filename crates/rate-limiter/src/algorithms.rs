use std::time::Duration;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::tables::{FixedWindowState, TokenBucketState};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RateLimitKey(String);

impl RateLimitKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct FixedWindow {
    /// Window size
    pub size: Duration,
    /// Max tokens allowed per window
    pub tokens: u64,
}

impl FixedWindow {
    pub(crate) fn get_window_start(&self, now: Timestamp) -> Timestamp {
        let size_ms = self.size.as_millis() as i64;
        let now_ms = now.as_millisecond();
        let window_start_ms = now_ms - (now_ms % size_ms);
        Timestamp::from_millisecond(window_start_ms).unwrap()
    }

    pub(crate) fn get_key(&self, identifier: &str) -> RateLimitKey {
        RateLimitKey(format!("rate_limiter:fixed_window:{}", identifier))
    }
}

pub struct TokenBucket {
    /// Token refill rate in tokens per refill interval
    pub refill_rate: u64,
    /// Token refill interval
    pub refill_interval: Duration,
    /// Max tokens allowed in the bucket
    pub bucket_size: u64,
}

impl TokenBucket {
    pub(crate) fn get_new_capacity(
        &self,
        current: u64,
        now: Timestamp,
        last_refill: Timestamp,
    ) -> u64 {
        let mut capacity = current;
        if last_refill < now {
            let elapsed_millis: u64 = now
                .duration_since(last_refill)
                .as_millis()
                .try_into()
                .unwrap();
            let refill_interval_millis: u64 = self.refill_interval.as_millis().try_into().unwrap();

            capacity += (elapsed_millis / refill_interval_millis) * self.refill_rate;
        }
        capacity.min(self.bucket_size)
    }

    pub(crate) fn get_key(&self, identifier: &str) -> RateLimitKey {
        RateLimitKey(format!("rate_limiter:token_bucket:{}", identifier))
    }
}

pub enum RateLimitConfig {
    FixedWindow(FixedWindow),
    TokenBucket(TokenBucket),
}

impl From<Vec<u8>> for FixedWindowState {
    fn from(value: Vec<u8>) -> Self {
        rmp_serde::from_slice(&value).expect("Failed to deserialize FixedWindowState")
    }
}

impl From<FixedWindowState> for Vec<u8> {
    fn from(state: FixedWindowState) -> Self {
        rmp_serde::to_vec(&state).expect("Failed to serialize FixedWindowState")
    }
}

impl From<Vec<u8>> for TokenBucketState {
    fn from(value: Vec<u8>) -> Self {
        rmp_serde::from_slice(&value).expect("Failed to deserialize TokenBucketState")
    }
}

impl From<TokenBucketState> for Vec<u8> {
    fn from(state: TokenBucketState) -> Self {
        rmp_serde::to_vec(&state).expect("Failed to serialize TokenBucketState")
    }
}
