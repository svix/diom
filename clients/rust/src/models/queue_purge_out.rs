// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuePurgeOut {
    /// Number of messages purged
    pub purged_count: u64,
}

impl QueuePurgeOut {
    pub fn new(purged_count: u64) -> Self {
        Self { purged_count }
    }
}
