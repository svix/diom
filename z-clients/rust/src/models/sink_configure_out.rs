// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SinkConfigureOut {
    pub topic: String,

    pub consumer_group: String,
}

impl SinkConfigureOut {
    pub fn new(topic: String, consumer_group: String) -> Self {
        Self {
            topic,
            consumer_group,
        }
    }
}
