// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformOut {
    /// JSON-encoded value returned by the script's `handler` function.
    pub output: String,
}

impl TransformOut {
    pub fn new(output: String) -> Self {
        Self { output }
    }
}
