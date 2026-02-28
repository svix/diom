// this file is @generated
use serde::{Deserialize, Serialize};

use super::stream_msg_out::StreamMsgOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamReceiveOut {
    pub msgs: Vec<StreamMsgOut>,
}

impl StreamReceiveOut {
    pub fn new(msgs: Vec<StreamMsgOut>) -> Self {
        Self { msgs }
    }
}
