use diom_error::Result;
use fjall::{Database, Error, KeyspaceCreateOptions};

use crate::entities::{CacheConfig, KeyValueConfig, ModuleConfig, StorageType, StreamConfig};

pub mod entities;
pub mod operations;
mod tables;

pub use self::tables::Namespace;

pub const DEFAULT_NAMESPACE_NAME: &str = "default";

pub fn namespace_name(key: &str) -> &str {
    if !key.contains('/') {
        return DEFAULT_NAMESPACE_NAME;
    }
    match key.split("/").next() {
        Some(k) => k,
        None => DEFAULT_NAMESPACE_NAME,
    }
}

#[derive(Clone)]
pub struct State {
    db: fjall::Database,
    keyspace: fjall::Keyspace,
    // TODO(jbrown|2026-02-20) this needs to live in the SerializedStateMachine, not here
    pub both_dbs: BothDatabases,
}

// Yeah this is dumb AF. I don't care
// right now.
#[derive(Clone)]
pub struct BothDatabases {
    pub ephemeral_db: fjall::Database,
    pub persistent_db: fjall::Database,
}

impl State {
    pub fn init(both_dbs: BothDatabases) -> Result<Self, Error> {
        const NAMESPACE_KEYSPACE: &str = "_diom_cfggroup";

        let db = both_dbs.persistent_db.clone();
        let keyspace = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(NAMESPACE_KEYSPACE, || opts)?
        };

        Ok(Self {
            db,
            both_dbs,
            keyspace,
        })
    }

    pub fn db(&self) -> &fjall::Database {
        &self.db
    }

    pub fn keyspace(&self) -> &fjall::Keyspace {
        &self.keyspace
    }

    pub fn flush_and_sync(&self) -> Result<(), Error> {
        self.db.persist(fjall::PersistMode::SyncAll)?;
        self.both_dbs
            .persistent_db
            .persist(fjall::PersistMode::SyncAll)?;
        self.both_dbs
            .ephemeral_db
            .persist(fjall::PersistMode::SyncAll)?;
        Ok(())
    }

    pub fn fetch_namespace<C: ModuleConfig>(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<C>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_kv_namespace(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<KeyValueConfig>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_cache_namespace(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<CacheConfig>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_idempotency_namespace(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<KeyValueConfig>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_stream_namespace(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<StreamConfig>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_namespace_with_default<C: ModuleConfig>(
        &self,
        namespace_name: String,
    ) -> Result<Option<Namespace<C>>> {
        let fetch = self.fetch_namespace(&namespace_name)?;
        if fetch.is_some() {
            return Ok(fetch);
        }
        tracing::trace!(
            namespace_name,
            "cannot find namespace, falling back to default namespace"
        );

        Namespace::fetch(&self.keyspace, DEFAULT_NAMESPACE_NAME)
    }

    pub fn fetch_all_namespaces<C: ModuleConfig>(
        &self,
    ) -> Result<impl Iterator<Item = Result<Namespace<C>>>> {
        Namespace::fetch_all(&self.keyspace)
    }

    pub fn give_me_the_right_db<C: ModuleConfig>(&self, namespace: &Namespace<C>) -> Database {
        match namespace.storage_type {
            StorageType::Persistent => self.both_dbs.persistent_db.clone(),
            StorageType::Ephemeral => self.both_dbs.ephemeral_db.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_name() {
        assert_eq!(namespace_name("tom/bar"), "tom");
        assert_eq!(namespace_name("tom/bar/baz"), "tom");
        assert_eq!(namespace_name("bill"), DEFAULT_NAMESPACE_NAME);
    }
}
