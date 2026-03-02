// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgStreamReceiveIn {
    pub topic: String,

    pub consumer_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_millis: Option<u64>,
}

impl MsgStreamReceiveIn {
    pub fn new(topic: String, consumer_group: String) -> Self {
        Self {
            topic,
            consumer_group,
            batch_size: None,
            lease_duration_millis: None,
        }
    }
}
