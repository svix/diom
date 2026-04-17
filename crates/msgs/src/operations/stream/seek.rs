use diom_core::{PersistableValue, task::spawn_blocking_in_current_span, types::UnixTimestampMs};
use diom_error::{Error, Result};
use diom_id::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Offset, Partition, SeekPosition, TopicIn, TopicName},
    storage::{MsgRow, StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamSeekResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub enum SeekTarget {
    Offset(Offset),
    Position(SeekPosition),
    Timestamp(UnixTimestampMs),
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
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
    ) -> Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.topic, Some(tp.partition)),
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
    async fn apply_real(self, state: &State) -> Result<StreamSeekResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let mut batch = state.db.batch();

            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
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
                    SeekTarget::Timestamp(ts) => MsgRow::first_offset_at_or_after(
                        &state.msg_table,
                        topic_row.id,
                        partition,
                        *ts,
                    )?,
                };

                let lease = StreamLeaseRow {
                    offset,
                    expiry: UnixTimestampMs::UNIX_EPOCH,
                    end_offset: 0,
                };

                batch.insert_row(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                    &lease,
                )?;
            }

            batch.commit().map_err(Error::from)?;

            state
                .metrics
                .record_stream_seek(&self.topic, &self.consumer_group);
            Ok(StreamSeekResponseData {})
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSeekResponseData {}

impl MsgsRequest for StreamSeekOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> StreamSeekResponse {
        StreamSeekResponse::new(self.apply_real(state.msgs).await)
    }
}
