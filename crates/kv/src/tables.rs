use std::borrow::Cow;

use coyote_error::ResultExt;
use fjall_utils::{TableKey, TableRow};
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
    pub expiry: Option<Timestamp>,
}

impl TableRow for KvPairRow {
    const TABLE_PREFIX: &'static str = "_KV_PAIR_";
    type Key = String;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.key)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {
    pub expiry: Timestamp,
    pub key: String,

    #[serde(skip)]
    computed_key: ExpirationKey,
}

impl ExpirationRow {
    pub(crate) fn new(expiry: Timestamp, key: String) -> Self {
        Self {
            computed_key: ExpirationKey::from(expiry, key.clone()),
            expiry,
            key,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ExpirationKey(String);

impl ExpirationKey {
    pub(crate) fn from(expiration_time: Timestamp, key: String) -> Self {
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();
        let ts_hex = hex::encode(ts_bytes);
        let computed_key = format!("{ts_hex}\0{key}");

        Self(computed_key)
    }
}

impl TableKey for ExpirationKey {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.0.as_bytes())
    }

    fn try_from_bytes(bytes: &[u8]) -> coyote_error::Result<Self> {
        String::from_utf8(bytes.to_owned())
            .map(Self)
            .map_err_generic()
    }
}

impl TableRow for ExpirationRow {
    const TABLE_PREFIX: &'static str = "_KV_EXP_";

    type Key = ExpirationKey;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.computed_key)
    }
}
