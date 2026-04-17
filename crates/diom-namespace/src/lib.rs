use std::sync::LazyLock;

use diom_error::Result;
use fjall::{Error, KeyspaceCreateOptions};
use fjall_utils::Databases;

use crate::entities::{ModuleConfig, NamespaceName};

pub mod entities;
pub mod operations;
mod storage;

pub use self::storage::Namespace;

pub static DEFAULT_NAMESPACE_NAME: LazyLock<NamespaceName> =
    LazyLock::new(|| NamespaceName("default".to_owned()));

pub fn parse_namespace(key: &str) -> (Option<&str>, &str) {
    match key.split_once(":") {
        Some((ns, key)) => (
            if !ns.is_empty() && ns != DEFAULT_NAMESPACE_NAME.as_str() {
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
    pub both_dbs: Databases,
}

impl State {
    pub fn init(both_dbs: Databases) -> Result<Self, Error> {
        const NAMESPACE_KEYSPACE: &str = "mgmt_namespace";

        let db = both_dbs.persistent.clone();
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

    #[tracing::instrument(skip_all, fields(?namespace_name))]
    pub fn fetch_namespace<C: ModuleConfig>(
        &self,
        namespace_name: Option<&NamespaceName>,
    ) -> Result<Option<Namespace<C>>> {
        if let Some(ns) = namespace_name
            && *ns == *DEFAULT_NAMESPACE_NAME
        {
            return Err(diom_error::Error::bad_request(
                "no_explicit_default_namespace",
                "Explicitly setting the \"default\" namespace is not allowed.",
            ));
        }

        Namespace::fetch(
            &self.keyspace,
            namespace_name.unwrap_or(&DEFAULT_NAMESPACE_NAME),
        )
    }

    /// Like `fetch_namespace` but allows passing `default` explicitly for admin purposes.
    #[tracing::instrument(skip_all, fields(?namespace_name))]
    pub fn fetch_namespace_admin<C: ModuleConfig>(
        &self,
        namespace_name: &str,
    ) -> Result<Option<Namespace<C>>> {
        Namespace::fetch(&self.keyspace, namespace_name)
    }

    #[tracing::instrument(skip_all)]
    pub fn fetch_all_namespaces<C: ModuleConfig>(
        &self,
    ) -> Result<impl Iterator<Item = Namespace<C>>> {
        Namespace::fetch_all(&self.keyspace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_parse_key() {
        assert_eq!(parse_namespace("tom:bar"), (Some("tom"), "bar"));
        assert_eq!(parse_namespace("tom:bar/baz"), (Some("tom"), "bar/baz"));
        assert_eq!(parse_namespace("bill"), (None, "bill"));
        assert_eq!(parse_namespace(":bar"), (None, "bar"));
        assert_eq!(
            parse_namespace(&format!("{}:bar", DEFAULT_NAMESPACE_NAME.as_str())),
            (None, "bar")
        );
    }
}
