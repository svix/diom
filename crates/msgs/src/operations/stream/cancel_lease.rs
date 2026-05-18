use diom_core::{PersistableValue, task::spawn_blocking_in_current_span, types::UnixTimestampMs};
use diom_error::{Error, OptionExt as _, Result};
use diom_id::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, TopicPartition},
    storage::{StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamCancelLeaseResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct StreamCancelLeaseOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicPartition,
    consumer_group: ConsumerGroup,
}

impl StreamCancelLeaseOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicPartition,
        consumer_group: ConsumerGroup,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<StreamCancelLeaseResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic.topic),
            )?
            .ok_or_not_found("topic")?;

            let key = StreamLeaseKey::build_key(
                &topic_row.id,
                &self.topic.partition,
                &self.consumer_group,
            );

            let lease = StreamLeaseRow::fetch(&state.metadata_tables, key.clone())?
                .filter(|l| l.expiry > now)
                .ok_or_else(|| {
                    Error::bad_request("behavior-error", "no active lease for partition")
                })?;

            let mut updated = lease;
            updated.expiry = UnixTimestampMs::UNIX_EPOCH;

            let mut batch = state.db.batch();
            batch.insert_row(&state.metadata_tables, key, &updated)?;
            batch.commit().map_err(Error::from)?;

            state
                .metrics
                .record_stream_lease_cancelled(&self.topic.topic, &self.consumer_group);
            Ok(StreamCancelLeaseResponseData {})
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCancelLeaseResponseData {}

impl MsgsRequest for StreamCancelLeaseOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> StreamCancelLeaseResponse {
        StreamCancelLeaseResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
