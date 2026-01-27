use std::fmt::Display;

use blake2::{Blake2b512, Digest};
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
    pub hashed_key: HashedKey,
}

impl TableRow for KvPairRow {
    const TABLE_PREFIX: &'static str = "_KV_PAIR_";
    type Key = HashedKey;

    fn get_key(&self) -> &Self::Key {
        &self.hashed_key
    }
}

impl KvPairRow {
    pub fn new(key: String, value: Vec<u8>, expires_at: Option<Timestamp>) -> Self {
        Self {
            hashed_key: HashedKey::from(key.clone()),
            key,
            value,
            expires_at,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct HashedKey(String);

impl HashedKey {
    pub(crate) fn from(key: String) -> Self {
        // FIXME(@svix-lucho): is this a good hashing method?
        let mut hasher = Blake2b512::new();
        hasher.update(key.as_bytes());
        Self(hex::encode(hasher.finalize()))
    }
}

impl Display for HashedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {
    pub expiration_time: Timestamp,
    pub key: String,

    #[serde(skip)]
    computed_key: ExpirationKey,
}

impl ExpirationRow {
    pub(crate) fn new(expiration_time: Timestamp, key: String) -> Self {
        Self {
            computed_key: ExpirationKey::from(expiration_time, key.clone()),
            expiration_time,
            key,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExpirationKey(String);

impl ExpirationKey {
    pub(crate) fn from(expiration_time: Timestamp, key: String) -> Self {
        let hashed_key = HashedKey::from(key);
        let ts_ms = expiration_time.as_millisecond();
        let ts_bytes = ts_ms.to_be_bytes();
        let ts_hex = hex::encode(ts_bytes);
        let computed_key = format!("{ts_hex}\0{hashed_key}");

        Self(computed_key)
    }
}

impl Display for ExpirationKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TableRow for ExpirationRow {
    const TABLE_PREFIX: &'static str = "_KV_EXP_";

    type Key = ExpirationKey;

    fn get_key(&self) -> &Self::Key {
        &self.computed_key
    }
}
