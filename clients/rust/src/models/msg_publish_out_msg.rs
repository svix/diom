// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishOutMsg {
    pub topic: String,

    pub offset: u64,
}

impl MsgPublishOutMsg {
    pub fn new(topic: String, offset: u64) -> Self {
        Self { topic, offset }
    }
}
