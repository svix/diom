// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheDeleteIn {
    pub key: String,
}

impl CacheDeleteIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
