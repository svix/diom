// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateStreamIn {
    /// How many bytes in total the stream will retain before dropping data.
    #[serde(rename = "maxByteSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_byte_size: Option<u64>,

    pub name: String,

    /// How long messages are retained in the stream before being permanently nuked.
    #[serde(rename = "retentionPeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_seconds: Option<u64>,
}

impl CreateStreamIn {
    pub fn new(name: String) -> Self {
        Self {
            max_byte_size: None,
            name,
            retention_period_seconds: None,
        }
    }
}
