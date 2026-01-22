// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueNackIn {
    /// Message ID to negative acknowledge
    pub message_id: String,

    pub name: String,
}

impl QueueNackIn {
    pub fn new(message_id: String, name: String) -> Self {
        Self { message_id, name }
    }
}
