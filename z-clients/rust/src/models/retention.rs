// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Retention {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

impl Retention {
    pub fn new() -> Self {
        Self {
            period_ms: None,
            size_bytes: None,
        }
    }

    pub fn with_period_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.period_ms = value.into();
        self
    }

    pub fn with_size_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.size_bytes = value.into();
        self
    }
}
