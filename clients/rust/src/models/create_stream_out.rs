// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateStreamOut {
    pub created_at: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_byte_size: Option<u64>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_seconds: Option<u64>,

    pub updated_at: jiff::Timestamp,
}

impl CreateStreamOut {
    pub fn new(created_at: jiff::Timestamp, name: String, updated_at: jiff::Timestamp) -> Self {
        Self {
            created_at,
            max_byte_size: None,
            name,
            retention_period_seconds: None,
            updated_at,
        }
    }
}
