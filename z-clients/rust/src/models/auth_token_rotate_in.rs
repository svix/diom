// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenRotateIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// Milliseconds from now until the old token expires. `None` means expire immediately.
    #[serde(rename = "expiry_ms", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<std::time::Duration>,
}

impl AuthTokenRotateIn {
    pub fn new(id: String) -> Self {
        Self {
            namespace: None,
            id,
            prefix: None,
            suffix: None,
            expiry: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_prefix(mut self, value: impl Into<Option<String>>) -> Self {
        self.prefix = value.into();
        self
    }

    pub fn with_suffix(mut self, value: impl Into<Option<String>>) -> Self {
        self.suffix = value.into();
        self
    }

    pub fn with_expiry(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.expiry = value.into();
        self
    }
}
