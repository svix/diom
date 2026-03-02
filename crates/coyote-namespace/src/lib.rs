use coyote_error::Result;
use fjall::{Error, KeyspaceCreateOptions};

use crate::entities::{ModuleConfig, StorageType};

pub mod entities;
pub mod operations;
mod tables;

pub use self::tables::Namespace;

pub const DEFAULT_NAMESPACE_NAME: &str = "default";

pub fn namespace_parse_key(key: &str) -> (Option<&str>, &str) {
    match key.split_once(":") {
        Some((ns, key)) => (
            if !ns.is_empty() && ns != DEFAULT_NAMESPACE_NAME {
                Some(ns)
            } else {
                None
            },
            key,
        ),
        None => (None, key),
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
        const NAMESPACE_KEYSPACE: &str = "mgmt_namespace";

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
        namespace_name: Option<&str>,
    ) -> Result<Option<Namespace<C>>> {
        if let Some(ns) = namespace_name
            && ns == DEFAULT_NAMESPACE_NAME
        {
            return Err(coyote_error::Error::http(
                coyote_error::HttpError::bad_request(
                    Some("no_explicit_default_namespace".to_owned()),
                    Some("Explicitly setting the \"default\" namespace is not allowed.".to_owned()),
                ),
            ));
        }

        Namespace::fetch(
            &self.keyspace,
            namespace_name.unwrap_or(DEFAULT_NAMESPACE_NAME),
        )
    }

    /// Like `fetch_namespace` but allows passing `default` explicitly for admin purposes.
    pub fn fetch_namespace_admin<C: ModuleConfig>(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<C>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    pub fn fetch_all_namespaces<C: ModuleConfig>(
        &self,
    ) -> Result<impl Iterator<Item = Result<Namespace<C>>>> {
        Namespace::fetch_all(&self.keyspace)
    }

    pub fn give_me_the_right_db<C: ModuleConfig>(
        &self,
        namespace: &Namespace<C>,
    ) -> fjall::Database {
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
    fn test_namespace_parse_key() {
        assert_eq!(namespace_parse_key("tom:bar"), (Some("tom"), "bar"));
        assert_eq!(namespace_parse_key("tom:bar/baz"), (Some("tom"), "bar/baz"));
        assert_eq!(namespace_parse_key("bill"), (None, "bill"));
        assert_eq!(namespace_parse_key(":bar"), (None, "bar"));
        assert_eq!(
            namespace_parse_key(&format!("{DEFAULT_NAMESPACE_NAME}:bar")),
            (None, "bar")
        );
    }
}
