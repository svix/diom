// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetOut {
    pub version: u64,
}

impl KvSetOut {
    pub fn new(version: u64) -> Self {
        Self { version }
    }
}
