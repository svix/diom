use std::{collections::HashMap, num::NonZeroU16, time::Duration};

use diom_error::Error;
use diom_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, Partition, TopicId, TopicIn, TopicName},
    tables::{MsgRow, QueueLeaseRow, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueReceiveResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    batch_size: NonZeroU16,
    lease_duration_millis: u64,
}

impl QueueReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration_millis: u64,
    ) -> diom_error::Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.raw, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };
        Ok(Self {
            namespace_id,
            topic,
            partition,
            consumer_group,
            batch_size,
            lease_duration_millis,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size))]
    fn apply_real(
        self,
        state: &State,
        now: Timestamp,
    ) -> diom_operations::Result<QueueReceiveResponseData> {
        let lease_duration = Duration::from_millis(self.lease_duration_millis);
        let mut remaining = self.batch_size.get();
        let mut all_msgs: Vec<QueueReceiveMsg> = Vec::with_capacity(remaining.into());

        let expiry = now + lease_duration;

        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch_or_create(
            &state.metadata_tables,
            &mut batch,
            self.namespace_id,
            &self.topic,
            now,
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
                StreamLeaseRow::key_for(topic_row.id, partition, &self.consumer_group),
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
                    state,
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

            if remaining == 0 {
                break;
            }
        }

        batch.commit().map_err(Error::from)?;

        Span::current().record("msgs_returned", all_msgs.len());

        Ok(QueueReceiveResponseData { msgs: all_msgs })
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
) -> diom_error::Result<u16> {
    let mut count = 0;

    for (i, msg) in msgs.into_iter().enumerate() {
        let offset = scan_offset + i as u64;
        let msg_id = MsgId::new(partition, offset);

        if let Some(lease) = QueueLeaseRow::fetch(
            &state.metadata_tables,
            QueueLeaseRow::key_for(topic_id, &msg_id, consumer_group),
        )? && !lease.is_available(now)
        {
            continue;
        }

        batch.insert_row(
            &state.metadata_tables,
            QueueLeaseRow::key_for(topic_id, &msg_id, consumer_group),
            &QueueLeaseRow { expiry },
        )?;

        all_msgs.push(QueueReceiveMsg {
            msg_id,
            value: msg.value,
            headers: msg.headers,
            timestamp: msg.timestamp,
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
) -> diom_error::Result<()> {
    loop {
        let check_id = MsgId::new(partition, cursor.offset);
        match QueueLeaseRow::fetch(
            &state.metadata_tables,
            QueueLeaseRow::key_for(topic_id, &check_id, consumer_group),
        )? {
            Some(lease) if lease.is_acked() => {
                batch.remove(
                    &state.metadata_tables,
                    QueueLeaseRow::key_for(topic_id, &check_id, consumer_group).into_fjall_key(),
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
    fn apply(self, state: MsgsRaftState<'_>, now: Timestamp) -> QueueReceiveResponse {
        QueueReceiveResponse(self.apply_real(state.msgs, now))
    }
}
