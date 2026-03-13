use coyote_error::Error;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, TopicName},
    tables::{QueueLeaseRow, StreamLeaseRow, TopicRow},
};

use super::{
    super::{MsgsRaftState, MsgsRequest, QueueAckResponse},
    receive::compact_cursor,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueAckOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    msg_ids: Vec<MsgId>,
}

impl QueueAckOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        consumer_group: ConsumerGroup,
        msg_ids: Vec<MsgId>,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            msg_ids,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> coyote_operations::Result<QueueAckResponseData> {
        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )?
        .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

        let mut batch = state.db.batch();

        // Track which partitions were affected for cursor compaction
        let mut affected_partitions = std::collections::BTreeSet::new();

        for msg_id in &self.msg_ids {
            QueueLeaseRow::write_ack(
                &mut batch,
                &state.metadata_tables,
                topic_row.id,
                msg_id,
                &self.consumer_group,
            )?;
            affected_partitions.insert(msg_id.partition);
        }

        // Compact cursor for each affected partition
        for partition in affected_partitions {
            let Some(mut cursor) = StreamLeaseRow::fetch(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
            )?
            else {
                continue;
            };

            compact_cursor(
                &mut cursor,
                &mut batch,
                state,
                topic_row.id,
                partition,
                &self.consumer_group,
            )?;

            batch.insert_row(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
                &cursor,
            )?;
        }

        batch.commit().map_err(Error::from)?;

        Ok(QueueAckResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueAckResponseData {}

impl MsgsRequest for QueueAckOperation {
    fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &coyote_operations::OpContext,
    ) -> QueueAckResponse {
        QueueAckResponse(self.apply_real(state.msgs))
    }
}
