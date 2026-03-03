// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct KvGetIn {}

impl KvGetIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvGetIn_ {
    pub key: String,
}
