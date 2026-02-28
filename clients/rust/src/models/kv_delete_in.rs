// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvDeleteIn {
    pub key: String,
}

impl KvDeleteIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
