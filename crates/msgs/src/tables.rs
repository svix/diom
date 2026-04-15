use diom_core::{
    PersistableValue,
    types::{AsMillisecond, ByteString, UnixTimestampMs},
};
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use std::collections::HashMap;

use diom_error::Result;
use fjall_utils::{FjallKeyAble, TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::entities::{ConsumerGroup, MsgId, MsgsIdempotencyKey, Offset, Partition, TopicName};

#[repr(u8)]
enum RowType {
    Topic = 0,
    StreamLease = 1,
    Msg = 2,
    QueueLease = 3,
    QueueConfig = 4,
    Idempotency = 5,
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub name: TopicName,
    pub partitions: u16,
}

impl TableRow for TopicRow {
    const ROW_TYPE: u8 = RowType::Topic as u8;
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::Topic)]
pub(crate) struct TopicKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) topic: TopicName,
}

impl TopicRow {
    pub(crate) fn new(
        name: TopicName,
        now: impl AsMillisecond,
        id_random_bytes: UuidV7RandomBytes,
    ) -> Self {
        Self {
            id: TopicId::new(now, id_random_bytes),
            name,
            partitions: 1,
        }
    }

    pub(crate) fn partitions_shuffled(&self) -> Vec<u16> {
        use rand::seq::SliceRandom;
        let mut list: Vec<u16> = (0..self.partitions).collect();
        list.shuffle(&mut rand::rng());
        list
    }

    /// Returns the existing row, or creates a new one and inserts it into the batch.
    pub(crate) fn fetch_or_create(
        metadata_tables: &fjall::Keyspace,
        batch: &mut fjall::OwnedWriteBatch,
        namespace_id: NamespaceId,
        topic: &TopicName,
        now: impl AsMillisecond,
        id_random_bytes: UuidV7RandomBytes,
    ) -> Result<Self> {
        let key = TopicKey::build_key(&namespace_id, topic);
        if let Some(row) = Self::fetch(metadata_tables, key.clone())? {
            return Ok(row);
        }
        let row = Self::new(topic.clone(), now, id_random_bytes);
        batch.insert_row(metadata_tables, key, &row)?;
        Ok(row)
    }
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct StreamLeaseRow {
    pub offset: u64,
    pub expiry: UnixTimestampMs,
    /// Last offset in the current leased batch. The lease is only released
    /// when the committed offset reaches this value.
    pub end_offset: Offset,
}

impl StreamLeaseRow {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            offset: 0,
            expiry: UnixTimestampMs::UNIX_EPOCH,
            end_offset: 0,
        })
    }
}

impl TableRow for StreamLeaseRow {
    const ROW_TYPE: u8 = RowType::StreamLease as u8;
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::StreamLease)]
pub(crate) struct StreamLeaseKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) consumer_group: ConsumerGroup,
}

/// Per-message lease/ack tracking for queue semantics.
///
/// - `expiry > now` → message is leased (in-flight to a consumer)
/// - `expiry == UnixTimestampMs::MAX` → message is permanently acked
/// - `expiry <= now` → lease expired, message is available again
/// - No row → message was never leased, available
///
/// Rows below the queue cursor are deleted during cursor compaction to prevent unbounded growth.
#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct QueueLeaseRow {
    pub expiry: UnixTimestampMs,
    pub dlq: bool,
    pub attempt_count: u32,
}

impl QueueLeaseRow {
    /// Permanently acked — will never be re-delivered.
    pub(crate) fn acked() -> Self {
        Self {
            expiry: UnixTimestampMs::MAX,
            dlq: false,
            attempt_count: 0,
        }
    }

    /// Sent to the dead-letter queue.
    pub(crate) fn dlq_marker(attempt_count: u32) -> Self {
        Self {
            expiry: UnixTimestampMs::MAX,
            dlq: true,
            attempt_count,
        }
    }

    /// Writes an ack row into the batch, permanently marking the message as consumed.
    pub(crate) fn write_ack(
        batch: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        msg_id: &MsgId,
        consumer_group: &ConsumerGroup,
    ) -> Result<()> {
        batch.insert_row(
            keyspace,
            QueueLeaseKey::build_key(&topic_id, &msg_id.partition, &msg_id.offset, consumer_group),
            &Self::acked(),
        )?;
        Ok(())
    }

