use std::marker::PhantomData;

use super::readonly_db::ReadableKeyspace;
use coyote_error::{Result, ResultExt};
use serde::{Serialize, de::DeserializeOwned};

/// A trait for types that can be stored as rows in a fjall keyspace.
///
/// This is useful for having logical "tables" within the same keyspace.
pub trait TableRow: Sized + Serialize + DeserializeOwned {
    // FIXME: can probably get rid of this, and encode it in the type system.
    const ROW_TYPE: u8;

    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec_named(&self)
            .map(|bytes| bytes.into())
            .map_err_generic()
    }

    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err_generic()
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

    fn values<K: ReadableKeyspace>(keyspace: &K) -> Result<impl Iterator<Item = Self>> {
        let prefix = &[Self::ROW_TYPE];
        Ok(keyspace.prefix(prefix).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
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

/// Can't change the size, will break everything.
pub type TableKeyType = u8;

pub struct TableKey<Tag: TableRow> {
    key: fjall::UserKey,
    _table: PhantomData<Tag>,
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
