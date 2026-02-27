// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamMsgOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub offset: u64,

    pub timestamp: jiff::Timestamp,

    pub topic: String,

    pub value: Vec<u8>,
}

impl StreamMsgOut {
    pub fn new(offset: u64, timestamp: jiff::Timestamp, topic: String, value: Vec<u8>) -> Self {
        Self {
            headers: None,
            offset,
            timestamp,
            topic,
            value,
        }
    }
}
