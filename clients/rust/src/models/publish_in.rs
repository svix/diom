// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishIn {
    pub msgs: Vec<MsgIn>,

    pub topic: String,
}

impl PublishIn {
    pub fn new(msgs: Vec<MsgIn>, topic: String) -> Self {
        Self { msgs, topic }
    }
}
