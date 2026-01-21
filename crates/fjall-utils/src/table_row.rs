use std::fmt::Display;

use diom_error::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};

/// A trait for types that can be stored as rows in a fjall keyspace.
///
/// This is useful for having logical "tables" within the same keyspace.
pub trait TableRow: Sized + Serialize + DeserializeOwned {
    const TABLE_PREFIX: &'static str;

    type Key: Display;

    /// Return the field used for indexing into the table.
    fn get_key(&self) -> &Self::Key;

    fn make_fjall_key(key: &Self::Key) -> fjall::UserKey {
        let prefix = Self::TABLE_PREFIX;
        format!("{prefix}\0{key}").into()
    }

    fn to_fjall_entry(&self) -> Result<(fjall::UserKey, fjall::UserValue)> {
        let key = Self::make_fjall_key(self.get_key());
        // FIXME(@svix-gabriel) - it's not clear if we're committed to using msgpack
        // for internal serialization. Using messagepack for now, but this
        // should be easy to change later.
        let value = rmp_serde::to_vec(&self).map_err(Error::generic)?;

        Ok((key, value.into()))
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err(Error::generic)
    }

    // fn fetch(keyspace: &fjall::Keyspace, key: &Self::Key) -> Result<Option<Self>> {
    //     let key = Self::make_fjall_key(key);
    //     keyspace.get(&key)?.map(Self::from_fjall_value).transpose()
    // }
}
