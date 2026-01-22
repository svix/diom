// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueSendIn {
    /// Delay before messages become available (seconds). Mutually exclusive with scheduled_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<u64>,

    /// Array of message payloads to send
    pub messages: Vec<String>,

    pub name: String,

    /// Specific time when messages should become available. Mutually exclusive with delay_seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<jiff::Timestamp>,
}

impl QueueSendIn {
    pub fn new(messages: Vec<String>, name: String) -> Self {
        Self {
            delay_seconds: None,
            messages,
            name,
            scheduled_at: None,
        }
    }
}
