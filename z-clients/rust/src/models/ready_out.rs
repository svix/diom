// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadyOut {
    pub ok: bool,

    pub message: String,
}

impl ReadyOut {
    pub fn new(ok: bool, message: String) -> Self {
        Self { ok, message }
    }
}
