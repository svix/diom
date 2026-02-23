// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyGetGroupIn {
    pub name: String,
}

impl IdempotencyGetGroupIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
