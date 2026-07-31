// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SinkListIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Limit the number of returned items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,
}

impl SinkListIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            limit: None,
            iterator: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SinkListIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub topic: String,

    /// Limit the number of returned items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,
}
