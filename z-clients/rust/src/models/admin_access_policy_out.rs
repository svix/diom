// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyOut {
    pub id: String,

    pub description: String,

    pub rules: Vec<AccessRule>,

    pub created: u64,

    pub updated: u64,
}

impl AdminAccessPolicyOut {
    pub fn new(
        id: String,
        description: String,
        rules: Vec<AccessRule>,
        created: u64,
        updated: u64,
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
