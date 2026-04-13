// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenCreateOut {
    pub id: String,

    pub token: String,

    pub created: u64,

    pub updated: u64,
}

impl AdminAuthTokenCreateOut {
    pub fn new(id: String, token: String, created: u64, updated: u64) -> Self {
        Self {
            id,
            token,
            created,
            updated,
        }
    }
}
