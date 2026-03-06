use std::{collections::HashMap, fmt, num::NonZeroU64, ops::Deref, str::FromStr, time::Duration};

use diom_error::Error;
use diom_namespace::{entities::NamespaceName, parse_namespace};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self},
};
use uuid::Uuid;
use validator::Validate;

pub type Offset = u64;

pub const DEFAULT_PARTITION_COUNT: u16 = 1;

/// Arbitrary for now — may be raised later.
pub const MAX_PARTITION_COUNT: u16 = 64;

pub const TOPIC_PARTITION_DELIMITER: &str = "~";

pub const NAMESPACE_DELIMITER: char = ':';

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
pub struct TopicName {
    namespace: Option<NamespaceName>,
    topic: String,
}

impl TopicName {
    pub fn new(namespace: Option<NamespaceName>, topic: String) -> Result<Self, Error> {
        if topic.contains(TOPIC_PARTITION_DELIMITER) {
            Err(Error::generic("invalid topic"))
        } else if topic.len() > 64 {
            Err(Error::generic("topic cannot exceed 64 bytes"))
        } else {
            Ok(Self { namespace, topic })
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_ref().map(|x| &x[..])
    }
}

/// Derefs to the topic name (without namespace or partition).
impl Deref for TopicName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.topic
    }
}

impl fmt::Display for TopicName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{}{}{}", namespace, NAMESPACE_DELIMITER, self.topic)
        } else {
            write!(f, "{}", self.topic)
        }
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
        let (ns, topic) = parse_namespace(&s);
        Self::new(ns.map(|x| x.to_owned()), topic.to_owned()).map_err(de::Error::custom)
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
    pub raw: TopicName,
    pub partition: Partition,
}

impl TopicPartition {
    pub fn new(raw: TopicName, partition: Partition) -> Self {
        Self { raw, partition }
    }

    pub fn namespace(&self) -> Option<&str> {
        self.raw.namespace()
    }
}

impl TryFrom<String> for TopicPartition {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (ns, rest) = parse_namespace(&value);
        let (topic, idx_str) = rest
            .rsplit_once(TOPIC_PARTITION_DELIMITER)
            .ok_or_else(|| Error::generic("missing '~' separator in topic"))?;
        let idx: u16 = idx_str
            .parse()
            .map_err(|_| Error::generic("invalid partition index in topic"))?;
        let partition = Partition::new(idx)?;
        let raw = TopicName::new(ns.map(|x| x.to_owned()), topic.to_owned())?;
        Ok(Self { raw, partition })
    }
}

impl fmt::Display for TopicPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.raw, TOPIC_PARTITION_DELIMITER, self.partition.0
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
    /// Returns the raw topic name (without partition suffix).
    pub fn topic_name(&self) -> &TopicName {
        match self {
            TopicIn::TopicName(topic_name) => topic_name,
            TopicIn::TopicPartition(topic_partition) => &topic_partition.raw,
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        self.topic_name().namespace()
    }
}

impl<'de> Deserialize<'de> for TopicIn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (ns, rest) = parse_namespace(&s);
        if rest.contains(TOPIC_PARTITION_DELIMITER) {
            // Re-parse the full string via Topic::try_from (it handles default namespace too)
            TopicPartition::try_from(s)
                .map(TopicIn::TopicPartition)
                .map_err(de::Error::custom)
        } else {
            TopicName::new(ns.map(|x| x.to_owned()), rest.to_owned())
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
    /// Optional partition key. Messages with the same key are routed to the same partition.
    pub key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamMsgOut {
    pub offset: Offset,
    pub topic: TopicPartition,
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
pub struct ConsumerGroup(pub(crate) String);

// FIXME: change ConsumerGroup inner type to Cow<'static, str> to avoid allocating for
// the static `__queue__` sentinel.
impl ConsumerGroup {
    const MAX_LEN: usize = 64;

    /// Synthetic consumer group used internally by queue operations.
    pub(crate) fn queue() -> Self {
        Self("__queue__".to_owned())
    }

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

// FIXME: should be a newtype
pub type TopicId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct Retention {
    #[serde(default = "default_retention_millis")]
    pub millis: NonZeroU64,
    #[serde(default = "default_retention_bytes")]
    pub bytes: NonZeroU64,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            millis: default_retention_millis(),
            bytes: default_retention_bytes(),
        }
    }
}

pub fn default_retention_millis() -> NonZeroU64 {
    (Duration::from_hours(24 * 30).as_millis() as u64)
        .try_into()
        .unwrap()
}

pub fn default_retention_bytes() -> NonZeroU64 {
    NonZeroU64::new(1_000_000_000_000).expect("constant is non-zero")
}
