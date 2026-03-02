// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgStreamCommitIn {}

impl MsgStreamCommitIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgStreamCommitIn_ {
    pub topic: String,

    pub consumer_group: String,

    pub offset: u64,
}
