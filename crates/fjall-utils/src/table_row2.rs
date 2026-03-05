use byteorder::{BigEndian, ByteOrder};
use fjall::Guard;
use std::{
    borrow::Cow,
    ops::{Bound, RangeBounds},
};
use uuid::Uuid;

use super::readonly_db::ReadableKeyspace;
use diom_error::{Error, Result, ResultExt};
use serde::{Serialize, de::DeserializeOwned};

/// A trait for primary keys in fjall.
pub trait TableKey: Clone {
    fn as_bytes(&self) -> Cow<'_, [u8]>;

    fn try_from_bytes(bytes: &[u8]) -> Result<Self>;
}

impl TableKey for String {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.as_bytes())
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        let owned: Vec<u8> = bytes.to_owned();
        Self::from_utf8(owned).map_err_generic()
    }
}

impl TableKey for Vec<u8> {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self)
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(bytes.to_owned())
    }
}

impl TableKey for Uuid {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.as_bytes())
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        Self::from_slice(bytes).map_err_generic()
    }
}

impl TableKey for u64 {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        let mut buf = vec![0u8; 8];
        BigEndian::write_u64(&mut buf, *self);
        Cow::Owned(buf)
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 8 {
            return Err(Error::generic("invalid byte length in key"));
        }
        Ok(BigEndian::read_u64(bytes))
    }
}

impl MonotonicTableKey for u64 {}

/// Marker trait indicating that a table key is **monotonic** with its
/// underlying byte representation; that is to say, if K1 > K2, then byte(K1) > byte(K2)
pub trait MonotonicTableKey {}

/// A trait for types that can be stored as rows in a fjall keyspace.
///
/// This is useful for having logical "tables" within the same keyspace.
pub trait TableRow: Sized + Serialize + DeserializeOwned {
    const TABLE_PREFIX: &'static str;

    type Key: TableKey;

    /// Return the field used for indexing into the table.
    fn get_key(&self) -> Cow<'_, Self::Key>;

    fn make_fjall_key(key: &Self::Key) -> fjall::UserKey {
        let raw_key_bytes = key.as_bytes();
        let mut key_bytes = if Self::TABLE_PREFIX.is_empty() {
            Vec::<u8>::with_capacity(raw_key_bytes.len())
        } else {
            let mut key_bytes = Vec::<u8>::with_capacity(
                Self::TABLE_PREFIX.len() + b"\0".len() + raw_key_bytes.len(),
            );
            key_bytes.extend_from_slice(Self::TABLE_PREFIX.as_bytes());
            key_bytes.extend_from_slice(b"\0");
            key_bytes
        };
        key_bytes.extend_from_slice(&raw_key_bytes);
        key_bytes.into()
    }

    fn to_fjall_entry(&self) -> Result<(fjall::UserKey, fjall::UserValue)> {
        let key = Self::make_fjall_key(self.get_key().as_ref());
        // FIXME(@svix-gabriel) - it's not clear if we're committed to using msgpack
        // for internal serialization. Using messagepack for now, but this
        // should be easy to change later.
        let value = rmp_serde::to_vec_named(&self).map_err_generic()?;

        Ok((key, value.into()))
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err_generic()
    }

