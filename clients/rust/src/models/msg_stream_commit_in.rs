// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgStreamCommitIn {
    pub consumer_group: String,

    pub offset: u64,

    pub topic: String,
}

impl MsgStreamCommitIn {
    pub fn new(consumer_group: String, offset: u64, topic: String) -> Self {
        Self {
            consumer_group,
            offset,
            topic,
        }
    }
}
