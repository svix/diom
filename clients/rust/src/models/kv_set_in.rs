// this file is @generated
use serde::{Deserialize, Serialize};

use super::operation_behavior::OperationBehavior;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetIn {
    pub key: String,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,
}

impl KvSetIn {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            key,
            value,
            ttl: None,
            behavior: None,
        }
    }
}
