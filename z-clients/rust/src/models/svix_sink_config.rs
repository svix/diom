// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a Svix sink. Each message is forwarded as a Svix message-create call
/// (`POST {server_url}/api/v1/app/{app_id}/msg/`). This is a thin convenience over an HTTP sink.
/// The `app_id`, `event_type`, and `payload` are templates rendered per-message
/// (see [`diom_core::template_str`]).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SvixSinkConfig {
    /// Svix API token, sent as the bearer credential. Obfuscated in list responses.
    pub token: String,

    /// Target Svix application. Can be optionally templated.
    pub app_id: String,

    /// Svix event type. Can be optionally templated.
    pub event_type: String,

    /// Templated message payload. When absent, the raw message value bytes are used (must be JSON).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    /// Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
    /// key derived from the sink and message identity (namespace, topic, consumer_group, partition,
    /// offset) is used so retries are de-duplicated by Svix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,

    /// Optional base URL override. When absent, the region is inferred from the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
}

impl SvixSinkConfig {
    pub fn new(token: String, app_id: String, event_type: String) -> Self {
        Self {
            token,
            app_id,
            event_type,
            payload: None,
            idempotency_key: None,
            server_url: None,
        }
    }

    pub fn with_payload(mut self, value: impl Into<Option<String>>) -> Self {
        self.payload = value.into();
        self
    }

    pub fn with_idempotency_key(mut self, value: impl Into<Option<String>>) -> Self {
        self.idempotency_key = value.into();
        self
    }

    pub fn with_server_url(mut self, value: impl Into<Option<String>>) -> Self {
        self.server_url = value.into();
        self
    }
}
