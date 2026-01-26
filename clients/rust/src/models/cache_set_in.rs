// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheSetIn {
    /// Time to live in milliseconds
    pub expire_in: u64,

    pub key: String,

    pub value: String,
}

impl CacheSetIn {
    pub fn new(expire_in: u64, key: String, value: String) -> Self {
        Self {
            expire_in,
            key,
            value,
        }
    }
}
