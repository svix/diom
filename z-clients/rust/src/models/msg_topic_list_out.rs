// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicListOut {
    pub id: String,

    pub name: String,

    pub partitions: u64,
}

impl MsgTopicListOut {
    pub fn new(id: String, name: String, partitions: u64) -> Self {
        Self {
            id,
            name,
            partitions,
        }
    }
}
