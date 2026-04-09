use std::{
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use super::readonly_db::ReadableKeyspace;
use coyote_error::{Result, ResultExt};
use serde::{Serialize, de::DeserializeOwned};
use tap::Pipe;

/// A trait for types that can be stored as rows in a fjall keyspace.
///
/// This is useful for having logical "tables" within the same keyspace.
pub trait TableRow: Sized + Serialize + DeserializeOwned {
    // FIXME: can probably get rid of this, and encode it in the type system.
    const ROW_TYPE: u8;

    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec_named(&self)
            .map(|bytes| bytes.into())
            .or_internal_error()
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).or_internal_error()
    }

    fn fetch<K: ReadableKeyspace>(keyspace: &K, key: TableKey<Self>) -> Result<Option<Self>> {
        keyspace
            .get(key.into_fjall_key())?
            .map(Self::from_fjall_value)
            .transpose()
    }

    fn insert(keyspace: &fjall::Keyspace, key: TableKey<Self>, row: &Self) -> Result<()> {
        let fjall_key = key.key;
        let value = row.to_fjall_value()?;
        keyspace.insert(fjall_key, value)?;
        Ok(())
    }

    fn remove(keyspace: &fjall::Keyspace, key: TableKey<Self>) -> Result<()> {
        keyspace.remove(key.into_fjall_key())?;
        Ok(())
    }

    fn iter<K: ReadableKeyspace>(keyspace: &K) -> impl Iterator<Item = (fjall::Slice, Self)> {
        let prefix = &[Self::ROW_TYPE];
        keyspace.prefix(prefix).map(|g| {
            let (k, v) = g.into_inner().expect("io error");
            let v = Self::from_fjall_value(v).expect("deserialize error?");
            (k, v)
        })
    }

    fn keys<K: ReadableKeyspace>(keyspace: &K) -> Result<impl Iterator<Item = fjall::Slice>> {
        let prefix = &[Self::ROW_TYPE];
        Ok(keyspace.prefix(prefix).map(|g| g.key().expect("key error")))
    }

    fn values<K: ReadableKeyspace>(keyspace: &K) -> Result<impl Iterator<Item = Self>> {
        let prefix = &[Self::ROW_TYPE];
        Ok(keyspace.prefix(prefix).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }

    /// Scan rows in key order within `prefix`, stopping after `limit` rows.
    ///
    /// `iterator` is an exclusive start key: pass `None` to start from the beginning of the
    /// prefix, or `Some(key)` to resume after that key.
    fn list_range<K: ReadableKeyspace>(
        keyspace: &K,
        prefix: &[u8],
        iterator: Option<Vec<u8>>,
        limit: usize,
    ) -> Result<Vec<(fjall::Slice, Self)>> {
        let start = match iterator {
            None => Bound::Included(prefix.to_vec()),
            Some(key) => Bound::Excluded(key),
        };
        let mut results = Vec::new();
        for item in keyspace.range((start, Bound::Unbounded)).take(limit) {
            let (key, value) = item.into_inner()?;
            if !key.starts_with(prefix) {
                break;
            }
            let row = Self::from_fjall_value(value)?;
            results.push((key, row));
        }
        Ok(results)
    }
}

/// Adds convenience methods to fjall's WriteBatch that work with TableRow
pub trait WriteBatchExt {
    fn insert_row<T: TableRow>(
        &mut self,
        keyspace: &fjall::Keyspace,
        key: TableKey<T>,
        row: &T,
    ) -> Result<()>;

    fn remove_row<T: TableRow>(
        &mut self,
        keyspace: &fjall::Keyspace,
        key: TableKey<T>,
    ) -> Result<()>;
}

impl WriteBatchExt for fjall::OwnedWriteBatch {
    fn insert_row<T: TableRow>(
        &mut self,
        keyspace: &fjall::Keyspace,
        key: TableKey<T>,
        row: &T,
    ) -> Result<()> {
        self.insert(keyspace, key.into_fjall_key(), row.to_fjall_value()?);
        Ok(())
    }

    fn remove_row<T: TableRow>(
        &mut self,
        keyspace: &fjall::Keyspace,
        key: TableKey<T>,
    ) -> Result<()> {
        self.remove(keyspace, key.into_fjall_key());
        Ok(())
    }
}

/// Adds convenience methods to fjall's Keyspace that work with TableRow
pub trait KeyspaceExt {
    fn insert_row<T: TableRow>(&self, key: TableKey<T>, row: &T) -> Result<()>;

    fn get_row<T: TableRow>(&self, key: TableKey<T>) -> Result<Option<T>>;

    fn remove_row<T: TableRow>(&self, key: TableKey<T>) -> Result<()>;
}

impl KeyspaceExt for fjall::Keyspace {
    fn insert_row<T: TableRow>(&self, key: TableKey<T>, row: &T) -> Result<()> {
        self.insert(key.into_fjall_key(), row.to_fjall_value()?)?;
        Ok(())
    }

    fn get_row<T: TableRow>(&self, key: TableKey<T>) -> Result<Option<T>> {
        if let Some(value) = self.get(key.into_fjall_key())? {
            Some(T::from_fjall_value(value)?)
        } else {
            None
        }
        .pipe(Ok)
    }

    fn remove_row<T: TableRow>(&self, key: TableKey<T>) -> Result<()> {
        self.remove(key.into_fjall_key())?;
        Ok(())
    }
}

