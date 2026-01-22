// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuePurgeIn {
    pub name: String,
}

impl QueuePurgeIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
