use std::{collections::HashMap, fmt, ops::Deref};

use diom_error::Error;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type Offset = u64;

pub const DEFAULT_PARTITION_COUNT: u16 = 1;

/// Arbitrary for now — may be raised later.
pub const MAX_PARTITION_COUNT: u16 = 64;

pub const TOPIC_PARTITION_DELIMITER: &str = "~";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Partition(u16);

impl Partition {
    pub fn new(index: u16) -> Result<Self, Error> {
        if index < MAX_PARTITION_COUNT {
            Ok(Self(index))
        } else {
            Err(Error::invalid_user_input(format!(
                "partition cannot be higher than {MAX_PARTITION_COUNT}"
            )))
        }
    }

    pub fn get(self) -> u16 {
        self.0
    }
}

/// A topic identifier without the partition.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RawTopic(String);

impl RawTopic {
    pub fn new(s: String) -> Result<Self, Error> {
        if s.contains(TOPIC_PARTITION_DELIMITER) {
            Err(Error::generic("invalid topic"))
        } else if s.len() > 64 {
            // arbitrary limit for now. If you want to change this, just do it.
            // No need to tag me or make a ticket or something.
            Err(Error::generic("topic cannot exceed 64 bytes"))
        } else {
            Ok(Self(s))
        }
    }
}

impl Deref for RawTopic {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RawTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl JsonSchema for RawTopic {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Topic {
    pub raw: RawTopic,
    pub partition: Partition,
}

impl Topic {
    pub fn new(raw: RawTopic, partition: Partition) -> Self {
        Self { raw, partition }
    }
}

impl TryFrom<String> for Topic {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (topic, idx_str) = value
            .rsplit_once(TOPIC_PARTITION_DELIMITER)
            .ok_or_else(|| Error::generic("missing '~' separator in topic"))?;
        let idx: u16 = idx_str
            .parse()
            .map_err(|_| Error::generic("invalid partition index in topic"))?;
        let partition = Partition::new(idx)?;
        let raw = RawTopic::new(topic.to_owned())?;
        Ok(Self { raw, partition })
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.raw.0, TOPIC_PARTITION_DELIMITER, self.partition.0
        )
    }
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Topic::try_from(s).map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for Topic {
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

/// Topic input from the user, which may or may not contain the partition.
// TODO: remove Serialize — only needed temporarily for Raft operation serialization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TopicIn {
    Raw(RawTopic),
    WithPartition(Topic),
}

impl TopicIn {
    /// Returns the raw topic name (without partition suffix).
    pub fn raw_topic(&self) -> &RawTopic {
        match self {
            TopicIn::Raw(raw) => raw,
            TopicIn::WithPartition(topic) => &topic.raw,
        }
    }
}

impl<'de> Deserialize<'de> for TopicIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains(TOPIC_PARTITION_DELIMITER) {
            Topic::try_from(s)
                .map(TopicIn::WithPartition)
                .map_err(serde::de::Error::custom)
        } else {
            RawTopic::new(s)
                .map(TopicIn::Raw)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl JsonSchema for TopicIn {
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

pub fn random_partition(partition_count: u16) -> Partition {
    Partition(rand::random_range(..partition_count))
}

/// Deterministically maps a key to a partition via hash.
pub fn partition_for_key(key: &[u8], partition_count: u16) -> Partition {
    let hash = djb2_hash(key);
    Partition((hash % u32::from(partition_count)) as u16)
}

fn djb2_hash(data: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    for &b in data {
        hash = hash.wrapping_mul(33).wrapping_add(u32::from(b));
    }
    hash
}

/// An opaque message ID that internally encodes `(partition, offset)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MsgId {
    pub partition: Partition,
    pub offset: Offset,
}

impl MsgId {
    pub fn new(partition: Partition, offset: Offset) -> Self {
        Self { partition, offset }
    }
}

impl fmt::Display for MsgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.partition.0, self.offset)
    }
}

impl Serialize for MsgId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for MsgId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (part_str, off_str) = s
            .split_once(':')
            .ok_or_else(|| serde::de::Error::custom("Invalid MsgId"))?;
        let partition: u16 = part_str.parse().map_err(serde::de::Error::custom)?;
        let offset: Offset = off_str.parse().map_err(serde::de::Error::custom)?;
        let partition = Partition::new(partition).map_err(serde::de::Error::custom)?;
        Ok(MsgId { partition, offset })
    }
}

impl JsonSchema for MsgId {
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
    pub topic: Topic,
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