    fn fetch<K: ReadableKeyspace>(keyspace: &K, key: &Self::Key) -> Result<Option<Self>> {
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

    fn key_from_key(key: fjall::UserKey) -> Result<Self::Key> {
        let without_bytes = if Self::TABLE_PREFIX.is_empty() {
            &key
        } else {
            &key[Self::TABLE_PREFIX.len() + 1..]
        };
        Self::Key::try_from_bytes(without_bytes)
    }

    /// Iterate over all of the key/value pairs in this table, in sorted order
    fn iter<K: ReadableKeyspace>(keyspace: &K) -> impl Iterator<Item = Result<(Self::Key, Self)>> {
        let prefix = Self::prefix_with(&[]);
        keyspace.prefix(prefix).map(|g| {
            let (k, v) = g.into_inner()?;
            let value = Self::from_fjall_value(v)?;
            let key = Self::key_from_key(k)?;
            Ok((key, value))
        })
    }

    fn values<K: ReadableKeyspace>(keyspace: &K) -> Result<impl Iterator<Item = Self>> {
        Ok(keyspace.prefix(Self::TABLE_PREFIX).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }

    fn prefix_with(bytes: &[u8]) -> Vec<u8> {
        if Self::TABLE_PREFIX.is_empty() {
            bytes.to_owned()
        } else {
            let mut prefix_bytes = Vec::with_capacity(Self::TABLE_PREFIX.len() + 1 + bytes.len());
            prefix_bytes.extend(Self::TABLE_PREFIX.as_bytes());
            prefix_bytes.push(0x00);
            prefix_bytes.extend(bytes);
            prefix_bytes
        }
    }

    fn end_prefix() -> Option<Vec<u8>> {
        if Self::TABLE_PREFIX.is_empty() {
            None
        } else {
            let mut prefix_bytes = Vec::with_capacity(Self::TABLE_PREFIX.len() + 1);
            prefix_bytes.extend(Self::TABLE_PREFIX.as_bytes());
            prefix_bytes.push(0x01);
            Some(prefix_bytes)
        }
    }
}

fn increment(v: &mut Vec<u8>) {
    for byte in v.iter_mut().rev() {
        if *byte < 0xff {
            *byte += 1;
            break;
        }
    }
    v.insert(0, 0x01)
}

fn range_helper<T, B>(
    keyspace: &fjall::Keyspace,
    bounds: B,
) -> impl DoubleEndedIterator<Item = Guard>
where
    T: TableRow,
    T::Key: MonotonicTableKey,
    B: RangeBounds<T::Key>,
{
    let start = match bounds.start_bound() {
        Bound::Unbounded => T::prefix_with(&[]),
        Bound::Included(x) => T::prefix_with(x.as_bytes().as_ref()),
        Bound::Excluded(x) => {
            let mut bytes = T::prefix_with(x.as_bytes().as_ref());
            increment(&mut bytes);
            bytes
        }
    };
    match bounds.end_bound() {
        Bound::Unbounded => {
            if let Some(end) = T::end_prefix() {
                keyspace.range(start..end)
            } else {
                keyspace.range(start..)
            }
        }
        Bound::Included(x) => keyspace.range(start..=T::prefix_with(x.as_bytes().as_ref())),
        Bound::Excluded(x) => keyspace.range(start..T::prefix_with(x.as_bytes().as_ref())),
    }
}

pub trait MonotonicTableRowExt: TableRow {
    fn range<B: RangeBounds<Self::Key>>(
        keyspace: &fjall::Keyspace,
        bounds: B,
    ) -> impl DoubleEndedIterator<Item = Result<(Self::Key, Self)>>;

    fn keys_in_range<B: RangeBounds<Self::Key>>(
        keyspace: &fjall::Keyspace,
        bounds: B,
    ) -> Result<Vec<fjall::UserKey>>;
}

impl<T: TableRow> MonotonicTableRowExt for T
where
    T::Key: MonotonicTableKey,
{
    fn range<B: RangeBounds<Self::Key>>(
        keyspace: &fjall::Keyspace,
        bounds: B,
    ) -> impl DoubleEndedIterator<Item = Result<(Self::Key, Self)>> {
        range_helper::<Self, B>(keyspace, bounds).map(|g| {
            let (k, v) = g.into_inner()?;
            let value = Self::from_fjall_value(v)?;
            let key = Self::key_from_key(k)?;
            Ok((key, value))
        })
    }

    fn keys_in_range<B: RangeBounds<Self::Key>>(
        keyspace: &fjall::Keyspace,
        bounds: B,
    ) -> Result<Vec<fjall::UserKey>> {
        range_helper::<Self, B>(keyspace, bounds)
            .map(|g| g.key().map_err(Into::into))
            .collect()
    }
}

/// Adds convenient methods to fjall's Keyspace to work with TableRow
pub trait KeyspaceExt {
    fn ingest_rows<T: TableRow, I: Iterator<Item = T>>(&self, rows: I) -> Result<()>;
}

impl KeyspaceExt for fjall::Keyspace {
    fn ingest_rows<T: TableRow, I: Iterator<Item = T>>(&self, rows: I) -> Result<()> {
        let mut i = self.start_ingestion()?;
        for row in rows {
            let (k, v) = row.to_fjall_entry()?;
            i.write(k, v)?;
        }
        i.finish()?;
        Ok(())
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
