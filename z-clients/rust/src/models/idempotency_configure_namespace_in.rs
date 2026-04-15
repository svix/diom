// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyConfigureNamespaceIn {
    pub name: String,
}

impl IdempotencyConfigureNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
