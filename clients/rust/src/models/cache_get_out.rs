// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetOut {
    /// Time of expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<jiff::Timestamp>,

    pub key: String,

    pub value: String,
}

impl CacheGetOut {
    pub fn new(key: String, value: String) -> Self {
        Self {
            expires_at: None,
            key,
            value,
        }
    }
}
