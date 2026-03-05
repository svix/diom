use fjall::Database;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ReadonlyDatabase;

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub enum StorageType {
    #[default]
    Persistent = 0,
    Ephemeral = 1,
}

/// A handle to both of the databases. Should only ever be accessed while holding
/// a read lock on the raft StateMachine
pub struct Databases {
    pub persistent: Database,
    pub ephemeral: Database,
}

impl Databases {
    /// Construct a new handle from raw fjall databases
    pub fn new(persistent: Database, ephemeral: Database) -> Self {
        Self {
            persistent,
            ephemeral,
        }
    }

    /// Get a handle to the particular database for a given StorageType
    pub fn db_for(&self, storage_type: StorageType) -> &Database {
        match storage_type {
            StorageType::Persistent => &self.persistent,
            StorageType::Ephemeral => &self.ephemeral,
        }
    }

    /// Construct a `ReadonlyDatabases` handle from this handle
    pub fn readonly(&self) -> ReadonlyDatabases {
        ReadonlyDatabases {
            inner: Databases {
                persistent: self.persistent.clone(),
                ephemeral: self.ephemeral.clone(),
            },
        }
    }
}

/// A handle to readonly versions of both databases
pub struct ReadonlyDatabases {
    inner: Databases,
}

impl Clone for ReadonlyDatabases {
    fn clone(&self) -> Self {
        self.inner.readonly()
    }
}

pub trait ReadonlyConnection {
    fn db_for(&self, storage_type: StorageType) -> ReadonlyDatabase;
}

impl ReadonlyConnection for Databases {
    fn db_for(&self, storage_type: StorageType) -> ReadonlyDatabase {
        match storage_type {
            StorageType::Persistent => ReadonlyDatabase::new(self.persistent.clone()),
            StorageType::Ephemeral => ReadonlyDatabase::new(self.ephemeral.clone()),
        }
    }
}

impl ReadonlyConnection for ReadonlyDatabases {
    fn db_for(&self, storage_type: StorageType) -> ReadonlyDatabase {
        match storage_type {
            StorageType::Persistent => ReadonlyDatabase::new(self.inner.persistent.clone()),
            StorageType::Ephemeral => ReadonlyDatabase::new(self.inner.ephemeral.clone()),
        }
    }
}
