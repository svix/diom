// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueSendOut {
    /// Array of unique message IDs for the enqueued messages
    pub message_ids: Vec<String>,
}

impl QueueSendOut {
    pub fn new(message_ids: Vec<String>) -> Self {
        Self { message_ids }
    }
}
