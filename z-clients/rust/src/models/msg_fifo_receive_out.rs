// this file is @generated
use serde::{Deserialize, Serialize};

use super::fifo_msg_out::FifoMsgOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgFifoReceiveOut {
    pub msgs: Vec<FifoMsgOut>,
}

impl MsgFifoReceiveOut {
    pub fn new(msgs: Vec<FifoMsgOut>) -> Self {
        Self { msgs }
    }
}
