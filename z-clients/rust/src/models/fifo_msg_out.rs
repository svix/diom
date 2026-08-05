// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FifoMsgOut {
    pub msg_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    pub headers: std::collections::HashMap<String, String>,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub timestamp: jiff::Timestamp,

    #[serde(
        default,
        with = "crate::unix_timestamp_ms_serde::optional",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<jiff::Timestamp>,
}

impl FifoMsgOut {
    pub fn new(
        msg_id: String,
        value: Vec<u8>,
        headers: std::collections::HashMap<String, String>,
        timestamp: jiff::Timestamp,
    ) -> Self {
        Self {
            msg_id,
            key: None,
            value,
            headers,
            timestamp,
            scheduled_at: None,
        }
    }

    pub fn with_key(mut self, value: impl Into<Option<String>>) -> Self {
        self.key = value.into();
        self
    }

    pub fn with_scheduled_at(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.scheduled_at = value.into();
        self
    }
}
