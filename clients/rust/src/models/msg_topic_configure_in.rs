// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicConfigureIn {
    pub topic: String,

    pub partitions: u16,
}

impl MsgTopicConfigureIn {
    pub fn new(topic: String, partitions: u16) -> Self {
        Self { topic, partitions }
    }
}
