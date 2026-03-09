use coyote_namespace::entities::NamespaceId;
use std::collections::HashMap;

use coyote_error::{Result, ResultExt as _};
use fjall_utils::{TableKey, TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ConsumerGroup, MsgId, Offset, Partition, TopicId, TopicName};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Topic = 0,
    StreamLease = 1,
    Msg = 2,
    QueueLease = 3,
}

const SIZE_U64: usize = size_of::<u64>();

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

    pub(crate) fn new(name: TopicName, now: Timestamp) -> Self {
        Self {
            id: uuid::Uuid::new_v7(uuid::Timestamp::from_unix(
                uuid::NoContext,
                now.as_second() as u64,
                now.subsec_nanosecond() as u32,
            )),
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
        now: Timestamp,
    ) -> Result<Self> {
        if let Some(row) = Self::fetch(metadata_tables, Self::key_for(namespace_id, topic))? {
            return Ok(row);
        }
        let row = Self::new(topic.clone(), now);
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
            expiry: Timestamp::MIN,
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

    pub(crate) fn is_available(&self, now: Timestamp) -> bool {
        self.expiry <= now
    }

    pub(crate) fn is_acked(&self) -> bool {
        self.expiry == Timestamp::MAX
    }
}

impl TableRow for QueueLeaseRow {
    const ROW_TYPE: u8 = RowType::QueueLease as u8;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

impl MsgRow {
    pub(crate) fn key_for(
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
    ) -> TableKey<Self> {
        TableKey::init_key(
            Self::ROW_TYPE,
            &[
                topic_id.as_bytes(),
                &partition.get().to_be_bytes(),
                &offset.to_be_bytes(),
            ],
            &[],
        )
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let start = Self::key_for(topic_id, partition, Offset::MIN).into_fjall_key();
        let end = Self::key_for(topic_id, partition, Offset::MAX).into_fjall_key();
        let item = keyspace.range(start..=end).next_back();
        match item {
            Some(kv) => {
                let key = kv.key()?;
                let offset = u64::from_be_bytes(
                    key[key.len().saturating_sub(SIZE_U64)..]
                        .try_into()
                        .expect("We know the size is right"),
                );
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
        let start = Self::key_for(topic_id, partition, offset).into_fjall_key();
        let end = Self::key_for(topic_id, partition, offset + batch_size as u64).into_fjall_key();
        for entry in keyspace.range(start..end) {
            let val = entry.value()?;
            let msg = rmp_serde::from_slice(&val).map_err_generic()?;
            results.push(msg);
        }

        tracing::Span::current().record("msgs_found", results.len());

        Ok(results)
    }
}

impl TableRow for MsgRow {
    const ROW_TYPE: u8 = RowType::Msg as u8;
}
