use diom_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use diom_error::Error;

use crate::{
    State,
    entities::{ConsumerGroup, Offset, TopicPartition},
    tables::{StreamLeaseRow, TableRow, TopicRow},
};

use super::{MsgsRaftState, MsgsRequest, StreamCommitResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicPartition,
    consumer_group: ConsumerGroup,
    offset: Offset,
    now: Timestamp,
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
            now: Timestamp::now(),
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> diom_operations::Result<StreamCommitResponseData> {
        let mut batch = state.db.batch();
        let topic = self.topic;

        let topic_row = TopicRow::fetch(&state.metadata_tables, self.namespace_id, &topic.raw)?
            .ok_or_else(|| Error::invalid_user_input("partition must exist"))?;

        let mut lease = StreamLeaseRow::fetch(
            &state.metadata_tables,
            topic_row.id,
            topic.partition,
            &self.consumer_group,
        )?
        .ok_or_else(|| Error::invalid_user_input("lease not found"))?;

        lease.offset = self.offset + 1;
        lease.expiry = Timestamp::MIN;

        batch.insert(
            &state.metadata_tables,
            StreamLeaseRow::construct_key(topic_row.id, topic.partition, &self.consumer_group),
            lease.to_fjall_value()?,
        );

        batch.commit().map_err(Error::from)?;

        Ok(StreamCommitResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitResponseData {}

impl MsgsRequest for StreamCommitOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> StreamCommitResponse {
        StreamCommitResponse(self.apply_real(state.msgs))
    }
}
