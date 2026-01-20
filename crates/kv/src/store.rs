use std::{fmt::Display, num::NonZeroU64};

use diom_error::{Error, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// A row in a logical table inside the metadata keyspace.
pub(crate) trait TableRow: Sized + Serialize + DeserializeOwned {
    const TABLE_PREFIX: &'static str;

    type Key: Display;

    /// Return the field used for indexing into the table.
    fn get_key(&self) -> &Self::Key;

    fn make_fjall_key(key: &Self::Key) -> fjall::UserKey {
        let prefix = Self::TABLE_PREFIX;
        format!("{prefix}{key}").into()
    }

    fn fjall_key(&self) -> fjall::UserKey {
        Self::make_fjall_key(self.get_key())
    }

    fn fjall_value(&self) -> Result<fjall::UserValue> {
        let slice = rmp_serde::to_vec(&self).map_err(Error::generic)?;
        Ok(slice.into())
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err(Error::generic)
    }

    fn insert(&self, state: &State) -> Result<()> {
        state
            .metadata_tables
            .insert(self.fjall_key(), self.fjall_value()?)?;
        Ok(())
    }

    fn fetch(state: &State, key: &Self::Key) -> Result<Option<Self>> {
        let key = Self::make_fjall_key(key);

        state
            .metadata_tables
            .get(&key)?
            .map(Self::from_fjall_value)
            .transpose()
    }
}

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
// static_assertions::const_assert!(diom_utils::are_all_unique(&[
//     NameToStreamRow::TABLE_PREFIX,
//     StreamRow::TABLE_PREFIX,
// ]));

#[derive(Serialize, Deserialize)]
pub(crate) struct KvPairRow {
    pub key: String,
    pub value: Vec<u8>,
}

impl TableRow for KvPairRow {
    const TABLE_PREFIX: &'static str = "_KVPAIR_";
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.key
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ExpirationRow {
    pub expiration_time: Timestamp,
    pub key: String,
}

impl TableRow for ExpirationRow {
    const TABLE_PREFIX: &'static str = "_CSTRM_";

    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}
