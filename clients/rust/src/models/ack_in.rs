// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AckIn {
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,

    #[serde(rename = "maxMsgId")]
    pub max_msg_id: u64,

    #[serde(rename = "minMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_msg_id: Option<u64>,

    #[serde(rename = "streamId")]
    pub stream_id: String,
}

impl AckIn {
    pub fn new(consumer_group: String, max_msg_id: u64, stream_id: String) -> Self {
        Self {
            consumer_group,
            max_msg_id,
            min_msg_id: None,
            stream_id,
        }
    }
}
