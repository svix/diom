use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::NamespaceId;
use fjall::Keyspace;
use fjall_utils::{FjallKey, TableRow};
use serde::{Deserialize, Serialize};

use crate::entities::{ModuleConfig, NamespaceName};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Namespace = 0,
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::Namespace)]
pub(crate) struct NamespaceKey {
    #[key(0)]
    pub(crate) module: u32,
    #[key(1)]
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug, PersistableValue)]
#[serde(bound = "C: ModuleConfig")]
pub struct Namespace<C: ModuleConfig> {
    pub id: NamespaceId,
    pub name: NamespaceName,

    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,

    // Module-specific
    pub config: C,
}

impl<C: ModuleConfig> TableRow for Namespace<C> {
    const ROW_TYPE: u8 = RowType::Namespace as u8;
}

impl<C: ModuleConfig> Namespace<C> {
    pub(crate) fn module_id() -> u32 {
        C::module() as u32
    }

    pub(crate) fn fetch(keyspace: &Keyspace, namespace_name: &str) -> Result<Option<Self>> {
        let key = NamespaceKey::build_key(&Self::module_id(), namespace_name);
        <Self as TableRow>::fetch(keyspace, key)
    }

    pub(crate) fn fetch_all(keyspace: &Keyspace) -> Result<impl Iterator<Item = Self>> {
        let prefix = NamespaceKey::prefix_module(&Self::module_id());
        Ok(keyspace.prefix(prefix).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }
}
