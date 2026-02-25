use std::marker::PhantomData;

use crate::ReadableKeyspace;
use diom_error::{Error, Result, ResultExt};

use serde::{Serialize, de::DeserializeOwned};

/// This is a useful wrapper for key/value pairs stored in a fjall keyspace
/// whose key and type are statically known; data is always serialized as msgpack
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
            .map(|v| rmp_serde::from_slice(&v).map_err_generic())
            .transpose()
            .map_err(|err| {
                tracing::warn!(key = self.key, ?err, "error deserializing key from DB");
                Error::generic(err)
            })
    }

    pub fn store(&self, keyspace: &fjall::Keyspace, value: &T) -> diom_error::Result<()> {
        let serialized = rmp_serde::encode::to_vec_named(&value).map_err_generic()?;
        keyspace.insert(self.key, serialized).map_err_generic()
    }

    pub fn remove(&self, keyspace: &fjall::Keyspace) -> diom_error::Result<()> {
        keyspace.remove(self.key).map_err_generic()
    }

    pub fn store_tx(
        &self,
        tx: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
        value: &T,
    ) -> diom_error::Result<()> {
        let serialized = rmp_serde::encode::to_vec_named(&value).map_err_generic()?;
        tx.insert(keyspace, self.key, serialized);
        Ok(())
    }

    pub fn remove_tx(
        &self,
        tx: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
    ) -> diom_error::Result<()> {
        tx.remove(keyspace, self.key);
        Ok(())
    }
}
