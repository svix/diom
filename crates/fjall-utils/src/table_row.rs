use std::borrow::Cow;

use diom_error::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};

/// A trait for types that can be stored as rows in a fjall keyspace.
///
/// This is useful for having logical "tables" within the same keyspace.
pub trait TableRow: Sized + Serialize + DeserializeOwned {
    const TABLE_PREFIX: &'static str;

    type Key: AsRef<[u8]> + Clone;

    /// Return the field used for indexing into the table.
    fn get_key(&self) -> Cow<'_, Self::Key>;

    fn make_fjall_key(key: &Self::Key) -> fjall::UserKey {
        let mut key_bytes =
            Vec::<u8>::with_capacity(Self::TABLE_PREFIX.len() + b"\0".len() + key.as_ref().len());
        key_bytes.extend_from_slice(Self::TABLE_PREFIX.as_bytes());
        key_bytes.extend_from_slice(b"\0");
        key_bytes.extend_from_slice(key.as_ref());
        key_bytes.into()
    }

    fn to_fjall_entry(&self) -> Result<(fjall::UserKey, fjall::UserValue)> {
        let key = Self::make_fjall_key(self.get_key().as_ref());
        // FIXME(@svix-gabriel) - it's not clear if we're committed to using msgpack
        // for internal serialization. Using messagepack for now, but this
        // should be easy to change later.
        let value = rmp_serde::to_vec(&self).map_err(Error::generic)?;

        Ok((key, value.into()))
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err(Error::generic)
    }

    fn fetch(keyspace: &fjall::Keyspace, key: &Self::Key) -> Result<Option<Self>> {
        let key = Self::make_fjall_key(key);
        keyspace.get(&key)?.map(Self::from_fjall_value).transpose()
    }

    fn insert(keyspace: &fjall::Keyspace, row: &Self) -> Result<()> {
        let (key, value) = row.to_fjall_entry()?;
        keyspace.insert(key, value)?;
        Ok(())
    }

    fn remove(keyspace: &fjall::Keyspace, key: &Self::Key) -> Result<()> {
        let key = Self::make_fjall_key(key);
        keyspace.remove(key)?;
        Ok(())
    }

    fn iter(keyspace: &fjall::Keyspace) -> Result<impl Iterator<Item = Self>> {
        Ok(keyspace.prefix(Self::TABLE_PREFIX).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }
}

/// Adds convenience methods to fjall's WriteBatch that work with TableRow
pub trait WriteBatchExt {
    fn insert_row<T: TableRow>(&mut self, keyspace: &fjall::Keyspace, row: &T) -> Result<()>;

    fn remove_row<T: TableRow>(&mut self, keyspace: &fjall::Keyspace, key: &T::Key) -> Result<()>;
}

impl WriteBatchExt for fjall::OwnedWriteBatch {
    fn insert_row<T: TableRow>(&mut self, keyspace: &fjall::Keyspace, row: &T) -> Result<()> {
        let (key, value) = row.to_fjall_entry()?;
        self.insert(keyspace, key, value);
        Ok(())
    }

    fn remove_row<T: TableRow>(&mut self, keyspace: &fjall::Keyspace, key: &T::Key) -> Result<()> {
        let key = T::make_fjall_key(key);
        self.remove(keyspace, key);
        Ok(())
    }
}
