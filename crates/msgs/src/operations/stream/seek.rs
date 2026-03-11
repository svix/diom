use diom_error::Error;
use diom_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Offset, Partition, SeekPosition, TopicIn, TopicName},
    tables::{MsgRow, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamSeekResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeekTarget {
    Offset(Offset),
    Position(SeekPosition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSeekOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    target: SeekTarget,
}

impl StreamSeekOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        target: SeekTarget,
    ) -> diom_error::Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.raw, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };

        if matches!(target, SeekTarget::Offset(_)) && partition.is_none() {
            return Err(Error::invalid_user_input(
                "offset-based seek requires a partition-level topic (e.g. topic~0)",
            ));
        }

        Ok(Self {
            namespace_id,
            topic,
            partition,
            consumer_group,
            target,
        })
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> diom_operations::Result<StreamSeekResponseData> {
        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )?
        .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

        let partitions = self
            .partition
            .map(|p| vec![p.get()])
            .unwrap_or_else(|| (0..topic_row.partitions).collect());

        for partition_idx in partitions {
            let partition = Partition::new(partition_idx)?;

            let offset = match &self.target {
                SeekTarget::Position(SeekPosition::Earliest) => 0,
                SeekTarget::Position(SeekPosition::Latest) => {
                    MsgRow::next_offset(&state.msg_table, topic_row.id, partition)?
                }
                SeekTarget::Offset(o) => *o,
            };

            let lease = StreamLeaseRow {
                offset,
                expiry: Timestamp::UNIX_EPOCH,
                end_offset: 0,
            };

            batch.insert_row(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
                &lease,
            )?;
        }

        batch.commit().map_err(Error::from)?;

        Ok(StreamSeekResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSeekResponseData {}

impl MsgsRequest for StreamSeekOperation {
    fn apply(self, state: MsgsRaftState<'_>, _timestamp: Timestamp) -> StreamSeekResponse {
        StreamSeekResponse(self.apply_real(state.msgs))
    }
}
