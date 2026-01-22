// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetOut {
    /// Time of expiry
    pub expires_at: jiff::Timestamp,

    pub key: String,

    pub value: String,
}

impl KvGetOut {
    pub fn new(expires_at: jiff::Timestamp, key: String, value: String) -> Self {
        Self {
            expires_at,
            key,
            value,
        }
    }
}
