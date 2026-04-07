// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleOut {
    pub id: String,

    pub description: String,

    pub rules: Vec<AccessRule>,

    pub policies: Vec<String>,

    pub context: std::collections::HashMap<String, String>,

    pub created: u64,

    pub updated: u64,
}

impl AdminRoleOut {
    pub fn new(
        id: String,
        description: String,
        rules: Vec<AccessRule>,
        policies: Vec<String>,
        context: std::collections::HashMap<String, String>,
        created: u64,
        updated: u64,
    ) -> Self {
        Self {
            id,
            description,
            rules,
            policies,
            context,
            created,
            updated,
        }
    }
}
