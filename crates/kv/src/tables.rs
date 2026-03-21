use diom_error::{Result, ResultExt};
use diom_id::NamespaceId;
use fjall_utils::{TableKey, TableRow};
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
    pub value: Vec<u8>,
    pub expiry: Option<Timestamp>,
    #[serde(default)]
    pub version: u64,
}

impl TableRow for KvPairRow {
    const ROW_TYPE: u8 = RowType::Pair as u8;
}

impl KvPairRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[key])
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

    #[cfg(test)]
    pub(crate) fn extract_ts_from_fjall_key(key: &fjall::UserKey) -> Result<Timestamp> {
        let offset = 1 /* row_type */;
        let array: [u8; 8] = (&key[offset..=size_of::<i64>()])
            .try_into()
            .or_internal_error()?;
        let millis = i64::from_be_bytes(array);
        Timestamp::from_millisecond(millis).or_internal_error()
    }

    pub(crate) fn extract_key_from_fjall_key(key: &fjall::UserKey) -> Result<(NamespaceId, &str)> {
        let namespace_offset = 1 /* row_type */ + size_of::<i64>() /* timestamp */;
        let fixed_sizes = namespace_offset + size_of::<NamespaceId>() /* namespace */;
        let namespace_id =
            NamespaceId::from_slice(&key[namespace_offset..fixed_sizes]).or_internal_error()?;
        let main_key = str::from_utf8(&key[fixed_sizes..]).or_internal_error()?;
        Ok((namespace_id, main_key))
    }
}

impl TableRow for ExpirationRow {
    const ROW_TYPE: u8 = RowType::Expiration as u8;

    // We only store data in the keys
    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        Ok(b"".into())
    }
}
