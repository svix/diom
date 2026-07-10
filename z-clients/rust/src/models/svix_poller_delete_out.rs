// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvixPollerDeleteOut {
    pub topic: String,

    pub poller_id: String,

    pub success: bool,
}

impl SvixPollerDeleteOut {
    pub fn new(topic: String, poller_id: String, success: bool) -> Self {
        Self {
            topic,
            poller_id,
            success,
        }
    }
}
