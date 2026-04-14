use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_id::NamespaceId;
use fjall_utils::{TableKey, TableRow};
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    TokenBucket = 0,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, PersistableValue)]
pub struct TokenBucketRow {
    pub tokens: u64,
    pub last_refill: UnixTimestampMs,
}

impl TableRow for TokenBucketRow {
    const ROW_TYPE: u8 = RowType::TokenBucket as u8;
}

impl TokenBucketRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}
