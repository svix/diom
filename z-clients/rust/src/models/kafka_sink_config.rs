// this file is @generated
use serde::{Deserialize, Serialize};

use super::kafka_security::KafkaSecurity;

/// Configuration for a Kafka sink. Each message is produced to `topic` on the target cluster. By
/// default the message value and headers pass through unchanged, but each can be templated
/// per-message (see [`diom_core::template_str`]).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KafkaSinkConfig {
    /// Comma-separated `host:port` list of the target cluster's bootstrap brokers.
    pub bootstrap_servers: String,

    /// Destination Kafka topic.
    pub topic: String,

    /// Templated record key rendered per-message. When absent, records are produced without a key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Templated record value. When absent, the raw message value bytes are produced unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Templated record headers merged on top of the message's own headers (which pass through by
    /// default). A templated header overrides a passed-through one with the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<KafkaSecurity>,
}

impl KafkaSinkConfig {
    pub fn new(bootstrap_servers: String, topic: String) -> Self {
        Self {
            bootstrap_servers,
            topic,
            key: None,
            value: None,
            headers: None,
            security: None,
        }
    }

    pub fn with_key(mut self, value: impl Into<Option<String>>) -> Self {
        self.key = value.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<Option<String>>) -> Self {
        self.value = value.into();
        self
    }

    pub fn with_headers(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.headers = value.into();
        self
    }

    pub fn with_security(mut self, value: impl Into<Option<KafkaSecurity>>) -> Self {
        self.security = value.into();
        self
    }
}
