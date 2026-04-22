use diom_core::{
    PersistableValue,
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

#[derive(Serialize, Deserialize, PersistableValue)]
pub struct KvPairRow {
    pub value: ByteString,
    pub expiry: Option<UnixTimestampMs>,
    pub version: u64,
}

impl TableRow for KvPairRow {
    const ROW_TYPE: u8 = RowType::Pair as u8;
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
