use std::{
    collections::HashMap,
    fmt,
    num::{NonZeroU32, NonZeroU64},
    ops::{self, Deref},
    str::FromStr,
};

use diom_core::{
    PersistableValue,
    template_str::Template,
    types::{ByteString, DurationMs, UnixTimestampMs},
};
use diom_error::Error;
use fjall_utils::FjallKeyComponent;
use schemars::{JsonSchema, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use sha2::{Digest, Sha256};

pub type Offset = u64;

pub const DEFAULT_PARTITION_COUNT: u16 = 1;

/// Arbitrary for now — may be raised later.
pub const MAX_PARTITION_COUNT: u16 = 64;

pub const TOPIC_PARTITION_DELIMITER: &str = "~";

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    FjallKeyComponent,
    PersistableValue,
)]
#[serde(transparent)]
pub struct Partition(u16);

impl Partition {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    pub(crate) fn new(index: u16) -> Option<Self> {
        if index < MAX_PARTITION_COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    pub fn get(self) -> u16 {
        self.0
    }
}

impl FromStr for Partition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.parse::<u16>().map_err(|e| e.to_string())?;
        Self::new(index)
            .ok_or_else(|| format!("partition cannot be higher than {MAX_PARTITION_COUNT}"))
    }
}

impl ops::Rem<u16> for Partition {
    type Output = Self;

    fn rem(self, rhs: u16) -> Self::Output {
        Self(self.0 % rhs)
    }
}

/// A topic identifier without the partition.
///
/// Carries the `namespace` that owns this topic. Serializes as `"namespace:topic"`, or just
/// `"topic"` when the namespace is the default.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, FjallKeyComponent, PersistableValue,
)]
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

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "example": "some_topic_name",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
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

impl FromStr for TopicPartition {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (topic, idx_str) = value
            .rsplit_once(TOPIC_PARTITION_DELIMITER)
            .ok_or_else(|| Error::internal("missing '~' separator in topic"))?;
        let partition: Partition = idx_str
            .parse()
            .map_err(|_| Error::internal("invalid partition index in topic"))?;
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
        s.parse().map_err(de::Error::custom)
    }
}

impl JsonSchema for TopicPartition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "example": "some_topic_name~0",
        })
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
            s.parse()
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

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "example": "some_topic_name",
        })
    }
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, FjallKeyComponent, PersistableValue,
)]
#[serde(transparent)]
pub struct MsgsIdempotencyKey([u8; 32]);

impl MsgsIdempotencyKey {
    pub fn new(authorization: Option<&[u8]>, topic: &TopicName, key: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(authorization.unwrap_or(b"_internal_"));
        hasher.update(b":");
        hasher.update(topic.as_bytes());
        hasher.update(b":");
        hasher.update(key.as_bytes());
        Self(hasher.finalize().into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PersistableValue)]
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
        let partition: Partition = part_str.parse().map_err(de::Error::custom)?;
        let offset: Offset = off_str.parse().map_err(de::Error::custom)?;
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
pub struct MsgIn {
    pub value: ByteString,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamMsgOut {
    pub offset: Offset,
    pub topic: TopicPartition,
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueMsgOut {
    pub msg_id: MsgId,
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

/// A Svix poller configuration as returned by a list query.
///
/// The autoconfig `token` is obfuscated (e.g. `auto_v1_eyJh...fQ==`) so the
/// secret is never returned in full, to let callers recognize
/// which credential is configured.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SvixPollerListItem {
    pub topic: TopicName,
    pub poller_id: String,
    pub token: String,
}

/// Obfuscates a secret token for display, keeping only a short prefix and
/// suffix so callers can recognize it without recovering the secret, e.g.
/// `auto_v1_eyJh...fQ==`. Tokens too short to partially reveal are fully masked.
pub(crate) fn obfuscate_token(token: &str) -> String {
    const PREFIX_LEN: usize = 12;
    const SUFFIX_LEN: usize = 4;

    let chars: Vec<char> = token.chars().collect();
    if chars.len() <= PREFIX_LEN + SUFFIX_LEN {
        return "...".to_owned();
    }

    let prefix: String = chars[..PREFIX_LEN].iter().collect();
    let suffix: String = chars[chars.len() - SUFFIX_LEN..].iter().collect();
    format!("{prefix}...{suffix}")
}

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue,
)]
#[serde(rename_all = "lowercase")]
pub enum SeekPosition {
    Earliest,
    #[default]
    Latest,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue,
)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    #[default]
    Post,
    Put,
    Patch,
}

