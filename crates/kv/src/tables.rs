use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    KvPairRow::TABLE_PREFIX,
    ExpirationRow::TABLE_PREFIX,
]));

#[derive(Serialize, Deserialize)]
pub struct KvPairRow {
    pub key: String,
    pub value: Vec<u8>,
    pub expires_at: Option<Timestamp>,
}

impl TableRow for KvPairRow {
    const TABLE_PREFIX: &'static str = "_KV_PAIR_";
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.key
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {
    pub expiration_time: Timestamp,
    pub key: String,

    #[serde(skip)]
    computed_key: String,
}

impl ExpirationRow {
    pub(crate) fn new(expiration_time: Timestamp, key: String) -> Self {
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();
        let ts_hex = hex::encode(ts_bytes);
        let computed_key = format!("{ts_hex}\0{key}");

        Self {
            expiration_time,
            key,
            computed_key,
        }
    }
}

impl TableRow for ExpirationRow {
    const TABLE_PREFIX: &'static str = "_KV_EXP_";

    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.computed_key
    }
}
