// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetGroupIn {
    pub name: String,
}

impl KvGetGroupIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
