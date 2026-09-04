use diom_core::{PersistableVersioned, types::UnixTimestampMs};
use diom_id::NamespaceId;
use fjall_utils::FjallKey;

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    TokenBucket = 0,
}

#[derive(Debug, Eq, PartialEq, PersistableVersioned)]
#[versioned(row_type = RowType::TokenBucket)]
pub struct TokenBucketRow {
    pub tokens: u64,
    pub last_refill: UnixTimestampMs,
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::TokenBucket)]
pub(crate) struct TokenBucketKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) identifier: String,
}

#[cfg(test)]
mod byte_fixtures {
    use super::*;
    use fjall_utils::fixtures::decode_row;
    #[test]
    fn token_bucket_row_v0() {
        // bytes come from a serialized TokenBucketRow.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let row: TokenBucketRow = decode_row("002a80d095ffbc31");
        assert_eq!(row.tokens, 42);
        assert_eq!(
            row.last_refill,
            UnixTimestampMs::try_from_millisecond(1_700_000_000_000).unwrap()
        );
    }
}
