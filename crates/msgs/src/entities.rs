use std::{collections::HashMap, fmt, num::NonZeroU64, ops::Deref, str::FromStr};

use coyote_core::types::DurationMs;
use coyote_error::Error;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self},
};
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

impl FromStr for Partition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s
            .parse::<u16>()
            .map_err(|e| Error::invalid_user_input(e.to_string()))?;
        Self::new(index)
    }
}

/// A topic identifier without the partition.
///
/// Carries the `namespace` that owns this topic. Serializes as `"namespace:topic"`, or just
/// `"topic"` when the namespace is the default.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopicName(String);

impl TopicName {
    pub fn new(topic: String) -> Result<Self, Error> {
        if topic.contains(TOPIC_PARTITION_DELIMITER) {
            Err(Error::internal("invalid topic"))
        } else if topic.len() > 64 {
            Err(Error::internal("topic cannot exceed 64 bytes"))
        } else {
            Ok(Self(topic))
        }
    }

    // FIXME(@svix-jplatte): This is used by the macro in endpoints/msgs.rs
    // Update the macro to be less stupid and remove this weird identity method.
    pub fn name(&self) -> &Self {
        self
    }
}

/// Derefs to the topic name (without namespace or partition).
impl Deref for TopicName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TopicName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for TopicName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for TopicName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(de::Error::custom)
    }
}

impl JsonSchema for TopicName {
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
pub struct TopicPartition {
    pub topic: TopicName,
    pub partition: Partition,
}

impl TopicPartition {
    pub fn new(topic: TopicName, partition: Partition) -> Self {
        Self { topic, partition }
    }

    pub fn name(&self) -> &TopicName {
        &self.topic
    }
}

impl TryFrom<String> for TopicPartition {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (topic, idx_str) = value
            .rsplit_once(TOPIC_PARTITION_DELIMITER)
            .ok_or_else(|| Error::internal("missing '~' separator in topic"))?;
        let idx: u16 = idx_str
            .parse()
            .map_err(|_| Error::internal("invalid partition index in topic"))?;
        let partition = Partition::new(idx)?;
        let topic = TopicName::new(topic.to_owned())?;
        Ok(Self { topic, partition })
    }
}

impl fmt::Display for TopicPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.topic, TOPIC_PARTITION_DELIMITER, self.partition.0
        )
    }
}

impl Serialize for TopicPartition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for TopicPartition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TopicPartition::try_from(s).map_err(de::Error::custom)
    }
}

impl JsonSchema for TopicPartition {
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopicIn {
    TopicName(TopicName),
    TopicPartition(TopicPartition),
}

impl TopicIn {
    /// Returns the topic name (without partition suffix).
    pub fn name(&self) -> &TopicName {
        match self {
            Self::TopicName(name) => name,
            Self::TopicPartition(part) => &part.topic,
        }
    }
}

impl<'de> Deserialize<'de> for TopicIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.contains(TOPIC_PARTITION_DELIMITER) {
            // Re-parse the full string via TopicPartition::try_from
            TopicPartition::try_from(s)
                .map(TopicIn::TopicPartition)
                .map_err(de::Error::custom)
        } else {
            TopicName::new(s)
                .map(TopicIn::TopicName)
                .map_err(de::Error::custom)
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

pub fn partition_for_key(key: Option<&str>, partition_count: u16) -> Partition {
    match key {
        Some(key) => partition_for_key_hash(key, partition_count),
        None => random_partition(partition_count),
    }
}

fn random_partition(partition_count: u16) -> Partition {
    Partition(rand::random_range(..partition_count))
}

/// Deterministically maps a key to a partition via hash.
fn partition_for_key_hash(key: &str, partition_count: u16) -> Partition {
    let hash = djb2_hash(key.as_bytes());
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
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for MsgId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (part_str, off_str) = s
            .split_once(':')
            .ok_or_else(|| de::Error::custom("Invalid MsgId"))?;
        let partition: u16 = part_str.parse().map_err(de::Error::custom)?;
        let offset: Offset = off_str.parse().map_err(de::Error::custom)?;
        let partition = Partition::new(partition).map_err(de::Error::custom)?;
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
    /// Optional partition key.
    ///
    /// Messages with the same key are routed to the same partition.
    pub key: Option<String>,
    /// Optional delay in milliseconds.
    ///
    /// The message will not be delivered to queue consumers
    /// until the delay has elapsed from the time of publish.
    #[serde(default, rename = "delay_ms")]
    pub delay: Option<DurationMs>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamMsgOut {
    pub offset: Offset,
    pub topic: TopicPartition,
    pub value: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueMsgOut {
    pub msg_id: MsgId,
    pub value: Vec<u8>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub timestamp: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SeekPosition {
    Earliest,
    #[default]
    Latest,
}

/// A validated consumer group identifier.
///
/// Must be at most 64 bytes and only contain ASCII alphanumeric characters, `_`, `-`, or `.`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct ConsumerGroup(pub(crate) String);

impl ConsumerGroup {
    const MAX_LEN: usize = 64;

    fn validate_str(s: &str) -> Result<(), &'static str> {
        if s.len() > Self::MAX_LEN {
            return Err("consumer group name must be at most 64 bytes");
        }
        if !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
        {
            return Err(
                "consumer group name must only contain alphanumeric characters, '_', and '-' and '.'",
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
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::validate_str(&s).map_err(de::Error::custom)?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema)]
pub struct Retention {
    #[serde(rename = "period_ms")]
    pub period: Option<DurationMs>,
    /// FIXME(817) - We're not sure yet how we want to implement this,
    /// and its not part of MVP, so obscuring it for now.
    #[schemars(skip)]
    pub size_bytes: Option<NonZeroU64>,
}
