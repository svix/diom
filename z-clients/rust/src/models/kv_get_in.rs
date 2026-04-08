// this file is @generated
use serde::{Deserialize, Serialize};

use super::consistency::Consistency;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KvGetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_postgres: bool,
}

impl KvGetIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            consistency: None,
            use_postgres: false,
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

    pub fn with_use_postgres(mut self, value: bool) -> Self {
        self.use_postgres = value;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvGetIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub use_postgres: bool,
}
