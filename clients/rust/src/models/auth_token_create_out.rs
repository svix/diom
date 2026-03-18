// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateOut {
    pub id: String,

    pub created_at: jiff::Timestamp,

    pub updated_at: jiff::Timestamp,

    pub token: String,
}

impl AuthTokenCreateOut {
    pub fn new(
        id: String,
        created_at: jiff::Timestamp,
        updated_at: jiff::Timestamp,
        token: String,
    ) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            token,
        }
    }
}
