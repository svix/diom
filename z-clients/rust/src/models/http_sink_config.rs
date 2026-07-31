// this file is @generated
use serde::{Deserialize, Serialize};

use super::http_method::HttpMethod;

/// Configuration for an HTTP sink. The `url`, `headers`, and `body` are templates rendered
/// per-message (see [`diom_core::template_str`]).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpSinkConfig {
    /// Destination URL.
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// Templated request body. When absent, the raw message value bytes are sent unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl HttpSinkConfig {
    pub fn new(url: String) -> Self {
        Self {
            url,
            method: None,
            headers: None,
            body: None,
        }
    }

    pub fn with_method(mut self, value: impl Into<Option<HttpMethod>>) -> Self {
        self.method = value.into();
        self
    }

    pub fn with_headers(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.headers = value.into();
        self
    }

    pub fn with_body(mut self, value: impl Into<Option<String>>) -> Self {
        self.body = value.into();
        self
    }
}
