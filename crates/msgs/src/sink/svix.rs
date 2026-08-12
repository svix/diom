#![allow(clippy::disallowed_types)]
use std::fmt::Write;

use diom_core::template_str::CompiledTemplate;
use diom_id::NamespaceId;
use sha2::{Digest, Sha256};

use super::{build_vars, send_request};
use crate::{
    entities::{ConsumerGroup, SvixSinkConfig},
    operations::StreamReceiveMsg,
};

/// A Svix sink's templates compiled once. Delivery renders `app_id` into the URL and wraps the
/// message value as the Svix message payload under the templated `event_type`.
pub(super) struct CompiledSvixSink<'a> {
    namespace_id: NamespaceId,
    consumer_group: ConsumerGroup,
    token: &'a str,
    base_url: String,
    app_id: CompiledTemplate<'a>,
    event_type: CompiledTemplate<'a>,
    payload: Option<CompiledTemplate<'a>>,
    idempotency_key: Option<CompiledTemplate<'a>>,
}

impl<'a> CompiledSvixSink<'a> {
    pub(super) fn new(
        namespace_id: NamespaceId,
        consumer_group: ConsumerGroup,
        config: &'a SvixSinkConfig,
    ) -> Self {
        let base_url = config
            .server_url
            .clone()
            .unwrap_or_else(|| svix_base_url_from_token(&config.token));
        Self {
            namespace_id,
            consumer_group,
            token: &config.token,
            base_url,
            app_id: config.app_id.compile(),
            event_type: config.event_type.compile(),
            payload: config.payload.as_ref().map(|t| t.compile()),
            idempotency_key: config.idempotency_key.as_ref().map(|t| t.compile()),
        }
    }

    pub(super) async fn deliver(
        &self,
        http: &reqwest::Client,
        msg: &StreamReceiveMsg,
    ) -> Result<(), String> {
        let vars = build_vars(msg);
        let app_id = self.app_id.apply(&vars);
        let url = format!(
            "{}/api/v1/app/{}/msg/",
            self.base_url.trim_end_matches('/'),
            app_id
        );

        // Build the JSON body programmatically so `event_type` is escaped and the payload is embedded
        // as real JSON rather than a quoted string.
        let payload: serde_json::Value = match &self.payload {
            Some(template) => serde_json::from_str(&template.apply(&vars))
                .map_err(|e| format!("svix payload is not valid JSON: {e}"))?,
            None => serde_json::from_slice(&msg.value)
                .map_err(|e| format!("svix message value is not valid JSON: {e}"))?,
        };
        let body = serde_json::json!({
            "eventType": self.event_type.apply(&vars),
            "payload": payload,
        });

        // Use the configured key when it renders to something non-empty, otherwise fall back to a
        // stable key derived from the sink and message identity so retries de-duplicate.
        let idempotency_key = self
            .idempotency_key
            .as_ref()
            .map(|t| t.apply(&vars))
            .filter(|key| !key.trim().is_empty())
            .unwrap_or_else(|| {
                default_idempotency_key(self.namespace_id, &self.consumer_group, msg)
            });

        let request = http
            .post(&url)
            .bearer_auth(self.token)
            .header("idempotency-key", idempotency_key)
            .json(&body);
        send_request(request).await
    }
}

/// Builds the fallback idempotency key as a hex SHA256 of the sink identity (namespace, topic,
/// consumer_group) and the message identity (partition, offset). This is stable across retries of
/// the same message by the same sink, and distinct across sinks and messages. Hashing keeps the
/// value header safe regardless of topic contents.
fn default_idempotency_key(
    namespace_id: NamespaceId,
    consumer_group: &ConsumerGroup,
    msg: &StreamReceiveMsg,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(namespace_id.as_bytes());
    hasher.update(b":");
    hasher.update(msg.topic.topic.as_bytes());
    hasher.update(b":");
    hasher.update(consumer_group.as_bytes());
    hasher.update(b":");
    hasher.update(msg.topic.partition.get().to_be_bytes());
    hasher.update(b":");
    hasher.update(msg.offset.to_be_bytes());
    let digest: [u8; 32] = hasher.finalize().into();

    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    out
}

/// Infers the Svix API base URL from the token's region suffix, matching the Svix SDK. Tokens
/// without a known region fall back to the default host.
fn svix_base_url_from_token(token: &str) -> String {
    match token.split('.').next_back() {
        Some("us") => "https://api.us.svix.com",
        Some("eu") => "https://api.eu.svix.com",
        Some("in") => "https://api.in.svix.com",
        Some("ca") => "https://api.ca.svix.com",
        Some("au") => "https://api.au.svix.com",
        _ => "https://api.svix.com",
    }
    .to_owned()
}