/// Configuration for a sink attached to a topic. New sink kinds can be added as variants without
/// breaking existing configs.
///
/// In the formats used by the public API (json/msgpack) this is internally tagged (e.g. `{"type": "http", ...}`)
/// This doesn't work for the non self-describing formats we use in raft, we so have to manually
/// externally tag it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SinkConfig {
    Http(HttpSinkConfig),
    Svix(SvixSinkConfig),
    Kafka(KafkaSinkConfig),
}

impl PersistableValue for SinkConfig {}

#[derive(Serialize, JsonSchema)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
#[schemars(rename = "SinkConfig")]
enum SinkConfigTaggedRef<'a> {
    Http(&'a HttpSinkConfig),
    Svix(&'a SvixSinkConfig),
    Kafka(&'a KafkaSinkConfig),
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
enum SinkConfigTagged {
    Http(HttpSinkConfig),
    Svix(SvixSinkConfig),
    Kafka(KafkaSinkConfig),
}

// Externally-tagged mirror used for non-self-describing formats
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum SinkConfigUntaggedRef<'a> {
    Http(&'a HttpSinkConfig),
    Svix(&'a SvixSinkConfig),
    Kafka(&'a KafkaSinkConfig),
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SinkConfigUntagged {
    Http(HttpSinkConfig),
    Svix(SvixSinkConfig),
    Kafka(KafkaSinkConfig),
}

impl Serialize for SinkConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match self {
                SinkConfig::Http(http) => SinkConfigTaggedRef::Http(http).serialize(serializer),
                SinkConfig::Svix(svix) => SinkConfigTaggedRef::Svix(svix).serialize(serializer),
                SinkConfig::Kafka(kafka) => SinkConfigTaggedRef::Kafka(kafka).serialize(serializer),
            }
        } else {
            match self {
                SinkConfig::Http(http) => SinkConfigUntaggedRef::Http(http).serialize(serializer),
                SinkConfig::Svix(svix) => SinkConfigUntaggedRef::Svix(svix).serialize(serializer),
                SinkConfig::Kafka(kafka) => {
                    SinkConfigUntaggedRef::Kafka(kafka).serialize(serializer)
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for SinkConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Ok(match SinkConfigTagged::deserialize(deserializer)? {
                SinkConfigTagged::Http(http) => SinkConfig::Http(http),
                SinkConfigTagged::Svix(svix) => SinkConfig::Svix(svix),
                SinkConfigTagged::Kafka(kafka) => SinkConfig::Kafka(kafka),
            })
        } else {
            Ok(match SinkConfigUntagged::deserialize(deserializer)? {
                SinkConfigUntagged::Http(http) => SinkConfig::Http(http),
                SinkConfigUntagged::Svix(svix) => SinkConfig::Svix(svix),
                SinkConfigUntagged::Kafka(kafka) => SinkConfig::Kafka(kafka),
            })
        }
    }
}

impl SinkConfig {
    /// Masks any secret credentials in place so the config can be returned to callers (e.g. from
    /// list endpoints) without leaking them.
    pub(crate) fn obfuscate_secrets(&mut self) {
        match self {
            SinkConfig::Http(_) => {}
            SinkConfig::Svix(svix) => svix.token = obfuscate_token(&svix.token),
            SinkConfig::Kafka(kafka) => kafka.security.obfuscate_secrets(),
        }
    }

    /// Rejects semantically invalid configs that deserialization cannot catch. HTTP and Svix
    /// templates are already validated when deserialized, so only Kafka has cross-field rules.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            SinkConfig::Http(_) | SinkConfig::Svix(_) => Ok(()),
            SinkConfig::Kafka(kafka) => kafka.security.validate(),
        }
    }
}

impl JsonSchema for SinkConfig {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        SinkConfigTaggedRef::schema_name()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        SinkConfigTaggedRef::schema_id()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        SinkConfigTaggedRef::json_schema(generator)
    }
}

/// Configuration for an HTTP sink. The `url`, `headers`, and `body` are templates rendered
/// per-message (see [`diom_core::template_str`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
pub struct HttpSinkConfig {
    /// Destination URL.
    pub url: Template,
    #[serde(default)]
    pub method: HttpMethod,
    #[serde(default)]
    pub headers: HashMap<Template, Template>,
    /// Templated request body. When absent, the raw message value bytes are sent unchanged.
    #[serde(default)]
    pub body: Option<Template>,
}

/// Configuration for a Svix sink. Each message is forwarded as a Svix message-create call
/// (`POST {server_url}/api/v1/app/{app_id}/msg/`). This is a thin convenience over an HTTP sink.
/// The `app_id`, `event_type`, and `payload` are templates rendered per-message
/// (see [`diom_core::template_str`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
pub struct SvixSinkConfig {
    /// Svix API token, sent as the bearer credential. Obfuscated in list responses.
    pub token: String,
    /// Target Svix application. Can be optionally templated.
    pub app_id: Template,
    /// Svix event type. Can be optionally templated.
    pub event_type: Template,
    /// Templated message payload. When absent, the raw message value bytes are used (must be JSON).
    #[serde(default)]
    pub payload: Option<Template>,
    /// Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
    /// key derived from the sink and message identity (namespace, topic, consumer_group, partition,
    /// offset) is used so retries are de-duplicated by Svix.
    #[serde(default)]
    pub idempotency_key: Option<Template>,
    /// Optional base URL override. When absent, the region is inferred from the token.
    #[serde(default)]
    pub server_url: Option<String>,
}

/// The connection security protocol, mapped onto librdkafka's `security.protocol`.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue,
)]
#[serde(rename_all = "kebab-case")]
pub enum SecurityProtocol {
    Plaintext,
    Ssl,
    SaslPlaintext,
    SaslSsl,
}

