use diom_core::{PersistableValue, task::spawn_blocking_in_current_span};
use diom_error::{Error, Result};
use diom_id::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Offset, TopicPartition},
    tables::{StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamCommitResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
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
    async fn apply_real(self, state: &State) -> Result<StreamCommitResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let mut batch = state.db.batch();
            let topic = self.topic;

            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &topic.topic),
            )?
            .ok_or_else(|| Error::invalid_user_input("partition must exist"))?;

            let mut lease = StreamLeaseRow::fetch(
                &state.metadata_tables,
                StreamLeaseKey::build_key(&topic_row.id, &topic.partition, &self.consumer_group),
            )?
            .ok_or_else(|| Error::invalid_user_input("lease not found"))?;

            lease.offset = self.offset + 1;
            if self.offset >= lease.end_offset {
                lease.expiry = jiff::Timestamp::UNIX_EPOCH;
            }

            batch.insert_row(
                &state.metadata_tables,
                StreamLeaseKey::build_key(&topic_row.id, &topic.partition, &self.consumer_group),
                &lease,
            )?;

            batch.commit().map_err(Error::from)?;

            state
                .metrics
                .record_stream_committed(&topic.topic, &self.consumer_group);
            Ok(StreamCommitResponseData {})
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitResponseData {}

impl MsgsRequest for StreamCommitOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> StreamCommitResponse {
        StreamCommitResponse::new(self.apply_real(state.msgs).await)
    }
}
