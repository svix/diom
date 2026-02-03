use std::{borrow::Cow, num::NonZeroU64};

use coyote_error::Result;
use fjall::Keyspace;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ConfigGroupId, ModuleConfig, StorageType};

#[derive(Serialize, Deserialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct ConfigGroup<C: ModuleConfig> {
    pub id: ConfigGroupId,
    pub name: String,
    pub storage_type: StorageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,

    // Module-specific
    pub config: C,
}

impl<C: ModuleConfig> TableRow for ConfigGroup<C> {
    const TABLE_PREFIX: &'static str = "";
    type Key = Vec<u8>;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(Self::key(&self.name))
    }
}

impl<C: ModuleConfig> ConfigGroup<C> {
    pub(crate) fn key(group_name: &str) -> Vec<u8> {
        let module = (C::module() as u8).to_be_bytes();

        let mut key = Vec::with_capacity(module.len() + b"\0".len() + group_name.len());
        key.extend_from_slice(&module);
        key.extend_from_slice(b"\0");
        key.extend_from_slice(group_name.as_bytes());
        key
    }

    pub(crate) fn fetch(keyspace: &Keyspace, group_name: &str) -> Result<Option<Self>> {
        let key = Self::key(group_name);
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
