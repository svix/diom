// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppendToStreamIn {
    pub msgs: Vec<MsgIn>,

    #[serde(rename = "streamId")]
    pub stream_id: String,
}

impl AppendToStreamIn {
    pub fn new(msgs: Vec<MsgIn>, stream_id: String) -> Self {
        Self { msgs, stream_id }
    }
}
