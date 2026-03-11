use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    FixedWindow = 0,
    TokenBucket = 1,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixedWindowState {
    pub count: u64,
    pub window_start: Timestamp,
}

impl TableRow for FixedWindowState {
    const ROW_TYPE: u8 = RowType::FixedWindow as u8;
}

impl FixedWindowState {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenBucketState {
    pub tokens: u64,
    pub last_refill: Timestamp,
}

impl TableRow for TokenBucketState {
    const ROW_TYPE: u8 = RowType::TokenBucket as u8;
}

impl TokenBucketState {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}
