// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueStatsOut {
    /// Number of available messages
    pub available: u64,

    /// Number of in-flight messages
    pub in_flight: u64,
}

impl QueueStatsOut {
    pub fn new(available: u64, in_flight: u64) -> Self {
        Self {
            available,
            in_flight,
        }
    }
}
