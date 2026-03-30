// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Retention {
    #[serde(rename = "period_ms", skip_serializing_if = "Option::is_none")]
    pub period: Option<std::time::Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

impl Retention {
    pub fn new() -> Self {
        Self {
            period: None,
            size_bytes: None,
        }
    }

    pub fn with_period(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.period = value.into();
        self
    }

    pub fn with_size_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.size_bytes = value.into();
        self
    }
}