impl SecurityProtocol {
    #[cfg(feature = "kafka")]
    fn librdkafka_value(self) -> &'static str {
        match self {
            Self::Plaintext => "plaintext",
            Self::Ssl => "ssl",
            Self::SaslPlaintext => "sasl_plaintext",
            Self::SaslSsl => "sasl_ssl",
        }
    }

    fn is_sasl(self) -> bool {
        matches!(self, Self::SaslPlaintext | Self::SaslSsl)
    }
}

/// The SASL mechanism, mapped onto librdkafka's `sasl.mechanism`.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue,
)]
#[serde(rename_all = "kebab-case")]
pub enum SaslMechanism {
    Plain,
    ScramSha256,
    ScramSha512,
}

impl SaslMechanism {
    #[cfg(feature = "kafka")]
    fn librdkafka_value(self) -> &'static str {
        match self {
            Self::Plain => "PLAIN",
            Self::ScramSha256 => "SCRAM-SHA-256",
            Self::ScramSha512 => "SCRAM-SHA-512",
        }
    }
}

/// Connection security for a Kafka sink. Every field is optional and maps 1:1 onto a librdkafka
/// config key, so absent fields leave librdkafka at its defaults. Certificates and keys are inline
/// PEMs so credentials live in the config rather than in per-node files.
#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue,
)]
pub struct KafkaSecurity {
    #[serde(default)]
    pub security_protocol: Option<SecurityProtocol>,
    #[serde(default)]
    pub sasl_mechanism: Option<SaslMechanism>,
    #[serde(default)]
    pub sasl_username: Option<String>,
    /// Secret. Obfuscated in list responses.
    #[serde(default)]
    pub sasl_password: Option<String>,
    /// Inline CA certificate PEM. When absent, the system trust roots are used.
    #[serde(default)]
    pub ssl_ca_pem: Option<String>,
    /// Inline client certificate PEM for mutual TLS.
    #[serde(default)]
    pub ssl_certificate_pem: Option<String>,
    /// Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses.
    #[serde(default)]
    pub ssl_key_pem: Option<String>,
    /// Password for an encrypted client key. Secret. Fully redacted in list responses.
    #[serde(default)]
    pub ssl_key_password: Option<String>,
    #[serde(default)]
    pub enable_ssl_certificate_verification: Option<bool>,
}

