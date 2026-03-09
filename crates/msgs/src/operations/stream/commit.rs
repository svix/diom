use diom_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use diom_error::Error;

use crate::{
    State,
    entities::{ConsumerGroup, Offset, TopicPartition},
    tables::{StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamCommitResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicPartition,
    consumer_group: ConsumerGroup,
    offset: Offset,
}

impl StreamCommitOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicPartition,
        consumer_group: ConsumerGroup,
        offset: Offset,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            offset,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> diom_operations::Result<StreamCommitResponseData> {
        let mut batch = state.db.batch();
        let topic = self.topic;

        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &topic.raw),
        )?
        .ok_or_else(|| Error::invalid_user_input("partition must exist"))?;

        let mut lease = StreamLeaseRow::fetch(
            &state.metadata_tables,
            StreamLeaseRow::key_for(topic_row.id, topic.partition, &self.consumer_group),
        )?
        .ok_or_else(|| Error::invalid_user_input("lease not found"))?;

        lease.offset = self.offset + 1;
        if self.offset >= lease.end_offset {
            lease.expiry = Timestamp::MIN;
        }

        batch.insert_row(
            &state.metadata_tables,
            StreamLeaseRow::key_for(topic_row.id, topic.partition, &self.consumer_group),
            &lease,
        )?;

        batch.commit().map_err(Error::from)?;

        Ok(StreamCommitResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitResponseData {}

impl MsgsRequest for StreamCommitOperation {
    fn apply(self, state: MsgsRaftState<'_>, _timestamp: Timestamp) -> StreamCommitResponse {
        StreamCommitResponse(self.apply_real(state.msgs))
    }
}
