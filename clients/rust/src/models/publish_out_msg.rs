// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishOutMsg {
    pub offset: u64,

    pub topic: String,
}

impl PublishOutMsg {
    pub fn new(offset: u64, topic: String) -> Self {
        Self { offset, topic }
    }
}
