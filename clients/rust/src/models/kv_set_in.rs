// this file is @generated
use serde::{Deserialize, Serialize};

use super::operation_behavior::OperationBehavior;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,

    /// Time to live in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_in: Option<u64>,

    pub key: String,

    pub value: Vec<u8>,
}

impl KvSetIn {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            behavior: None,
            expire_in: None,
            key,
            value,
        }
    }
}
