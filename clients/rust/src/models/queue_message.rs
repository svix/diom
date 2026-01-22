// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueMessage {
    pub message_id: String,

    pub payload: String,
}

impl QueueMessage {
    pub fn new(message_id: String, payload: String) -> Self {
        Self {
            message_id,
            payload,
        }
    }
}
