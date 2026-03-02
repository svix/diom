// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct CacheSetIn {}

impl CacheSetIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CacheSetIn_ {
    pub key: String,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    pub ttl: u64,
}
