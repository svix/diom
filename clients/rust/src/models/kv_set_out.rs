// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetOut {
    /// Whether the operation succeeded or was a noop due to pre-conditions.
    pub success: bool,
}

impl KvSetOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
