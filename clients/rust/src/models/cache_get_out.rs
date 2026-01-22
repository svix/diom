// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetOut {
    /// Time of expiry
    pub expires_at: u64,

    pub key: String,

    pub value: String,
}

impl CacheGetOut {
    pub fn new(expires_at: u64, key: String, value: String) -> Self {
        Self {
            expires_at,
            key,
            value,
        }
    }
}
