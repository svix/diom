// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueReceiveIn {
    /// Maximum number of messages to receive (default: 1, max: 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u32>,

    pub name: String,

    /// Visibility timeout in seconds (how long before message returns to queue if not ack'd)
    pub visibility_timeout_seconds: u64,
}

impl QueueReceiveIn {
    pub fn new(name: String, visibility_timeout_seconds: u64) -> Self {
        Self {
            batch_size: None,
            name,
            visibility_timeout_seconds,
        }
    }
}
