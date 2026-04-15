// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimitConfigureNamespaceIn {
    pub name: String,
}

impl RateLimitConfigureNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