impl KafkaSecurity {
    /// The librdkafka config key/value pairs for the fields that are set.
    #[cfg(feature = "kafka")]
    pub(crate) fn librdkafka_options(&self) -> Vec<(&'static str, String)> {
        let mut opts = Vec::new();
        let mut push = |key, value: Option<String>| {
            if let Some(value) = value {
                opts.push((key, value));
            }
        };
        push(
            "security.protocol",
            self.security_protocol
                .map(|p| p.librdkafka_value().to_owned()),
        );
        push(
            "sasl.mechanism",
            self.sasl_mechanism.map(|m| m.librdkafka_value().to_owned()),
        );
        push("sasl.username", self.sasl_username.clone());
        push("sasl.password", self.sasl_password.clone());
        push("ssl.ca.pem", self.ssl_ca_pem.clone());
        push("ssl.certificate.pem", self.ssl_certificate_pem.clone());
        push("ssl.key.pem", self.ssl_key_pem.clone());
        push("ssl.key.password", self.ssl_key_password.clone());
        push(
            "enable.ssl.certificate.verification",
            self.enable_ssl_certificate_verification
                .map(|v| v.to_string()),
        );
        opts
    }

    /// Rejects incoherent combinations before they reach the broker.
    fn validate(&self) -> Result<(), String> {
        let protocol_is_sasl = self
            .security_protocol
            .is_some_and(SecurityProtocol::is_sasl);

        if protocol_is_sasl && self.sasl_mechanism.is_none() {
            return Err("a SASL security_protocol requires sasl_mechanism".to_owned());
        }
        if self.sasl_mechanism.is_some() {
            if !protocol_is_sasl {
                return Err(
                    "sasl_mechanism requires a SASL security_protocol (sasl_plaintext or sasl_ssl)"
                        .to_owned(),
                );
            }
            if self.sasl_username.is_none() || self.sasl_password.is_none() {
                return Err("sasl_mechanism requires sasl_username and sasl_password".to_owned());
            }
        }
        // Mutual TLS needs both the client certificate and its key.
        if self.ssl_certificate_pem.is_some() != self.ssl_key_pem.is_some() {
            return Err("ssl_certificate_pem and ssl_key_pem must be provided together".to_owned());
        }
        Ok(())
    }

    fn obfuscate_secrets(&mut self) {
        if let Some(password) = &self.sasl_password {
            self.sasl_password = Some(obfuscate_token(password));
        }
        // Key material is fully redacted (even a prefix or suffix reveal is undesirable).
        if self.ssl_key_pem.is_some() {
            self.ssl_key_pem = Some("...".to_owned());
        }
        if self.ssl_key_password.is_some() {
            self.ssl_key_password = Some("...".to_owned());
        }
    }
}

/// Configuration for a Kafka sink. Each message is produced to `topic` on the target cluster. By
/// default the message value and headers pass through unchanged, but each can be templated
/// per-message (see [`diom_core::template_str`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
pub struct KafkaSinkConfig {
    /// Comma-separated `host:port` list of the target cluster's bootstrap brokers.
    pub bootstrap_servers: String,
    /// Destination Kafka topic.
    pub topic: String,
    /// Templated record key rendered per-message. When absent, records are produced without a key.
    #[serde(default)]
    pub key: Option<Template>,
    /// Templated record value. When absent, the raw message value bytes are produced unchanged.
    #[serde(default)]
    pub value: Option<Template>,
    /// Templated record headers merged on top of the message's own headers (which pass through by
    /// default). A templated header overrides a passed-through one with the same name.
    #[serde(default)]
    pub headers: HashMap<Template, Template>,
    /// Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT).
    #[serde(default)]
    pub security: KafkaSecurity,
}

fn default_sink_starting_position() -> SeekPosition {
    SeekPosition::Earliest
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
pub struct SinkSettings {
    /// Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
    #[serde(default = "default_sink_starting_position")]
    pub default_starting_position: SeekPosition,
    /// At most how many concurrent requests will be sent to the Sink.
    #[serde(default)]
    pub max_in_flight: Option<NonZeroU32>,
    pub config: SinkConfig,
}

/// A single sink configuration, as returned by list endpoints.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SinkListItem {
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub settings: SinkSettings,
}

/// A validated consumer group identifier.
///
/// Must be at most 64 bytes and only contain ASCII alphanumeric characters, `_`, `-`, or `.`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    FjallKeyComponent,
    PersistableValue,
)]
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

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "example": "some_consumer_group",
        })
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema, PersistableValue,
)]
pub struct Retention {
    #[serde(rename = "period_ms")]
    pub period: Option<DurationMs>,
    /// FIXME(817) - We're not sure yet how we want to implement this,
    /// and its not part of MVP, so obscuring it for now.
    #[schemars(skip)]
    pub size_bytes: Option<NonZeroU64>,
}
