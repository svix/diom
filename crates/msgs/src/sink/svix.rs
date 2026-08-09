#![allow(clippy::disallowed_types)]
use diom_core::template_str::CompiledTemplate;

use super::{build_vars, send_request};
use crate::{entities::SvixSinkConfig, operations::StreamReceiveMsg};

/// A Svix sink's templates compiled once. Delivery renders `app_id` into the URL and wraps the
/// message value as the Svix message payload under the templated `event_type`.
pub(super) struct CompiledSvixSink<'a> {
    token: &'a str,
    base_url: String,
    app_id: CompiledTemplate<'a>,
    event_type: CompiledTemplate<'a>,
    payload: Option<CompiledTemplate<'a>>,
}

impl<'a> CompiledSvixSink<'a> {
    pub(super) fn new(config: &'a SvixSinkConfig) -> Self {
        let base_url = config
            .server_url
            .clone()
            .unwrap_or_else(|| svix_base_url_from_token(&config.token));
        Self {
            token: &config.token,
            base_url,
            app_id: config.app_id.compile(),
            event_type: config.event_type.compile(),
            payload: config.payload.as_ref().map(|t| t.compile()),
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

        let request = http.post(&url).bearer_auth(self.token).json(&body);
        send_request(request).await
    }
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
