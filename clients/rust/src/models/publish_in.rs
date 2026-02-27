// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishIn {
    pub msgs: Vec<MsgIn>,

    pub name: String,

    pub topic: String,
}

impl PublishIn {
    pub fn new(msgs: Vec<MsgIn>, name: String, topic: String) -> Self {
        Self { msgs, name, topic }
    }
}
