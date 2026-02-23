// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetGroupIn {
    pub name: String,
}

impl CacheGetGroupIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
