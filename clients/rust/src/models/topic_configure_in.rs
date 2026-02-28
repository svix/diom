// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TopicConfigureIn {
    pub name: String,

    pub partitions: u16,

    pub topic: String,
}

impl TopicConfigureIn {
    pub fn new(name: String, partitions: u16, topic: String) -> Self {
        Self {
            name,
            partitions,
            topic,
        }
    }
}
