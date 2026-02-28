// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_publish_out_msg::MsgPublishOutMsg;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishOut {
    pub msgs: Vec<MsgPublishOutMsg>,
}

impl MsgPublishOut {
    pub fn new(msgs: Vec<MsgPublishOutMsg>) -> Self {
        Self { msgs }
    }
}
