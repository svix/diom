// this file is @generated
use serde::{Deserialize, Serialize};

use super::retention::Retention;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgNamespaceCreateOut {
    pub name: String,

    pub retention: Retention,

    pub created: u64,

    pub updated: u64,
}

impl MsgNamespaceCreateOut {
    pub fn new(name: String, retention: Retention, created: u64, updated: u64) -> Self {
        Self {
            name,
            retention,
            created,
            updated,
        }
    }
}
