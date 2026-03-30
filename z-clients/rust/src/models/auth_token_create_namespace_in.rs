// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateNamespaceIn {
    pub name: String,
}

impl AuthTokenCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
