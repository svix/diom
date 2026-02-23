// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetStreamIn {
    pub name: String,
}

impl GetStreamIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
