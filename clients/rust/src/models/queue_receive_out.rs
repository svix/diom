// this file is @generated
use serde::{Deserialize, Serialize};

use super::queue_message::QueueMessage;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueReceiveOut {
    /// Array of received messages (empty if no messages available)
    pub messages: Vec<QueueMessage>,
}

impl QueueReceiveOut {
    pub fn new(messages: Vec<QueueMessage>) -> Self {
        Self { messages }
    }
}
