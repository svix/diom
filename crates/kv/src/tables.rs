use std::{borrow::Cow, marker::PhantomData};

use diom_error::ResultExt;
use diom_namespace::entities::NamespaceId;
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

    fn try_from_bytes(bytes: &[u8]) -> diom_error::Result<Self> {
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

// Move to fjall-utils

struct MyTableKey<'a, Tag: TableRow> {
    key: Cow<'a, [u8]>,
    _unit: PhantomData<Tag>,
}

impl<'a, Tag: TableRow> MyTableKey<'a, Tag> {
    /// Construct the key to be used for fjall
    ///
    /// In the future: should probably just have a big enough key on the stack and use that.
    fn init_key(row_type: u8, fixed_parts: &[&[u8]], nul_delimited_parts: &[&str]) -> Self {
        let len = 1 /* u8 */
            + fixed_parts.iter().fold(0, |acc, e| acc + e.len()) /* all the fixed parts */
            + nul_delimited_parts.iter().fold(0, |acc, e| acc + e.len()) /* The parts that are nul delimited */
            + nul_delimited_parts.len().saturating_sub(0); /* the nul delimiters for the parts */
        let mut ret = Vec::with_capacity(len);
        ret.push(row_type);
        for part in fixed_parts {
            ret.extend_from_slice(part);
        }

        let nul_delimited_parts = itertools::Itertools::intersperse(
            nul_delimited_parts.iter().map(|x| x.as_bytes()),
            b"\0",
        );
        for part in nul_delimited_parts {
            ret.extend_from_slice(part);
        }

        Self {
            key: Cow::Owned(ret),
            _unit: PhantomData,
        }
    }

    fn init_from_bytes(key: &'a [u8]) -> Self {
        Self {
            key: Cow::Borrowed(key),
            _unit: PhantomData,
        }
    }
}

// Module specific

#[repr(u8)]
enum RowType {
    Pair = 0,
    Expiration = 1,
}

#[derive(Serialize, Deserialize)]
pub struct MyKvPairRow {
    pub value: Vec<u8>,
    pub expiry: Option<Timestamp>,
}

impl KvPairRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &str) -> MyTableKey<'_, Self> {
        MyTableKey::init_key(RowType::Pair as u8, &[namespace_id.as_bytes()], &[key])
    }
}
