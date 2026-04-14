use diom_core::task::spawn_blocking_in_current_span;
use diom_error::{Error, Result};
use diom_id::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Partition, TopicName},
    tables::{QueueLeaseKey, QueueLeaseRow, StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow},
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
    async fn apply_real(self, state: &State) -> Result<QueueRedriveDlqResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
            )?
            .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

            let mut batch = state.db.batch();
            let mut total_redriven: u64 = 0;

            for partition_idx in 0..topic_row.partitions {
                let partition = Partition::new(partition_idx)?;

                let Some(mut cursor) = StreamLeaseRow::fetch(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
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
                let mut partition_redriven: u64 = 0;
                for (msg_id, lease) in &leases {
                    if lease.is_dlq() {
                        batch.remove_row::<QueueLeaseRow, _>(
                            &state.metadata_tables,
                            QueueLeaseKey::build_key(
                                &topic_row.id,
                                &msg_id.partition,
                                &msg_id.offset,
                                &self.consumer_group,
                            ),
                        )?;
                        min_redriven =
                            Some(min_redriven.map_or(msg_id.offset, |m| m.min(msg_id.offset)));
                        partition_redriven += 1;
                    }
                }

                // Reset cursor to the earliest redriven offset so receive scans them again.
                if let Some(new_offset) = min_redriven {
                    cursor.offset = new_offset;
                    batch.insert_row(
                        &state.metadata_tables,
                        StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                        &cursor,
                    )?;
                }

                total_redriven += partition_redriven;
            }

            batch.commit().map_err(Error::from)?;

            state
                .metrics
                .record_queue_redrive(&self.topic, &self.consumer_group, total_redriven);
            Ok(QueueRedriveDlqResponseData {})
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRedriveDlqResponseData {}

impl MsgsRequest for QueueRedriveDlqOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> QueueRedriveDlqResponse {
        QueueRedriveDlqResponse::new(self.apply_real(state.msgs).await)
    }
}
