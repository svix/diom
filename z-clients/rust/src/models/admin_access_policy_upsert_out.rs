// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyUpsertOut {
    pub id: String,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub created: jiff::Timestamp,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub updated: jiff::Timestamp,
}

impl AdminAccessPolicyUpsertOut {
    pub fn new(id: String, created: jiff::Timestamp, updated: jiff::Timestamp) -> Self {
        Self {
            id,
            created,
            updated,
        }
    }
}
