use diom_core::types::ByteString;
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use std::collections::HashMap;

use diom_error::Result;
use fjall_utils::{FjallKeyAble, TableKey, TableRow, WriteBatchExt};
use jiff::Timestamp;
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

#[derive(Serialize, Deserialize)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub name: TopicName,
    pub partitions: u16,
}

impl TableRow for TopicRow {
    const ROW_TYPE: u8 = RowType::Topic as u8;
}

impl TopicRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, topic: &TopicName) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[namespace_id.as_bytes()], &[topic])
    }

    pub(crate) fn new(name: TopicName, now: Timestamp, id_random_bytes: UuidV7RandomBytes) -> Self {
        Self {
            id: TopicId::new(now, id_random_bytes),
            name,
            partitions: 1,
        }
    }

    pub(crate) fn partitions_shuffled(&self, rng: &mut impl rand::Rng) -> Vec<u16> {
        use rand::seq::SliceRandom;
        let mut list: Vec<u16> = (0..self.partitions).collect();
        list.shuffle(rng);
        list
    }

    /// Returns the existing row, or creates a new one and inserts it into the batch.
    pub(crate) fn fetch_or_create(
        metadata_tables: &fjall::Keyspace,
        batch: &mut fjall::OwnedWriteBatch,
        namespace_id: NamespaceId,
        topic: &TopicName,
        now: Timestamp,
        id_random_bytes: UuidV7RandomBytes,
    ) -> Result<Self> {
        if let Some(row) = Self::fetch(metadata_tables, Self::key_for(namespace_id, topic))? {
            return Ok(row);
        }
        let row = Self::new(topic.clone(), now, id_random_bytes);
        batch.insert_row(metadata_tables, Self::key_for(namespace_id, topic), &row)?;
        Ok(row)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamLeaseRow {
    pub offset: u64,
    pub expiry: Timestamp,
    /// Last offset in the current leased batch. The lease is only released
    /// when the committed offset reaches this value.
    pub end_offset: Offset,
}

impl StreamLeaseRow {
    const CONSUMER_GROUP_OFFSET: usize =
        size_of::<fjall_utils::TableKeyType>() + size_of::<TopicId>() + size_of::<Partition>();

    pub(crate) fn consumer_group_from_key(key: &[u8]) -> Option<&str> {
        std::str::from_utf8(key.get(Self::CONSUMER_GROUP_OFFSET..)?).ok()
    }

    /// Returns the key prefix for scanning all `StreamLeaseRow`s for a given topic.
    pub(crate) fn topic_scan_prefix(topic_id: TopicId) -> Vec<u8> {
        let mut prefix =
            Vec::with_capacity(size_of::<fjall_utils::TableKeyType>() + size_of::<TopicId>());
        prefix.push(Self::ROW_TYPE);
        prefix.extend_from_slice(topic_id.as_bytes());
        prefix
    }

    pub(crate) fn key_for(
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[topic_id.as_bytes(), &partition.get().to_be_bytes()],
            &[consumer_group],
        )
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            offset: 0,
            expiry: Timestamp::UNIX_EPOCH,
            end_offset: 0,
        })
    }
}

impl TableRow for StreamLeaseRow {
    const ROW_TYPE: u8 = RowType::StreamLease as u8;
}

/// Per-message lease/ack tracking for queue semantics.
///
/// - `expiry > now` → message is leased (in-flight to a consumer)
/// - `expiry == Timestamp::MAX` → message is permanently acked
/// - `expiry <= now` → lease expired, message is available again
/// - No row → message was never leased, available
///
/// Rows below the queue cursor are deleted during cursor compaction to prevent unbounded growth.
#[derive(Serialize, Deserialize)]
pub(crate) struct QueueLeaseRow {
    pub expiry: Timestamp,
    pub dlq: bool,
    pub attempt_count: u32,
}

