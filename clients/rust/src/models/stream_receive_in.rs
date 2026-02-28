// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamReceiveIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    pub consumer_group: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_millis: Option<u64>,

    pub name: String,

    pub topic: String,
}

impl StreamReceiveIn {
    pub fn new(consumer_group: String, name: String, topic: String) -> Self {
        Self {
            batch_size: None,
            consumer_group,
            lease_duration_millis: None,
            name,
            topic,
        }
    }
}
