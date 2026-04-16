use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_id::NamespaceId;
use fjall_utils::{FjallKeyAble, TableRow};
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

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::TokenBucket)]
pub(crate) struct TokenBucketKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) identifier: String,
}
