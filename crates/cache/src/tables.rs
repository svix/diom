use diom_core::types::ByteString;
use diom_id::NamespaceId;
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Cache = 0,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CacheRow {
    pub value: ByteString,
    pub expiry: Timestamp,
}

impl TableRow for CacheRow {
    const ROW_TYPE: u8 = RowType::Cache as u8;
}

impl CacheRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}
