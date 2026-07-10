// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvixPollerCreateOut {
    pub topic: String,

    pub poller_id: String,
}

impl SvixPollerCreateOut {
    pub fn new(topic: String, poller_id: String) -> Self {
        Self { topic, poller_id }
    }
}
