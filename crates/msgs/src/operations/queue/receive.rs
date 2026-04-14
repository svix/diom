use std::{collections::HashMap, num::NonZeroU16};

use diom_core::{
    PersistableValue,
    task::spawn_blocking_in_current_span,
    types::{ByteString, DurationMs},
};
use diom_error::{Error, Result};
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, Partition, TopicIn, TopicName},
    tables::{MsgRow, QueueLeaseKey, QueueLeaseRow, StreamLeaseKey, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueReceiveResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct QueueReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    batch_size: NonZeroU16,
    #[serde(rename = "lease_duration_ms")]
    lease_duration: DurationMs,
    topic_id_random_bytes: UuidV7RandomBytes,
}

impl QueueReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration: DurationMs,
    ) -> Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.topic, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };
        Ok(Self {
            namespace_id,
            topic,
            partition,
            consumer_group,
            batch_size,
            lease_duration,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size))]
    async fn apply_real(self, state: &State, now: Timestamp) -> Result<QueueReceiveResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let mut remaining = self.batch_size.get();
            let mut all_msgs: Vec<QueueReceiveMsg> = Vec::with_capacity(remaining.into());

            let expiry = now + self.lease_duration;

            let mut batch = state.db.batch();

            let topic_row = TopicRow::fetch_or_create(
                &state.metadata_tables,
                &mut batch,
                self.namespace_id,
                &self.topic,
                now,
                self.topic_id_random_bytes,
            )?;

            Span::current().record("partition_count", topic_row.partitions);

            let partitions = self
                .partition
                .map(|p| vec![p.get()])
                .unwrap_or_else(|| topic_row.partitions_shuffled());

            for partition_idx in partitions {
                let partition = Partition::new(partition_idx)?;

                // Fetch or create cursor for this partition.
                // Queue starts from offset 0 (earliest), unlike stream which starts from latest.
                let mut cursor = match StreamLeaseRow::fetch(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                )? {
                    Some(cursor) => cursor,
                    None => StreamLeaseRow::new()?,
                };

                let mut scan_offset = cursor.offset;

                // Scan messages from cursor, skipping leased and acked ones
                loop {
                    if remaining == 0 {
                        break;
                    }

                    let msgs = MsgRow::fetch_range(
                        &state.msg_table,
                        topic_row.id,
                        partition,
                        scan_offset,
                        remaining,
                    )?;

                    if msgs.is_empty() {
                        break;
                    }

                    let msgs_len = msgs.len() as u64;

                    let n = lease_available_msgs(
                        &state,
                        &mut batch,
                        &mut all_msgs,
                        msgs,
                        scan_offset,
                        partition,
                        topic_row.id,
                        &self.consumer_group,
                        now,
                        expiry,
                    )?;
                    remaining = remaining.saturating_sub(n);

                    scan_offset += msgs_len;
                }

                // Compact cursor: advance past contiguous acked messages
                compact_cursor(
                    &mut cursor,
                    &mut batch,
                    &state,
                    topic_row.id,
                    partition,
                    &self.consumer_group,
                )?;

                batch.insert_row(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                    &cursor,
                )?;

                if remaining == 0 {
                    break;
                }
            }

            batch.commit().map_err(Error::from)?;

            Span::current().record("msgs_returned", all_msgs.len());
            state.metrics.record_queue_received(
                &self.topic,
                &self.consumer_group,
                all_msgs.len() as u64,
            );
            Ok(QueueReceiveResponseData { msgs: all_msgs })
        })
        .await?
    }
}

/// Processes a batch of fetched messages, leasing any that are available.
#[allow(clippy::too_many_arguments)]
fn lease_available_msgs(
    state: &State,
    batch: &mut fjall::OwnedWriteBatch,
    all_msgs: &mut Vec<QueueReceiveMsg>,
    msgs: Vec<MsgRow>,
    scan_offset: u64,
    partition: Partition,
    topic_id: TopicId,
    consumer_group: &ConsumerGroup,
    now: Timestamp,
    expiry: Timestamp,
) -> Result<u16> {
    let mut count = 0;

    for (i, msg) in msgs.into_iter().enumerate() {
        let offset = scan_offset + i as u64;
        let msg_id = MsgId::new(partition, offset);

        let existing_lease = QueueLeaseRow::fetch(
            &state.metadata_tables,
            QueueLeaseKey::build_key(&topic_id, &msg_id.partition, &msg_id.offset, consumer_group),
        )?;

        if existing_lease
            .as_ref()
            .is_some_and(|l| !l.is_available(now))
        {
            continue;
        }

        // If the message is scheduled for future delivery, write a synthetic lease with
        // expiry = scheduled_at so subsequent scans skip it via the lease table without
        // re-checking the message body.
        if let Some(scheduled_at) = msg.scheduled_at
            && scheduled_at > now
        {
            batch.insert_row(
                &state.metadata_tables,
                QueueLeaseKey::build_key(
                    &topic_id,
                    &msg_id.partition,
                    &msg_id.offset,
                    consumer_group,
                ),
                &QueueLeaseRow {
                    expiry: scheduled_at,
                    dlq: false,
                    attempt_count: 0,
                },
            )?;
            continue;
        }

        // Preserve retry_count from previous lease so nack retries are tracked correctly
        let attempt_count = existing_lease.map(|l| l.attempt_count).unwrap_or(0);

        batch.insert_row(
            &state.metadata_tables,
            QueueLeaseKey::build_key(&topic_id, &msg_id.partition, &msg_id.offset, consumer_group),
            &QueueLeaseRow {
                expiry,
                dlq: false,
                attempt_count,
            },
        )?;

        all_msgs.push(QueueReceiveMsg {
            msg_id,
            value: msg.value,
            headers: msg.headers,
            timestamp: msg.timestamp,
            scheduled_at: msg.scheduled_at,
        });

        count += 1;
    }

    Ok(count)
}

/// Advances the cursor past contiguous acked messages, deleting their
/// [`QueueLeaseRow`] entries to prevent unbounded growth.
pub(crate) fn compact_cursor(
    cursor: &mut StreamLeaseRow,
    batch: &mut fjall::OwnedWriteBatch,
    state: &State,
    topic_id: TopicId,
    partition: Partition,
    consumer_group: &ConsumerGroup,
) -> Result<()> {
    loop {
        let check_id = MsgId::new(partition, cursor.offset);
        let key = QueueLeaseKey::build_key(
            &topic_id,
            &check_id.partition,
            &check_id.offset,
            consumer_group,
        );
        match QueueLeaseRow::fetch(&state.metadata_tables, key)? {
            Some(lease) if lease.is_acked() => {
                batch.remove_row::<QueueLeaseRow, _>(
                    &state.metadata_tables,
                    QueueLeaseKey::build_key(
                        &topic_id,
                        &check_id.partition,
                        &check_id.offset,
                        consumer_group,
                    ),
                )?;
                cursor.offset += 1;
            }
            // Advance past DLQ'd messages but keep their rows for redrive
            Some(lease) if lease.is_dlq() => {
                cursor.offset += 1;
            }
            _ => break,
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueReceiveMsg {
    pub msg_id: MsgId,
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
    pub scheduled_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueReceiveResponseData {
    pub msgs: Vec<QueueReceiveMsg>,
}

impl MsgsRequest for QueueReceiveOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> QueueReceiveResponse {
        QueueReceiveResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