    pub(crate) fn is_available(&self, now: UnixTimestampMs) -> bool {
        !self.dlq && self.expiry <= now
    }

    pub(crate) fn is_acked(&self) -> bool {
        !self.dlq && self.expiry == UnixTimestampMs::MAX
    }

    pub(crate) fn is_dlq(&self) -> bool {
        self.dlq
    }

    /// Returns all lease rows for a given (topic, partition, consumer_group) via prefix scan.
    pub(crate) fn scan_partition(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Result<Vec<(MsgId, Self)>> {
        let prefix = QueueLeaseKey::prefix_partition(&topic_id, &partition);
        let mut results = Vec::new();

        for guard in keyspace.prefix(&prefix) {
            let (key, val) = guard.into_inner()?;
            let cg = QueueLeaseKey::extract_consumer_group(&key)
                .expect("valid QueueLeaseKey in metadata table");
            if cg != *consumer_group {
                continue;
            }
            let offset =
                QueueLeaseKey::extract_offset(&key).expect("valid QueueLeaseKey in metadata table");

            let row = Self::from_fjall_value(val)?;
            results.push((MsgId::new(partition, offset), row));
        }

        Ok(results)
    }
}

impl TableRow for QueueLeaseRow {
    const ROW_TYPE: u8 = RowType::QueueLease as u8;
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::QueueLease)]
pub(crate) struct QueueLeaseKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) offset: Offset,
    #[key(3)]
    pub(crate) consumer_group: ConsumerGroup,
}

/// Per-consumer-group queue configuration
#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct QueueConfigRow {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

impl TableRow for QueueConfigRow {
    const ROW_TYPE: u8 = RowType::QueueConfig as u8;
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::QueueConfig)]
pub(crate) struct QueueConfigKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) consumer_group: ConsumerGroup,
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::Msg)]
pub(crate) struct MsgKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) offset: Offset,
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct MsgRow {
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

impl MsgRow {
    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let range = MsgKey::prefix_partition(&topic_id, &partition);
        let item = keyspace.prefix(range).next_back();
        match item {
            Some(kv) => {
                let key = kv.key()?;
                let offset = MsgKey::extract_offset(&key).expect("valid MsgKey in msg table");
                Ok(offset + 1)
            }
            None => Ok(0),
        }
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size))]
    pub(crate) fn fetch_range(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
        batch_size: u16,
    ) -> Result<Vec<Self>> {
        let mut results = Vec::with_capacity(batch_size as usize);
        let range = MsgKey::range(
            MsgKey {
                topic_id,
                partition,
                offset,
            }..MsgKey {
                topic_id,
                partition,
                offset: offset + batch_size as u64,
            },
        );
        for entry in keyspace.range(range) {
            let val = entry.value()?;
            let msg = Self::from_fjall_value(val)?;
            results.push(msg);
        }

        tracing::Span::current().record("msgs_found", results.len());

        Ok(results)
    }
}

impl TableRow for MsgRow {
    const ROW_TYPE: u8 = RowType::Msg as u8;
}

#[derive(Clone, Serialize, Deserialize, PersistableValue)]
pub(crate) struct IdempotencyRow {
    pub expiry: UnixTimestampMs,
}

impl TableRow for IdempotencyRow {
    const ROW_TYPE: u8 = RowType::Idempotency as u8;
}

#[derive(FjallKeyAble)]
#[table_key(prefix = RowType::Idempotency)]
pub(crate) struct IdempotencyKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) key: MsgsIdempotencyKey,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::entities::{ConsumerGroup, Partition};

    #[test]
    fn test_consumer_group_from_key() {
        use diom_id::TopicId;
        let topic_id = TopicId::new(UnixTimestampMs::UNIX_EPOCH, UuidV7RandomBytes::new_random());
        let partition = Partition::new(0).unwrap();
        let cg = ConsumerGroup::try_from("my-group").unwrap();
        let key = StreamLeaseKey::build_key(&topic_id, &partition, &cg);
        assert_eq!(
            &*StreamLeaseKey::extract_consumer_group(&key).unwrap(),
            "my-group"
        );
    }
}
