use chrono::{DateTime, Utc};
use fjall_utils::TableRow;
use serde::{Deserialize, Serialize};

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    FixedWindowState::TABLE_PREFIX,
    TokenBucketState::TABLE_PREFIX,
]));

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixedWindowState {
    // FIXME(@svix-lucho) - This makes us store the key in the table twice. We should avoid it.
    pub key: String,
    pub count: u64,
    pub window_start: DateTime<Utc>,
}

impl TableRow for FixedWindowState {
    const TABLE_PREFIX: &'static str = "_FIXED_WINDOW_";
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.key
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenBucketState {
    // FIXME(@svix-lucho) - This makes us store the key in the table twice. We should avoid it.
    pub key: String,
    pub tokens: u64,
    pub last_refill: DateTime<Utc>,
}

impl TableRow for TokenBucketState {
    const TABLE_PREFIX: &'static str = "_TOKEN_BUCKET_";
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.key
    }
}
