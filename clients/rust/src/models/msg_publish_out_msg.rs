// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishOutMsg {
    pub offset: u64,

    pub topic: String,
}

impl MsgPublishOutMsg {
    pub fn new(offset: u64, topic: String) -> Self {
        Self { offset, topic }
    }
}
