// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitCreateNamespaceOut {
    pub name: String,

    pub created: u64,

    pub updated: u64,
}

impl RateLimitCreateNamespaceOut {
    pub fn new(name: String, created: u64, updated: u64) -> Self {
        Self {
            name,
            created,
            updated,
        }
    }
}
