// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgIn {
    pub headers: std::collections::HashMap<String, String>,

    pub payload: Vec<u8>,
}

impl MsgIn {
    pub fn new(headers: std::collections::HashMap<String, String>, payload: Vec<u8>) -> Self {
        Self { headers, payload }
    }
}
