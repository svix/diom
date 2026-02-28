// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicConfigureIn {
    pub partitions: u16,

    pub topic: String,
}

impl MsgTopicConfigureIn {
    pub fn new(partitions: u16, topic: String) -> Self {
        Self { partitions, topic }
    }
}
