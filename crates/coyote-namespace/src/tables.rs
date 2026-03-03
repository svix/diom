use std::num::NonZeroU64;

use coyote_error::Result;
use fjall::Keyspace;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ModuleConfig, NamespaceId, NamespaceName, StorageType};

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "C: ModuleConfig")]
pub struct Namespace<C: ModuleConfig> {
    pub id: NamespaceId,
    pub name: NamespaceName,
    pub storage_type: StorageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,

    // Module-specific
    pub config: C,
}

impl<C: ModuleConfig> TableRow for Namespace<C> {
    const TABLE_PREFIX: &'static str = "";
    type Key = Vec<u8>;
}

impl<C: ModuleConfig> Namespace<C> {
    pub(crate) fn key(namespace_name: &str) -> Vec<u8> {
        let module = (C::module() as u8).to_be_bytes();

        let mut key = Vec::with_capacity(module.len() + b"\0".len() + namespace_name.len());
        key.extend_from_slice(&module);
        key.extend_from_slice(b"\0");
        key.extend_from_slice(namespace_name.as_bytes());
        key
    }

    pub(crate) fn fetch(keyspace: &Keyspace, namespace_name: &str) -> Result<Option<Self>> {
        let key = Self::key(namespace_name);
        <Self as TableRow>::fetch(keyspace, &key)
    }

    pub(crate) fn fetch_all(keyspace: &Keyspace) -> Result<impl Iterator<Item = Result<Self>>> {
        let prefix = format!("{}", C::module());
        Ok(keyspace.prefix(&prefix).map(|g| {
            let v = g.value()?;
            Self::from_fjall_value(v)
        }))
    }
}
