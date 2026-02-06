use diom_error::Result;
use fjall::{Database, Error, KeyspaceCreateOptions};

use crate::entities::{ModuleConfig, StorageType};

pub mod entities;
pub mod operations;
mod tables;

pub use self::tables::ConfigGroup;

pub const DEFAULT_GROUP_NAME: &str = "default";

pub fn group_name(key: &str) -> &str {
    if !key.contains('/') {
        return DEFAULT_GROUP_NAME;
    }
    match key.split("/").next() {
        Some(k) => k,
        None => DEFAULT_GROUP_NAME,
    }
}

#[derive(Clone)]
pub struct State {
    db: fjall::Database,
    keyspace: fjall::Keyspace,
    both_dbs: BothDatabases,
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
        const CONFIGGROUP_KEYSPACE: &str = "_diom_cfggroup";

        let db = both_dbs.persistent_db.clone();
        let keyspace = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(CONFIGGROUP_KEYSPACE, || opts)?
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

    pub fn fetch_group<C: ModuleConfig>(&self, group_name: &str) -> Result<Option<ConfigGroup<C>>> {
        ConfigGroup::fetch(&self.keyspace, group_name)
    }

    pub fn fetch_group_with_default<C: ModuleConfig>(
        &self,
        group_name: String,
    ) -> Result<Option<ConfigGroup<C>>> {
        let fetch = self.fetch_group(&group_name)?;
        if fetch.is_some() {
            return Ok(fetch);
        }
        tracing::trace!(
            group_name,
            "cannot find group, falling back to default group"
        );

        ConfigGroup::fetch(&self.keyspace, DEFAULT_GROUP_NAME)
    }

    pub fn fetch_all_groups<C: ModuleConfig>(
        &self,
    ) -> Result<impl Iterator<Item = Result<ConfigGroup<C>>>> {
        ConfigGroup::fetch_all(&self.keyspace)
    }

    pub fn give_me_the_right_db<C: ModuleConfig>(&self, configgroup: &ConfigGroup<C>) -> Database {
        match configgroup.storage_type {
            StorageType::Persistent => self.both_dbs.persistent_db.clone(),
            StorageType::Ephemeral => self.both_dbs.ephemeral_db.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_name() {
        assert_eq!(group_name("tom/bar"), "tom");
        assert_eq!(group_name("tom/bar/baz"), "tom");
        assert_eq!(group_name("bill"), DEFAULT_GROUP_NAME);
    }
}
