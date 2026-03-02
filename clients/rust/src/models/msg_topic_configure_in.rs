// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgTopicConfigureIn {}

impl MsgTopicConfigureIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgTopicConfigureIn_ {
    pub topic: String,

    pub partitions: u16,
}
