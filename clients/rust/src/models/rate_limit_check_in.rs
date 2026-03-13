// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCheckIn {
    pub key: String,

    /// Number of tokens to consume (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<u64>,

    /// Maximum capacity of the bucket
    pub capacity: u64,

    /// Number of tokens to add per refill interval
    pub refill_amount: u64,

    /// Interval in milliseconds between refills (minimum 1 millisecond)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refill_interval_millis: Option<u64>,
}

impl RateLimitCheckIn {
    pub fn new(key: String, capacity: u64, refill_amount: u64) -> Self {
        Self {
            key,
            tokens: None,
            capacity,
            refill_amount,
            refill_interval_millis: None,
        }
    }

    pub fn with_tokens(mut self, value: impl Into<Option<u64>>) -> Self {
        self.tokens = value.into();
        self
    }

    pub fn with_refill_interval_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.refill_interval_millis = value.into();
        self
    }
}
