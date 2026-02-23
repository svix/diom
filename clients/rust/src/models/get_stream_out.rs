// this file is @generated
use serde::{Deserialize, Serialize};

use super::storage_type::StorageType;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetStreamOut {
    pub created_at: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_byte_size: Option<u64>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_seconds: Option<u64>,

    pub storage_type: StorageType,

    pub updated_at: jiff::Timestamp,
}

impl GetStreamOut {
    pub fn new(
        created_at: jiff::Timestamp,
        name: String,
        storage_type: StorageType,
        updated_at: jiff::Timestamp,
    ) -> Self {
        Self {
            created_at,
            max_byte_size: None,
            name,
            retention_period_seconds: None,
            storage_type,
            updated_at,
        }
    }
}
