// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishIn {
    pub topic: String,

    pub msgs: Vec<MsgIn>,
}

impl MsgPublishIn {
    pub fn new(topic: String, msgs: Vec<MsgIn>) -> Self {
        Self { topic, msgs }
    }
}
