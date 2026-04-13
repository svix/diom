// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueMsgOut {
    pub msg_id: String,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    pub headers: std::collections::HashMap<String, String>,

    pub timestamp: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<u64>,
}

impl QueueMsgOut {
    pub fn new(
        msg_id: String,
        value: Vec<u8>,
        headers: std::collections::HashMap<String, String>,
        timestamp: u64,
    ) -> Self {
        Self {
            msg_id,
            value,
            headers,
            timestamp,
            scheduled_at: None,
        }
    }

    pub fn with_scheduled_at(mut self, value: impl Into<Option<u64>>) -> Self {
        self.scheduled_at = value.into();
        self
    }
}
