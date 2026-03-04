use byteorder::{BigEndian, ByteOrder};
use coyote_namespace::entities::NamespaceId;
use std::collections::HashMap;

use coyote_error::{Error, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ConsumerGroup, Offset, Partition, TopicId, TopicName};

const SIZE_U64: usize = size_of::<u64>();

fn construct_key(parts: &[&[u8]]) -> Vec<u8> {
    let len = parts.iter().fold(0, |acc, e| acc + e.len());
    let mut ret = Vec::with_capacity(len);
    for part in parts {
        ret.extend_from_slice(part);
    }
    ret
}

pub(crate) trait TableRow: serde::de::DeserializeOwned + Serialize {
    fn from_fjall_value(value: fjall::UserValue) -> Result<Self> {
        rmp_serde::from_slice(&value).map_err(Error::generic)
    }

    fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec(&self)
            .map(fjall::UserValue::from)
            .map_err(Error::generic)
    }

    fn fjall_fetch(keyspace: &fjall::Keyspace, key: &[u8]) -> Result<Option<Self>> {
        keyspace.get(key)?.map(Self::from_fjall_value).transpose()
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub name: TopicName,
    pub partitions: u16,
}

impl TopicRow {
    pub(crate) fn construct_key(namespace_id: NamespaceId, topic: &TopicName) -> Vec<u8> {
        let parts = ["top".as_bytes(), namespace_id.as_bytes(), topic.as_bytes()];

        construct_key(&parts)
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

    pub(crate) fn fetch(
        keyspace: &fjall::Keyspace,
        namespace_id: NamespaceId,
        topic: &TopicName,
    ) -> Result<Option<Self>> {
        let key = Self::construct_key(namespace_id, topic);
        Self::fjall_fetch(keyspace, &key)
    }
}

impl TableRow for TopicRow {}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamLeaseRow {
    pub offset: u64,
    pub expiry: Timestamp,
    /// Last offset in the current leased batch. The lease is only released
    /// when the committed offset reaches this value.
    pub end_offset: Offset,
}

impl StreamLeaseRow {
    fn construct_key_(topic_id: TopicId, partition: Partition, consumer_group: &str) -> Vec<u8> {
        let parts = [
            "strmleas".as_bytes(),
            topic_id.as_bytes(),
            &partition.get().to_be_bytes(),
            consumer_group.as_bytes(),
        ];

        construct_key(&parts)
    }

    pub(crate) fn construct_key(
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Vec<u8> {
        Self::construct_key_(topic_id, partition, &consumer_group.0)
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            offset: 0,
            expiry: Timestamp::MIN,
            end_offset: 0,
        })
    }

    pub(crate) fn fetch(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Result<Option<Self>> {
        let key = Self::construct_key(topic_id, partition, consumer_group);
        Self::fjall_fetch(keyspace, &key)
    }
}

impl TableRow for StreamLeaseRow {}

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

impl MsgRow {
    pub(crate) fn construct_key(
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
    ) -> Vec<u8> {
        let mut offset_buf = [0u8; SIZE_U64];
        BigEndian::write_u64(&mut offset_buf, offset);

        let parts = [
            "msg".as_bytes(),
            topic_id.as_bytes(),
            &partition.get().to_be_bytes(),
            &offset_buf,
        ];

        construct_key(&parts)
    }

    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let start = Self::construct_key(topic_id, partition, Offset::MIN);
        let end = Self::construct_key(topic_id, partition, Offset::MAX);
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
        let start = Self::construct_key(topic_id, partition, offset);
        let end = Self::construct_key(topic_id, partition, offset + batch_size as u64);
        for entry in keyspace.range(start..end) {
            let val = entry.value()?;
            let msg = rmp_serde::from_slice(&val).map_err(Error::generic)?;
            results.push(msg);
        }

        tracing::Span::current().record("msgs_found", results.len());

        Ok(results)
    }
}

impl TableRow for MsgRow {}
