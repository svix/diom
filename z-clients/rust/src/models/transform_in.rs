// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformIn {
    /// JSON-encoded payload passed to the script as `input`.
    pub input: String,

    /// JavaScript source. Must define a `handler(input)` function.
    pub script: String,

    /// How long to let the script run before being killed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_ms: Option<u64>,
}

impl TransformIn {
    pub fn new(input: String, script: String) -> Self {
        Self {
            input,
            script,
            max_duration_ms: None,
        }
    }

    pub fn with_max_duration_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.max_duration_ms = value.into();
        self
    }
}
