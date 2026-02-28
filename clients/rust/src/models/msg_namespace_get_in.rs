// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgNamespaceGetIn {
    pub name: String,
}

impl MsgNamespaceGetIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
