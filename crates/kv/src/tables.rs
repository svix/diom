use coyote_error::{Error, Result};
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Pair = 0,
    Expiration = 1,
    // FIXME: delete these two:
    OldExpiration = 2,
}

#[derive(Serialize, Deserialize)]
pub struct KvPairRow {
    pub key: String,
    pub value: Vec<u8>,
    pub expiry: Option<Timestamp>,
}

impl TableRow for KvPairRow {
    const ROW_TYPE: u8 = RowType::Pair as u8;
}

impl KvPairRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {}

impl ExpirationRow {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn key_for(
        namespace_id: NamespaceId,
        expiration_time: Timestamp,
        key: &str,
    ) -> TableKey<Self> {
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();

        TableKey::init_key(
            Self::ROW_TYPE,
            &[&ts_bytes, namespace_id.as_bytes()],
            &[key],
        )
    }

    pub(crate) fn extract_key_from_fjall_key(key: &fjall::UserKey) -> Result<(NamespaceId, &str)> {
        let namespace_offset = 1 /* row_type */ + size_of::<i64>() /* timestamp */;
        let fixed_sizes = namespace_offset + size_of::<NamespaceId>() /* namespace */;
        let namespace_id =
            NamespaceId::from_slice(&key[namespace_offset..fixed_sizes]).map_err(Error::generic)?;
        let main_key = str::from_utf8(&key[fixed_sizes..]).map_err(Error::generic)?;
        Ok((namespace_id, main_key))
    }
}

impl TableRow for ExpirationRow {
    const ROW_TYPE: u8 = RowType::Expiration as u8;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct OldExpirationRow {
    pub expiry: Timestamp,
    pub key: String,
}

impl OldExpirationRow {
    pub(crate) fn new(expiry: Timestamp, key: String) -> Self {
        Self { expiry, key }
    }

    pub(crate) fn key_for(expiration_time: Timestamp, key: &str) -> TableKey<Self> {
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();

        TableKey::init_key(Self::ROW_TYPE, &[&ts_bytes], &[key])
    }
}

impl TableRow for OldExpirationRow {
    const ROW_TYPE: u8 = RowType::OldExpiration as u8;
}
