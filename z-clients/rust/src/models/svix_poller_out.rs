// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvixPollerOut {
    pub topic: String,

    pub poller_id: String,

    /// The autoconfig token, obfuscated (e.g. `auto_v1_eyJh...fQ==`).
    pub token: String,
}

impl SvixPollerOut {
    pub fn new(topic: String, poller_id: String, token: String) -> Self {
        Self {
            topic,
            poller_id,
            token,
        }
    }
}
