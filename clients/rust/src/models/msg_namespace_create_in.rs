// this file is @generated
use serde::{Deserialize, Serialize};

use super::{retention::Retention, storage_type::StorageType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgNamespaceCreateIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,
}

impl MsgNamespaceCreateIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            retention: None,
            storage_type: None,
        }
    }
}
