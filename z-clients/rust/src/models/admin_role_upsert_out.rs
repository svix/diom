// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleUpsertOut {
    pub id: String,

    pub created: u64,

    pub updated: u64,
}

impl AdminRoleUpsertOut {
    pub fn new(id: String, created: u64, updated: u64) -> Self {
        Self {
            id,
            created,
            updated,
        }
    }
}
