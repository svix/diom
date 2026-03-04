use coyote_error::{Result, ResultExt};
use fjall_utils::{TableKey, TableKeyFromFjall, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Pair = 0,
    Expiration = 1,
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
    pub(crate) fn key_for(key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[], &[key])
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {
    pub expiry: Timestamp,
    pub key: String,
}

impl ExpirationRow {
    pub(crate) fn new(expiry: Timestamp, key: String) -> Self {
        Self { expiry, key }
    }

    pub(crate) fn key_for(expiration_time: Timestamp, key: &str) -> TableKey<Self> {
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();

        TableKey::init_key(Self::ROW_TYPE, &[&ts_bytes], &[key])
    }
}

impl TableKeyFromFjall for ExpirationRow {
    type Key = String;

    fn key_from_fjall_key(key: fjall::UserKey) -> Result<Self::Key> {
        String::from_utf8(key.to_vec()).map_err_generic()
    }
}

impl TableRow for ExpirationRow {
    const ROW_TYPE: u8 = RowType::Expiration as u8;
}
