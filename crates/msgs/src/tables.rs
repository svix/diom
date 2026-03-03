use std::collections::HashMap;

use byteorder::{BigEndian, ByteOrder};
use coyote_error::Result;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ConsumerGroup, Offset, Partition, TopicId, TopicName};

const SIZE_U64: usize = size_of::<u64>();

#[derive(Serialize, Deserialize)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub partitions: u16,
}

impl TopicRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, topic: &TopicName) -> Vec<u8> {
        let mut key = Vec::with_capacity(16 + topic.len());
        key.extend_from_slice(namespace_id.as_bytes());
        key.extend_from_slice(topic.as_bytes());
        key
    }

    pub(crate) fn new(now: Timestamp) -> Self {
        Self {
            id: uuid::Uuid::new_v7(uuid::Timestamp::from_unix(
                uuid::NoContext,
                now.as_second() as u64,
                now.subsec_nanosecond() as u32,
            )),
            partitions: 1,
        }
    }
}

impl TableRow for TopicRow {
    const TABLE_PREFIX: &'static str = "top";
    type Key = Vec<u8>;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamLeaseRow {
    pub offset: u64,
    pub expiry: Timestamp,
}

impl StreamLeaseRow {
    pub(crate) fn key_for(
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Vec<u8> {
        let mut key = Vec::with_capacity(16 + 2 + consumer_group.0.len());
        key.extend_from_slice(topic_id.as_bytes());
        key.extend_from_slice(&partition.get().to_be_bytes());
        key.extend_from_slice(consumer_group.0.as_bytes());
        key
    }

    pub(crate) fn new() -> Self {
        Self {
            offset: 0,
            expiry: Timestamp::MIN,
        }
    }
}

impl TableRow for StreamLeaseRow {
    const TABLE_PREFIX: &'static str = "strmleas";
    type Key = Vec<u8>;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

impl MsgRow {
    pub(crate) fn raw_key(topic_id: TopicId, partition: Partition, offset: Offset) -> Vec<u8> {
        let mut key = Vec::with_capacity(16 + 2 + SIZE_U64);
        key.extend_from_slice(topic_id.as_bytes());
        key.extend_from_slice(&partition.get().to_be_bytes());
        let mut offset_buf = [0u8; SIZE_U64];
        BigEndian::write_u64(&mut offset_buf, offset);
        key.extend_from_slice(&offset_buf);
        key
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let start = Self::make_fjall_key(&Self::raw_key(topic_id, partition, Offset::MIN));
        let end = Self::make_fjall_key(&Self::raw_key(topic_id, partition, Offset::MAX));
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
        let start = Self::make_fjall_key(&Self::raw_key(topic_id, partition, offset));
        let end = Self::make_fjall_key(&Self::raw_key(
            topic_id,
            partition,
            offset + batch_size as u64,
        ));
        for entry in keyspace.range(start..end) {
            let val = entry.value()?;
            results.push(Self::from_fjall_value(val)?);
        }

        tracing::Span::current().record("msgs_found", results.len());

        Ok(results)
    }
}

impl TableRow for MsgRow {
    const TABLE_PREFIX: &'static str = "msg";
    type Key = Vec<u8>;
}
