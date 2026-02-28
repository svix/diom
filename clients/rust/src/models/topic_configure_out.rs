// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopicConfigureOut {
    pub partitions: u16,
}

impl TopicConfigureOut {
    pub fn new(partitions: u16) -> Self {
        Self { partitions }
    }
}
