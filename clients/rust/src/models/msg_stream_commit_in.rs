// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgStreamCommitIn {
    pub topic: String,

    pub consumer_group: String,

    pub offset: u64,
}

impl MsgStreamCommitIn {
    pub fn new(topic: String, consumer_group: String, offset: u64) -> Self {
        Self {
            topic,
            consumer_group,
            offset,
        }
    }
}
