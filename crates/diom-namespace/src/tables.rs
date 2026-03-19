use std::num::NonZeroU64;

use diom_error::Result;
use diom_id::NamespaceId;
use fjall::Keyspace;
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ModuleConfig, NamespaceName, StorageType};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Namespace = 0,
}

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
    const ROW_TYPE: u8 = RowType::Namespace as u8;
}

impl<C: ModuleConfig> Namespace<C> {
    pub(crate) fn key_for(namespace_name: &str) -> TableKey<Self> {
        let module = (C::module() as u32).to_be_bytes();

        TableKey::init_key(Self::ROW_TYPE, &[&module], &[namespace_name])
    }

    pub(crate) fn fetch(keyspace: &Keyspace, namespace_name: &str) -> Result<Option<Self>> {
        let key = Self::key_for(namespace_name);
        <Self as TableRow>::fetch(keyspace, key)
    }

    pub(crate) fn fetch_all(keyspace: &Keyspace) -> Result<impl Iterator<Item = Self>> {
        let prefix = Self::key_for("");
        Ok(keyspace.prefix(prefix.into_fjall_key()).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }
}
