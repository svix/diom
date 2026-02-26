// this file is @generated
use serde::{Deserialize, Serialize};

use super::{retention::Retention, storage_type::StorageType};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetMsgTopicOut {
    pub created: jiff::Timestamp,

    pub name: String,

    pub retention: Retention,

    pub storage_type: StorageType,

    pub updated: jiff::Timestamp,
}

impl GetMsgTopicOut {
    pub fn new(
        created: jiff::Timestamp,
        name: String,
        retention: Retention,
        storage_type: StorageType,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            created,
            name,
            retention,
            storage_type,
            updated,
        }
    }
}
