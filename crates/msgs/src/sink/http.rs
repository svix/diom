use diom_core::template_str::CompiledTemplate;

use super::{build_vars, send_request};
use crate::{
    entities::{HttpMethod, HttpSinkConfig},
    operations::StreamReceiveMsg,
};

/// An HTTP sink's templates compiled once so each delivery only runs [`CompiledTemplate::apply`]
/// against the per-message variables, instead of recompiling every template per message.
pub(super) struct CompiledHttpSink<'a> {
    method: reqwest::Method,
    url: CompiledTemplate<'a>,
    headers: Vec<(CompiledTemplate<'a>, CompiledTemplate<'a>)>,
    body: Option<CompiledTemplate<'a>>,
}

impl<'a> CompiledHttpSink<'a> {
    pub(super) fn new(config: &'a HttpSinkConfig) -> Self {
        let method = match config.method {
            HttpMethod::Post => reqwest::Method::POST,
            HttpMethod::Put => reqwest::Method::PUT,
            HttpMethod::Patch => reqwest::Method::PATCH,
        };
        let headers = config
            .headers
            .iter()
            .map(|(name, value)| (name.compile(), value.compile()))
            .collect();
        Self {
            method,
            url: config.url.compile(),
            headers,
            body: config.body.as_ref().map(|t| t.compile()),
        }
    }

    /// Delivers a single message to the sink's destination.
    pub(super) async fn deliver(
        &self,
        http: &reqwest::Client,
        msg: &StreamReceiveMsg,
    ) -> Result<(), String> {
        let vars = build_vars(msg);
        let url = self.url.apply(&vars);

        let mut request = http.request(self.method.clone(), &url);
        for (name, value) in &self.headers {
            request = request.header(name.apply(&vars), value.apply(&vars));
        }

        let body: Vec<u8> = match &self.body {
            Some(template) => template.apply(&vars).into_bytes(),
            None => msg.value.to_vec(),
        };

        send_request(request.body(body)).await
    }
}
