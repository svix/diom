use std::fmt::Display;

use diom_error::{Error, Result};
use fjall::{OptimisticTxKeyspace, OptimisticWriteTx, Readable};
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
        format!("{prefix}{key}").into()
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

    fn fetch(keyspace: &fjall::Keyspace, key: &Self::Key) -> Result<Option<Self>> {
        let key = Self::make_fjall_key(key);
        keyspace.get(&key)?.map(Self::from_fjall_value).transpose()
    }

    fn iter(keyspace: &fjall::Keyspace) -> Result<impl Iterator<Item = Self>> {
        Ok(keyspace.iter().map(|g| {
            let v = g.value().expect("iter error?");
            let r = Self::from_fjall_value(v).expect("deserialize error?");
            r
        }))
    }

    fn take_tx(
        tx: &mut OptimisticWriteTx,
        keyspace: &OptimisticTxKeyspace,
        key: &Self::Key,
    ) -> Result<Option<Self>> {
        let key = Self::make_fjall_key(key);
        tx.take(keyspace, key)?
            .map(Self::from_fjall_value)
            .transpose()
    }

    fn get_tx(
        tx: &mut OptimisticWriteTx,
        keyspace: &OptimisticTxKeyspace,
        key: &Self::Key,
    ) -> Result<Option<Self>> {
        let key = Self::make_fjall_key(key);
        tx.get(keyspace, &key)?
            .map(Self::from_fjall_value)
            .transpose()
    }

    fn insert_tx(
        tx: &mut OptimisticWriteTx,
        keyspace: &OptimisticTxKeyspace,
        row: &Self,
    ) -> Result<()> {
        let (key, value) = row.to_fjall_entry()?;
        tx.insert(keyspace, key, value);
        Ok(())
    }

    fn remove_tx(
        tx: &mut OptimisticWriteTx,
        keyspace: &OptimisticTxKeyspace,
        row: &Self,
    ) -> Result<()> {
        let key = Self::make_fjall_key(row.get_key());
        tx.remove(keyspace, key);
        Ok(())
    }
}
