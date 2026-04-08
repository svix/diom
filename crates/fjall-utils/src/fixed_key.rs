use std::marker::PhantomData;

use crate::ReadableKeyspace;
use coyote_error::{Error, Result, ResultExt};

use serde::{Serialize, de::DeserializeOwned};

/// This is a useful wrapper for key/value pairs stored in a fjall keyspace
/// whose key and type are statically known; data is serialized as postcard with a V0 version prefix
pub struct FjallFixedKey<T: Serialize + DeserializeOwned + 'static> {
    key: &'static str,
    _phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + 'static> FjallFixedKey<T> {
    pub const fn new(key: &'static str) -> Self {
        Self {
            key,
            _phantom: PhantomData,
        }
    }

    pub fn get<K: ReadableKeyspace>(&self, keyspace: &K) -> Result<Option<T>> {
        keyspace
            .get(self.key)?
            .map(|v| {
                postcard::from_bytes::<crate::V0Wrapper<T>>(&v)
                    .map(|crate::V0Wrapper::V0(inner)| inner)
                    .or_internal_error()
            })
            .transpose()
            .map_err(|err| {
                tracing::warn!(key = self.key, ?err, "error deserializing key from DB");
                Error::internal(err)
            })
    }

    pub fn store(&self, keyspace: &fjall::Keyspace, value: &T) -> Result<()> {
        let serialized = postcard::to_allocvec(&crate::V0Wrapper::V0(value)).or_internal_error()?;
        keyspace.insert(self.key, serialized).or_internal_error()
    }

    pub fn remove(&self, keyspace: &fjall::Keyspace) -> Result<()> {
        keyspace.remove(self.key).or_internal_error()
    }

    pub fn store_tx(
        &self,
        tx: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
        value: &T,
    ) -> Result<()> {
        let serialized = postcard::to_allocvec(&crate::V0Wrapper::V0(value)).or_internal_error()?;
        tx.insert(keyspace, self.key, serialized);
        Ok(())
    }

    pub fn remove_tx(
        &self,
        tx: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
    ) -> Result<()> {
        tx.remove(keyspace, self.key);
        Ok(())
    }
}
