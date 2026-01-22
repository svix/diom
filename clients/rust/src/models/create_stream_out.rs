// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateStreamOut {
    #[serde(rename = "createdAt")]
    pub created_at: jiff::Timestamp,

    pub id: String,

    #[serde(rename = "maxByteSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_byte_size: Option<u64>,

    pub name: String,

    #[serde(rename = "retentionPeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_seconds: Option<u64>,

    #[serde(rename = "updatedAt")]
    pub updated_at: jiff::Timestamp,
}

impl CreateStreamOut {
    pub fn new(
        created_at: jiff::Timestamp,
        id: String,
        name: String,
        updated_at: jiff::Timestamp,
    ) -> Self {
        Self {
            created_at,
            id,
            max_byte_size: None,
            name,
            retention_period_seconds: None,
            updated_at,
        }
    }
}
