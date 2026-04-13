// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenRotateOut {
    pub id: String,

    pub token: String,

    pub created: u64,

    pub updated: u64,
}

impl AdminAuthTokenRotateOut {
    pub fn new(id: String, token: String, created: u64, updated: u64) -> Self {
        Self {
            id,
            token,
            created,
            updated,
        }
    }
}
