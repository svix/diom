// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicPartitionDescribeOut {
    pub partition_id: u16,

    /// The next offset to be committed to this partition
    pub high_water_mark: u64,
}

impl MsgTopicPartitionDescribeOut {
    pub fn new(partition_id: u16, high_water_mark: u64) -> Self {
        Self {
            partition_id,
            high_water_mark,
        }
    }
}
