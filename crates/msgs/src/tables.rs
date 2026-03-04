use byteorder::{BigEndian, ByteOrder};
use diom_namespace::entities::NamespaceId;
use std::collections::HashMap;

use diom_error::{Error, Result};
use fjall_utils::{TableKey, TableKey2, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::entities::{ConsumerGroup, Offset, Partition, TopicId, TopicName};

#[repr(u8)]
enum RowType {
    Topic = 0,
    StreamLease = 1,
    Msg = 2,
}

const SIZE_U64: usize = size_of::<u64>();

#[derive(Serialize, Deserialize)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub name: TopicName,
    pub partitions: u16,
}

impl TableRow for TopicRow {}

impl TopicRow {
    pub(crate) fn key_for(namespace_id: NamespaceId, topic: &TopicName) -> TableKey2<Self> {
        TableKey2::init_key(RowType::Topic as u8, &[namespace_id.as_bytes()], &[key])
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
}

impl TableRow for TopicRow {}

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
    ) -> TableKey2<Self> {
        TableKey2::init_key(
            RowType::StreamLease as u8,
            &[topic_id.as_bytes(), &partition.get().to_be_bytes()],
            &[consumer_group.as_str()],
        )
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            offset: 0,
            expiry: Timestamp::MIN,
        })
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
    pub(crate) fn key_for(
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
    ) -> TableKey2<Self> {
        TableKey2::init_key(
            RowType::Msg as u8,
            &[
                topic_id.as_bytes(),
                &partition.get().to_be_bytes(),
                offset.to_be_bytes(),
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
        let start = Self::key_for(topic_id, partition, Offset::MIN).fjall_key();
        let end = Self::key_for(topic_id, partition, Offset::MAX).fjall_key();
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
        let start = Self::key_for(topic_id, partition, offset).fjall_key();
        let end = Self::key_for(topic_id, partition, offset + batch_size).fjall_key();
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
