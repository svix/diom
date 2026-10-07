// this file is @generated
use serde::{Deserialize, Serialize};

use super::consistency::Consistency;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MsgTopicListIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,

    /// Limit the number of returned items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,
}

impl MsgTopicListIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            consistency: None,
            limit: None,
            iterator: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_consistency(mut self, value: impl Into<Option<Consistency>>) -> Self {
        self.consistency = value.into();
        self
    }

    pub fn with_limit(mut self, value: impl Into<Option<u64>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn with_iterator(mut self, value: impl Into<Option<String>>) -> Self {
        self.iterator = value.into();
        self
    }
}
