use std::time::Duration;

use diom_core::{PersistableValue, task::spawn_blocking_in_current_span};
use diom_error::{Error, Result};
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, Partition, TopicName},
    tables::{
        MsgKey, MsgRow, QueueConfigKey, QueueConfigRow, QueueLeaseKey, QueueLeaseRow, TopicKey,
        TopicRow,
    },
};

use super::super::{MsgsRaftState, MsgsRequest, QueueNackResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct QueueNackOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    msg_ids: Vec<MsgId>,
    dlq_topic_id_random_bytes: UuidV7RandomBytes,
}

impl QueueNackOperation {
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
            dlq_topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn apply_real(self, state: &State, now: Timestamp) -> Result<QueueNackResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let nack_count = self.msg_ids.len() as u64;
            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
            )?
            .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

            let config = QueueConfigRow::fetch(
                &state.metadata_tables,
                QueueConfigKey::build_key(&topic_row.id, &self.consumer_group),
            )?;

            let retry_schedule = config
                .as_ref()
                .map(|c| c.retry_schedule.as_slice())
                .unwrap_or_default();

            let mut batch = state.db.batch();
            let mut retried_count: u64 = 0;
            let mut dlq_count: u64 = 0;

            for msg_id in &self.msg_ids {
                let existing = QueueLeaseRow::fetch(
                    &state.metadata_tables,
                    QueueLeaseKey::build_key(
                        &topic_row.id,
                        &msg_id.partition,
                        &msg_id.offset,
                        &self.consumer_group,
                    ),
                )?;
                let attempt_count = existing.map(|r| r.attempt_count).unwrap_or(0);

                if let Some(&delay_ms) = retry_schedule.get(attempt_count as usize) {
                    // Schedule for retry: set expiry to now + delay, message becomes available after
                    let expiry = now + Duration::from_millis(delay_ms);
                    batch.insert_row(
                        &state.metadata_tables,
                        QueueLeaseKey::build_key(
                            &topic_row.id,
                            &msg_id.partition,
                            &msg_id.offset,
                            &self.consumer_group,
                        ),
                        &QueueLeaseRow {
                            expiry,
                            dlq: false,
                            attempt_count: attempt_count + 1,
                        },
                    )?;
                    retried_count += 1;
                } else if let Some(dlq_topic) = config.as_ref().and_then(|c| c.dlq_topic.as_ref()) {
                    // Retries exhausted, forward to DLQ topic
                    forward_to_dlq(
                        &state,
                        &mut batch,
                        self.namespace_id,
                        topic_row.id,
                        msg_id,
                        dlq_topic,
                        &self.consumer_group,
                        now,
                        self.dlq_topic_id_random_bytes,
                    )?;
                    dlq_count += 1;
                } else {
                    // No config or no DLQ topic — immediate DLQ
                    batch.insert_row(
                        &state.metadata_tables,
                        QueueLeaseKey::build_key(
                            &topic_row.id,
                            &msg_id.partition,
                            &msg_id.offset,
                            &self.consumer_group,
                        ),
                        &QueueLeaseRow::dlq_marker(attempt_count),
                    )?;
                    dlq_count += 1;
                }
            }

            batch.commit().map_err(Error::from)?;

            state.metrics.record_queue_nacked(
                &self.topic,
                &self.consumer_group,
                nack_count,
                retried_count,
                dlq_count,
            );
            Ok(QueueNackResponseData {})
        })
        .await?
    }
}

/// Copies a message to the DLQ topic and acks the original.
#[allow(clippy::too_many_arguments)]
fn forward_to_dlq(
    state: &State,
    batch: &mut fjall::OwnedWriteBatch,
    namespace_id: NamespaceId,
    source_topic_id: TopicId,
    msg_id: &MsgId,
    dlq_topic: &TopicName,
    consumer_group: &ConsumerGroup,
    now: Timestamp,
    dlq_topic_id_random_bytes: UuidV7RandomBytes,
) -> Result<()> {
    let original = MsgRow::fetch(
        &state.msg_table,
        MsgKey {
            topic_id: source_topic_id,
            partition: msg_id.partition,
            offset: msg_id.offset,
        },
    )?
    .ok_or_else(|| Error::internal("nacked message not found"))?;

    let dlq_topic_row = TopicRow::fetch_or_create(
        &state.metadata_tables,
        batch,
        namespace_id,
        dlq_topic,
        now,
        dlq_topic_id_random_bytes,
    )?;

    // Route deterministically: use source partition clamped to DLQ partition count
    let dlq_partition = Partition::new(msg_id.partition.get() % dlq_topic_row.partitions)?;
    let dlq_offset = MsgRow::next_offset(&state.msg_table, dlq_topic_row.id, dlq_partition)?;

    batch.insert_row(
        &state.msg_table,
        MsgKey {
            topic_id: dlq_topic_row.id,
            partition: dlq_partition,
            offset: dlq_offset,
        },
        &MsgRow {
            value: original.value,
            headers: original.headers,
            timestamp: original.timestamp,
            scheduled_at: None,
        },
    )?;

    // Ack to prevent re-delivery
    QueueLeaseRow::write_ack(
        batch,
        &state.metadata_tables,
        source_topic_id,
        msg_id,
        consumer_group,
    )?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueNackResponseData {}

impl MsgsRequest for QueueNackOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> QueueNackResponse {
        QueueNackResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
