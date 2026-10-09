// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_topic_partition_describe_out::MsgTopicPartitionDescribeOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicDescribeOut {
    /// The unique internal ID of this topic
    ///
    /// This can useful for debugging
    pub id: String,

    pub name: String,

    pub partitions: Vec<MsgTopicPartitionDescribeOut>,
}

impl MsgTopicDescribeOut {
    pub fn new(id: String, name: String, partitions: Vec<MsgTopicPartitionDescribeOut>) -> Self {
        Self {
            id,
            name,
            partitions,
        }
    }
}
