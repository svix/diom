// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SinkDeleteOut {
    pub topic: String,

    pub consumer_group: String,

    pub success: bool,
}

impl SinkDeleteOut {
    pub fn new(topic: String, consumer_group: String, success: bool) -> Self {
        Self {
            topic,
            consumer_group,
            success,
        }
    }
}
