// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetOut {
    /// Time of expiry
    pub expiry: jiff::Timestamp,

    #[serde(
        with = "crate::serde_bytes_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Vec<u8>>,
}

impl CacheGetOut {
    pub fn new(expiry: jiff::Timestamp) -> Self {
        Self {
            expiry,
            value: None,
        }
    }

    pub fn with_value(mut self, value: impl Into<Option<Vec<u8>>>) -> Self {
        self.value = value.into();
        self
    }
}
