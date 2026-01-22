// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueStatsIn {
    pub name: String,
}

impl QueueStatsIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