/// Can't change the size, will break everything.
pub type TableKeyType = u8;

pub struct TableKey<Tag: TableRow> {
    key: fjall::UserKey,
    _table: PhantomData<Tag>,
}

#[cfg(debug_assertions)]
impl<Tag: TableRow> ::std::fmt::Debug for TableKey<Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)
    }
}

impl<'a, Tag: TableRow> TableKey<Tag> {
    /// Construct the key to be used for fjall
    ///
    /// In the future: should probably just have a big enough key on the stack and use that.
    pub fn init_key(
        row_type: TableKeyType,
        fixed_parts: &[&[u8]],
        nul_delimited_parts: &[&str],
    ) -> Self {
        let len = size_of::<u8>() /* the row tag */
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
            key: ret.into(),
            _table: PhantomData,
        }
    }

    /// Helper function for parsing a key
    pub fn parse_key<const N: usize>(source: &[u8], part: &mut [u8; N], cursor: &mut usize) {
        let start = *cursor;
        let end = *cursor + N;

        part.copy_from_slice(&source[start..end]);
        *cursor = end;
    }

    pub fn init_from_bytes(key: &'a [u8]) -> Self {
        Self {
            key: key.into(),
            _table: PhantomData,
        }
    }

    pub fn into_fjall_key(self) -> fjall::UserKey {
        self.key
    }
}

pub trait TableKeyFromFjall {
    type Key;

    fn key_from_fjall_key(key: fjall::UserKey) -> Result<Self::Key>;
}

pub trait MonotonicTableKey: Copy {
    type BYTES: AsRef<[u8]>;

    const MIN: Self;
    const MAX: Self;

    fn successor(self) -> Self;
    fn to_be_bytes(self) -> Self::BYTES;
    fn from_slice(slice: &[u8]) -> Result<Self>;
}

impl MonotonicTableKey for u64 {
    type BYTES = [u8; 8];

    const MIN: u64 = u64::MIN;
    const MAX: u64 = u64::MAX;

    fn successor(self) -> u64 {
        self + 1
    }

    fn to_be_bytes(self) -> Self::BYTES {
        self.to_be_bytes()
    }

    fn from_slice(slice: &[u8]) -> Result<Self> {
        debug_assert!(slice.len() == 8);
        Ok(Self::from_be_bytes(slice.try_into().or_internal_error()?))
    }
}

fn range_helper<K, T, B>(keyspace: &K, bounds: B) -> impl DoubleEndedIterator<Item = fjall::Guard>
where
    K: ReadableKeyspace,
    T: MonotonicTableRow,
    B: RangeBounds<T::KeyType>,
{
    let start = match bounds.start_bound() {
        Bound::Unbounded => T::key_for_value(T::KeyType::MIN),
        Bound::Included(x) => T::key_for_value(*x),
        Bound::Excluded(x) => T::key_for_value((*x).successor()),
    }
    .into_fjall_key();
    match bounds.end_bound() {
        Bound::Unbounded => {
            keyspace.range(start..=T::key_for_value(T::KeyType::MAX).into_fjall_key())
        }
        Bound::Included(x) => keyspace.range(start..=T::key_for_value(*x).into_fjall_key()),
        Bound::Excluded(x) => keyspace.range(start..T::key_for_value(*x).into_fjall_key()),
    }
}

/// Specialization for TableRows whose key can be used for monotonic range operations
///
/// Specifically:
///  - the key type must have a defined "minimum" and "maximum" which follow algebraic rules
///  - the key must have a "successor" operation
///  - the key type must encode to bytes in a way that preserves ordering
pub trait MonotonicTableRow: TableRow {
    type KeyType: MonotonicTableKey;

    fn get_key(&self) -> Self::KeyType;

    #[doc(hidden)]
    fn key_for_value(val: Self::KeyType) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[val.to_be_bytes().as_ref()], &[])
    }

    fn key(&self) -> TableKey<Self> {
        Self::key_for_value(self.get_key())
    }

    /// Return an iterator over all key/value pairs within the given bounds
    fn range<K: ReadableKeyspace, B: RangeBounds<Self::KeyType> + Send + Clone>(
        keyspace: &K,
        bounds: B,
    ) -> impl DoubleEndedIterator<Item = Result<(Self::KeyType, Self)>> {
        range_helper::<K, Self, B>(keyspace, bounds).map(|item: fjall::Guard| {
            let (key, value) = item.into_inner()?;
            let key = Self::KeyType::from_slice(&key[1..])?;
            let value = Self::from_fjall_value(value)?;
            Ok((key, value))
        })
    }

    /// Remove all keys in the given range
    ///
    /// Implemented via a loop; may take a long time
    fn remove_keys_in_range<B: RangeBounds<Self::KeyType> + Send + Clone>(
        db: &fjall::Database,
        keyspace: &fjall::Keyspace,
        bounds: B,
        batch_size: usize,
        persist_mode: fjall::PersistMode,
    ) -> Result<usize> {
        let mut total_removed = 0;
        loop {
            let mut removed = 0;
            let mut tx = db.batch().durability(Some(persist_mode));
            for item in range_helper::<_, Self, B>(keyspace, bounds.clone()).take(batch_size) {
                let key = item.key().or_internal_error()?;
                tx.remove(keyspace, key);
                removed += 1;
            }
            tx.commit()?;
            if removed == 0 {
                break;
            } else {
                total_removed += removed;
            }
        }
        Ok(total_removed)
    }
}
