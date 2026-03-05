use std::{collections::HashMap, num::NonZeroU16, time::Duration};

use coyote_error::Error;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, Partition, TopicIn, TopicName},
    tables::{MsgRow, QueueLeaseRow, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueReceiveResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    batch_size: NonZeroU16,
    lease_duration_millis: u64,
    now: Timestamp,
}

impl QueueReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        batch_size: NonZeroU16,
        lease_duration_millis: u64,
    ) -> coyote_error::Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.raw, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };
        Ok(Self {
            namespace_id,
            topic,
            partition,
            batch_size,
            lease_duration_millis,
            now: Timestamp::now(),
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size))]
    fn apply_real(self, state: &State) -> coyote_operations::Result<QueueReceiveResponseData> {
        let lease_duration = Duration::from_millis(self.lease_duration_millis);
        let mut remaining = self.batch_size.get() as usize;
        let mut all_msgs: Vec<QueueReceiveMsg> = Vec::with_capacity(remaining);
        let expiry = self.now + lease_duration;
        let queue_cg = ConsumerGroup::queue();

        let mut batch = state.db.batch();

        let topic_row = match TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )? {
            Some(topic_row) => topic_row,
            None => {
                let topic_row = TopicRow::new(self.topic.clone(), self.now);
                batch.insert_row(
                    &state.metadata_tables,
                    TopicRow::key_for(self.namespace_id, &self.topic),
                    &topic_row,
                )?;
                topic_row
            }
        };

        Span::current().record("partition_count", topic_row.partitions);

        let partitions = if let Some(partition) = self.partition {
            vec![partition.get()]
        } else {
            let mut partition_list: Vec<u16> = (0..topic_row.partitions).collect();
            partition_list.shuffle(&mut rand::rng());
            partition_list
        };

        for partition_idx in partitions {
            let partition = Partition::new(partition_idx)?;

            // Fetch or create cursor for this partition (using StreamLeaseRow with __queue__ CG).
            // Queue starts from offset 0 (earliest), unlike stream which starts from latest.
            let mut cursor = match StreamLeaseRow::fetch(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &queue_cg),
            )? {
                Some(cursor) => cursor,
                None => StreamLeaseRow::new()?,
            };

            let mut scan_offset = cursor.offset;

            // Scan messages from cursor, skipping leased and acked ones
            'scan: loop {
                if remaining == 0 {
                    break;
                }

                let fetch_count = (remaining as u16).saturating_add(16);
                let msgs = MsgRow::fetch_range(
                    &state.msg_table,
                    topic_row.id,
                    partition,
                    scan_offset,
                    fetch_count,
                )?;

                if msgs.is_empty() {
                    break;
                }

                let msgs_len = msgs.len() as u64;

                for (i, msg) in msgs.into_iter().enumerate() {
                    let offset = scan_offset + i as u64;
                    let msg_id = MsgId::new(partition, offset);

                    if let Some(lease) = QueueLeaseRow::fetch(
                        &state.metadata_tables,
                        QueueLeaseRow::key_for(topic_row.id, &msg_id),
                    )? && (lease.is_acked() || !lease.is_available(self.now))
                    {
                        continue;
                    }

                    // Available — lease this message
                    batch.insert_row(
                        &state.metadata_tables,
                        QueueLeaseRow::key_for(topic_row.id, &msg_id),
                        &QueueLeaseRow { expiry },
                    )?;

                    all_msgs.push(QueueReceiveMsg {
                        msg_id,
                        value: msg.value,
                        headers: msg.headers,
                        timestamp: msg.timestamp,
                    });

                    remaining -= 1;
                    if remaining == 0 {
                        break 'scan;
                    }
                }

                scan_offset += msgs_len;
            }

            // Compact cursor: advance past contiguous acked messages
            compact_cursor(&mut cursor, &mut batch, state, topic_row.id, partition)?;

            batch.insert_row(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, partition, &queue_cg),
                &cursor,
            )?;

            if remaining == 0 {
                break;
            }
        }

        batch.commit().map_err(Error::from)?;

        Span::current().record("msgs_returned", all_msgs.len());

        Ok(QueueReceiveResponseData { msgs: all_msgs })
    }
}

/// Advances the cursor past contiguous acked messages, deleting their
/// [`QueueLeaseRow`] entries to prevent unbounded growth.
pub(crate) fn compact_cursor(
    cursor: &mut StreamLeaseRow,
    batch: &mut fjall::OwnedWriteBatch,
    state: &State,
    topic_id: crate::entities::TopicId,
    partition: Partition,
) -> coyote_error::Result<()> {
    loop {
        let check_id = MsgId::new(partition, cursor.offset);
        match QueueLeaseRow::fetch(
            &state.metadata_tables,
            QueueLeaseRow::key_for(topic_id, &check_id),
        )? {
            Some(lease) if lease.is_acked() => {
                batch.remove(
                    &state.metadata_tables,
                    QueueLeaseRow::key_for(topic_id, &check_id).into_fjall_key(),
                );
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
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueReceiveResponseData {
    pub msgs: Vec<QueueReceiveMsg>,
}

impl MsgsRequest for QueueReceiveOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> QueueReceiveResponse {
        QueueReceiveResponse(self.apply_real(state.msgs))
    }
}
