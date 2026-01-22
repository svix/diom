// this file is @generated
use serde::{Deserialize, Serialize};

use super::operation_behavior::OperationBehavior;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,

    /// Time to live in milliseconds
    pub expire_in: u64,

    pub key: String,

    pub value: String,
}

impl KvSetIn {
    pub fn new(expire_in: u64, key: String, value: String) -> Self {
        Self {
            behavior: None,
            expire_in,
            key,
            value,
        }
    }
}
