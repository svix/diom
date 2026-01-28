use std::borrow::Cow;

use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::algorithms::RateLimitKey;

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    FixedWindowState::TABLE_PREFIX,
    TokenBucketState::TABLE_PREFIX,
]));

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixedWindowState {
    pub key: RateLimitKey,
    pub count: u64,
    pub window_start: Timestamp,
}

impl TableRow for FixedWindowState {
    const TABLE_PREFIX: &'static str = "_FIXED_WINDOW_";
    type Key = String;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(self.key.as_str().to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenBucketState {
    pub key: RateLimitKey,
    pub tokens: u64,
    pub last_refill: Timestamp,
}

impl TableRow for TokenBucketState {
    const TABLE_PREFIX: &'static str = "_TOKEN_BUCKET_";
    type Key = String;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(self.key.as_str().to_string())
    }
}
