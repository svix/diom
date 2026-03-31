// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenExpireIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub id: String,

    /// Milliseconds from now until the token expires. `None` means expire immediately.
    #[serde(
        rename = "expiry_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub expiry: Option<std::time::Duration>,
}

impl AuthTokenExpireIn {
    pub fn new(id: String) -> Self {
        Self {
            namespace: None,
            id,
            expiry: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_expiry(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.expiry = value.into();
        self
    }
}
