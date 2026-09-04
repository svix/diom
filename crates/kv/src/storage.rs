use diom_core::{
    PersistableValue, PersistableVersioned,
    types::{ByteString, UnixTimestampMs},
};
use diom_error::Result;
use diom_id::NamespaceId;
use fjall_utils::{FjallKey, TableRow};
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Pair = 0,
    Expiration = 1,
}

#[derive(PersistableVersioned)]
#[versioned(row_type = RowType::Pair)]
pub struct KvPairRow {
    pub value: ByteString,
    pub expiry: Option<UnixTimestampMs>,
    pub version: u64,
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::Pair)]
pub(crate) struct KvPairKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) key: String,
}

#[derive(Serialize, Deserialize, Debug, PersistableValue)]
pub(crate) struct ExpirationRow {}

impl ExpirationRow {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::Expiration)]
pub(crate) struct ExpirationKey {
    #[key(0)]
    pub(crate) expiration_time: UnixTimestampMs,
    #[key(1)]
    pub(crate) namespace_id: NamespaceId,
    #[key(2)]
    pub(crate) key: String,
}

impl TableRow for ExpirationRow {
    const ROW_TYPE: u8 = RowType::Expiration as u8;

    // We only store data in the keys
    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        Ok(b"".into())
    }
}

#[cfg(test)]
mod byte_fixtures {
    use super::*;
    use fjall_utils::fixtures::decode_row;
    #[test]
    fn kv_pair_row_v0() {
        // bytes come from a serialized KvPairRow.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let row: KvPairRow = decode_row("000568656c6c6f0180d095ffbc3107");
        assert_eq!(row.value, b"hello");
        assert_eq!(
            row.expiry,
            UnixTimestampMs::try_from_millisecond(1_700_000_000_000)
        );
        assert_eq!(row.version, 7);
    }
}