impl QueueLeaseRow {
    pub(crate) fn key_for(
        topic_id: TopicId,
        msg_id: &MsgId,
        consumer_group: &ConsumerGroup,
    ) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[
                topic_id.as_bytes(),
                &msg_id.partition.get().to_be_bytes(),
                &msg_id.offset.to_be_bytes(),
            ],
            &[consumer_group],
        )
    }

    /// Permanently acked — will never be re-delivered.
    pub(crate) fn acked() -> Self {
        Self {
            expiry: Timestamp::MAX,
            dlq: false,
            attempt_count: 0,
        }
    }

    /// Sent to the dead-letter queue.
    pub(crate) fn dlq_marker(attempt_count: u32) -> Self {
        Self {
            expiry: Timestamp::MAX,
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
            Self::key_for(topic_id, msg_id, consumer_group),
            &Self::acked(),
        )?;
        Ok(())
    }

    pub(crate) fn is_available(&self, now: Timestamp) -> bool {
        !self.dlq && self.expiry <= now
    }

    pub(crate) fn is_acked(&self) -> bool {
        !self.dlq && self.expiry == Timestamp::MAX
    }

    pub(crate) fn is_dlq(&self) -> bool {
        self.dlq
    }

    // FIXME(@svix-gabriel): This manually parses the TableKey byte layout, which is
    // fragile and tightly coupled to `TableKey::init_key`'s encoding. Should be replaced
    // with a proper range/prefix scan API on TableRow.
    /// Returns all lease rows for a given (topic, partition, consumer_group) via prefix scan.
    pub(crate) fn scan_partition(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Result<Vec<(MsgId, Self)>> {
        // Key layout: [ROW_TYPE:1B][topic_id:16B][partition:2B][offset:8B][consumer_group]
        let mut prefix = Vec::with_capacity(1 + 16 + 2);
        prefix.push(Self::ROW_TYPE);
        prefix.extend_from_slice(topic_id.as_bytes());
        prefix.extend_from_slice(&partition.get().to_be_bytes());

        let cg_bytes = consumer_group.as_bytes();
        let mut results = Vec::new();

        for guard in keyspace.prefix(&prefix) {
            let (key, val) = guard.into_inner()?;
            let offset_start = 1 + 16 + 2;
            let offset_end = offset_start + 8;
            if key.len() < offset_end {
                continue;
            }
            let offset_bytes: [u8; 8] = key[offset_start..offset_end]
                .try_into()
                .expect("checked length");
            let offset = u64::from_be_bytes(offset_bytes);

            let key_cg = &key[offset_end..];
            if key_cg != cg_bytes {
                continue;
            }

            let row = Self::from_fjall_value(val)?;
            results.push((MsgId::new(partition, offset), row));
        }

        Ok(results)
    }
}

impl TableRow for QueueLeaseRow {
    const ROW_TYPE: u8 = RowType::QueueLease as u8;
}

/// Per-consumer-group queue configuration
#[derive(Serialize, Deserialize)]
pub(crate) struct QueueConfigRow {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

impl QueueConfigRow {
    pub(crate) fn key_for(topic_id: TopicId, consumer_group: &ConsumerGroup) -> TableKey<Self> {
        TableKey::init_key(Self::ROW_TYPE, &[topic_id.as_bytes()], &[consumer_group])
    }
}

impl TableRow for QueueConfigRow {
    const ROW_TYPE: u8 = RowType::QueueConfig as u8;
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

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
    pub scheduled_at: Option<Timestamp>,
}

impl MsgRow {
    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let range = MsgKey::range(
            MsgKey {
                topic_id,
                partition,
                offset: Offset::MIN,
            }..=MsgKey {
                topic_id,
                partition,
                offset: Offset::MAX,
            },
        );
        let item = keyspace.range(range).next_back();
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

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct IdempotencyRow {
    pub expiry: Timestamp,
}

impl TableRow for IdempotencyRow {
    const ROW_TYPE: u8 = RowType::Idempotency as u8;
}

impl IdempotencyRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, key: &MsgsIdempotencyKey) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[namespace_id.as_bytes(), key.as_bytes()],
            &[],
        )
    }
}

#[cfg(test)]
mod tests {
    use jiff::Timestamp;

    use super::*;
    use crate::entities::{ConsumerGroup, Partition};

    #[test]
    fn test_consumer_group_from_key() {
        use diom_id::TopicId;
        let topic_id = TopicId::new(Timestamp::UNIX_EPOCH, UuidV7RandomBytes::new_random());
        let partition = Partition::new(0).unwrap();
        let cg = ConsumerGroup::try_from("my-group").unwrap();
        let key = StreamLeaseRow::key_for(topic_id, partition, &cg).into_fjall_key();
        assert_eq!(
            StreamLeaseRow::consumer_group_from_key(&key),
            Some("my-group")
        );
    }
}
