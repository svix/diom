// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvConfigureNamespaceIn {
    pub name: String,
}

impl KvConfigureNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
