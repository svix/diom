// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyOut {
    pub id: String,

    pub description: String,

    pub rules: Vec<AccessRule>,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub created: jiff::Timestamp,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub updated: jiff::Timestamp,
}

impl AdminAccessPolicyOut {
    pub fn new(
        id: String,
        description: String,
        rules: Vec<AccessRule>,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            id,
            description,
            rules,
            created,
            updated,
        }
    }
}
