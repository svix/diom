use coyote_error::Error;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Partition, TopicName},
    tables::{QueueLeaseRow, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueRedriveDlqResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRedriveDlqOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
}

impl QueueRedriveDlqOperation {
    pub fn new(namespace_id: NamespaceId, topic: TopicName, consumer_group: ConsumerGroup) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> coyote_operations::Result<QueueRedriveDlqResponseData> {
        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )?
        .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

        let mut batch = state.db.batch();

        for partition_idx in 0..topic_row.partitions {
            let partition = Partition::new(partition_idx)?;

            let Some(mut cursor) = StreamLeaseRow::fetch(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
            )?
            else {
                continue;
            };

            let leases = QueueLeaseRow::scan_partition(
                &state.metadata_tables,
                topic_row.id,
                partition,
                &self.consumer_group,
            )?;

            let mut min_redriven: Option<u64> = None;
            for (msg_id, lease) in &leases {
                if lease.is_dlq() {
                    batch.remove(
                        &state.metadata_tables,
                        QueueLeaseRow::key_for(topic_row.id, msg_id, &self.consumer_group)
                            .into_fjall_key(),
                    );
                    min_redriven =
                        Some(min_redriven.map_or(msg_id.offset, |m| m.min(msg_id.offset)));
                }
            }

            // Reset cursor to the earliest redriven offset so receive scans them again.
            if let Some(new_offset) = min_redriven {
                cursor.offset = new_offset;
                batch.insert_row(
                    &state.metadata_tables,
                    StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
                    &cursor,
                )?;
            }
        }

        batch.commit().map_err(Error::from)?;

        Ok(QueueRedriveDlqResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRedriveDlqResponseData {}

impl MsgsRequest for QueueRedriveDlqOperation {
    fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &coyote_operations::OpContext,
    ) -> QueueRedriveDlqResponse {
        QueueRedriveDlqResponse(self.apply_real(state.msgs))
    }
}
