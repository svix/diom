use std::{collections::HashMap, fmt, ops::Deref};

use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type Offset = u64;

// FIXME(@svix-gabriel): Make partition count configurable per-namespace.
pub const DEFAULT_PARTITION_COUNT: u16 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PartitionIndex(u16);

impl PartitionIndex {
    pub fn new(index: u16) -> Option<Self> {
        if index < DEFAULT_PARTITION_COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    pub fn get(self) -> u16 {
        self.0
    }
}

/// Builds the partition-level topic name: `"{topic}~{partition}"`.
pub fn partition_topic_name(topic: &str, partition: PartitionIndex) -> String {
    format!("{topic}~{}", partition.0)
}

/// Splits a partition-level topic name back into `(topic, partition)`.
pub fn parse_partition_topic(s: &str) -> Result<(&str, PartitionIndex), &'static str> {
    let (topic, idx_str) = s
        .rsplit_once('~')
        .ok_or("missing '~' separator in partition topic name")?;
    let idx: u16 = idx_str
        .parse()
        .map_err(|_| "invalid partition index in topic name")?;
    PartitionIndex::new(idx)
        .map(|p| (topic, p))
        .ok_or("partition index out of range")
}

pub fn random_partition() -> PartitionIndex {
    PartitionIndex(rand::random_range(..DEFAULT_PARTITION_COUNT))
}

/// Deterministically maps a key to a partition via hash.
pub fn partition_for_key(key: &[u8]) -> PartitionIndex {
    let hash = djb2_hash(key);
    PartitionIndex(hash % DEFAULT_PARTITION_COUNT)
}

fn djb2_hash(data: &[u8]) -> u16 {
    let mut hash: u32 = 5381;
    for &b in data {
        hash = hash.wrapping_mul(33).wrapping_add(u32::from(b));
    }
    (hash % u32::from(DEFAULT_PARTITION_COUNT)) as u16
}

/// An opaque message ID that internally encodes `(partition, offset)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct MsgId(String);

impl MsgId {
    pub fn new(partition: PartitionIndex, offset: Offset) -> Self {
        Self(format!("{}:{offset}", partition.get()))
    }

    pub fn decode(&self) -> Option<(PartitionIndex, Offset)> {
        let (part_str, offset_str) = self.0.split_once(':')?;
        let part: u16 = part_str.parse().ok()?;
        let offset: Offset = offset_str.parse().ok()?;
        Some((PartitionIndex::new(part)?, offset))
    }
}

impl fmt::Display for MsgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct MsgIn {
    pub value: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Optional partition key. Messages with the same key are routed to the same partition.
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamMsgOut {
    pub offset: Offset,
    pub topic: String,
    pub value: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueMsgOut {
    pub msg_id: MsgId,
    pub value: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
}

/// A validated consumer group identifier.
///
/// Must be at most 64 bytes and only contain ASCII alphanumeric characters, `_`, or `-`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct ConsumerGroup(String);

impl ConsumerGroup {
    const MAX_LEN: usize = 64;

    fn validate_str(s: &str) -> Result<(), &'static str> {
        if s.len() > Self::MAX_LEN {
            return Err("consumer group name must be at most 64 bytes");
        }
        if !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(
                "consumer group name must only contain alphanumeric characters, '_', and '-'",
            );
        }
        Ok(())
    }
}

impl TryFrom<String> for ConsumerGroup {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::validate_str(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<&str> for ConsumerGroup {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::validate_str(s)?;
        Ok(Self(s.to_owned()))
    }
}

impl Deref for ConsumerGroup {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConsumerGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for ConsumerGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::validate_str(&s).map_err(serde::de::Error::custom)?;
        Ok(ConsumerGroup(s))
    }
}

impl JsonSchema for ConsumerGroup {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}
