use std::borrow::Cow;

use fjall::Keyspace;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use diom_error::Result;

use crate::entities::{ConfigGroupId, ModuleConfig, StorageType};

#[derive(Serialize, Deserialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct ConfigGroup<C: ModuleConfig> {
    pub id: ConfigGroupId,
    pub name: String,
    pub storage_type: StorageType,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,

    // Module-specific
    pub config: C,
}

impl<C: ModuleConfig> TableRow for ConfigGroup<C> {
    const TABLE_PREFIX: &'static str = C::TABLE_PREFIX;
    type Key = String;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.name)
    }
}

impl<C: ModuleConfig> ConfigGroup<C> {
    // TODO: Bytes not string
    pub(crate) fn key(group_name: &str) -> String {
        format!("{}\0{group_name}", C::module())
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
