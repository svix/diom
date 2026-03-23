use std::{
    future::{Future, IntoFuture},
    ops::Deref,
    pin::Pin,
};

use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, header};
use http_body_util::BodyExt as _;
use reqwest::StatusCode;
use serde::Serialize;
use url::Url;

const APPLICATION_MSGPACK: HeaderValue = HeaderValue::from_static("application/msgpack");

#[derive(Clone, Debug)]
pub struct TestClient {
    pub base_uri: String,
    pub auth_header: Option<String>,
    pub client: reqwest::Client,
}

impl TestClient {
    pub fn set_auth_header(&mut self, auth_header: String) {
        self.auth_header = Some(format!("Bearer {auth_header}"));
    }

    pub fn clear_auth_header(&mut self) {
        self.auth_header = None;
    }
}

impl TestClient {
    pub fn new(base_uri: String, auth_token: &str) -> TestClient {
        TestClient {
            base_uri,
            auth_header: Some(format!("Bearer {auth_token}")),
            client: reqwest::Client::new(),
        }
    }

    /// Create another `TestClient` with the same base URI and auth token,
    /// but using a distinct `reqwest::Client` object.
    pub fn clone_new_http_client(&self) -> Self {
        Self {
            base_uri: self.base_uri.clone(),
            auth_header: self.auth_header.clone(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) fn build_uri(&self, endpoint: impl AsRef<str>) -> String {
        format!("{}/{}", self.base_uri, endpoint.as_ref())
    }

    #[allow(unused)]
    pub fn inner(&self) -> reqwest::Client {
        self.client.clone()
    }

    pub fn get(&self, endpoint: impl Into<String>) -> TestRequestBuilder<'_> {
        self.request(http::Method::GET, endpoint)
    }

    pub fn post(&self, endpoint: impl Into<String>) -> TestRequestBuilder<'_> {
        self.request(http::Method::POST, endpoint)
    }

    pub fn put(&self, endpoint: impl Into<String>) -> TestRequestBuilder<'_> {
        self.request(http::Method::PUT, endpoint)
    }

    pub fn delete(&self, endpoint: impl Into<String>) -> TestRequestBuilder<'_> {
        self.request(http::Method::DELETE, endpoint)
    }

    pub fn patch(&self, endpoint: impl Into<String>) -> TestRequestBuilder<'_> {
        self.request(http::Method::PATCH, endpoint)
    }

    pub fn request(
        &self,
        method: http::Method,
        endpoint: impl Into<String>,
    ) -> TestRequestBuilder<'_> {
        let mut headers = HeaderMap::new();
        if let Some(header_val) = &self.auth_header {
            headers.insert(header::AUTHORIZATION, header_val.try_into().unwrap());
        }

        TestRequestBuilder {
            client: self,
            method,
            endpoint: endpoint.into(),
            headers,
            body: None,
        }
    }
}

pub struct TestRequestBuilder<'a> {
    client: &'a TestClient,
    method: http::Method,
    endpoint: String,
    headers: HeaderMap,
    body: Option<Bytes>,
}

impl TestRequestBuilder<'_> {
    #[track_caller]
    pub fn header<K, V>(mut self, name: K, value: V) -> Self
    where
        K: TryInto<HeaderName>,
        V: TryInto<HeaderValue>,
        K::Error: std::fmt::Debug,
        V::Error: std::fmt::Debug,
    {
        self.headers
            .insert(name.try_into().unwrap(), value.try_into().unwrap());
        self
    }

    pub fn body(mut self, bytes: impl Into<Bytes>) -> Self {
        assert!(self.body.is_none());
        self.body = Some(bytes.into());
        self
    }

    #[must_use]
    #[track_caller]
    pub fn json(mut self, body: impl Serialize) -> Self {
        assert!(self.body.is_none());
        self.headers.insert(header::CONTENT_TYPE, APPLICATION_JSON);
        self.body = Some(serde_json::to_vec(&body).unwrap().into());
        self
    }

    #[must_use]
    #[track_caller]
    pub fn msgpack(mut self, body: impl Serialize) -> Self {
        assert!(self.body.is_none());
        self.headers
            .insert(header::CONTENT_TYPE, APPLICATION_MSGPACK);
        self.body = Some(rmp_serde::to_vec_named(&body).unwrap().into());
        self
    }

    pub async fn send(self) -> reqwest::Result<TestResponse> {
        let url = Url::parse(&self.client.build_uri(self.endpoint)).unwrap();
        let mut req = reqwest::Request::new(self.method, url);
        *req.headers_mut() = self.headers;
        if let Some(body) = self.body {
            *req.body_mut() = Some(body.into());
        }

        let response: http::Response<reqwest::Body> = self.client.client.execute(req).await?.into();

        let (parts, body) = response.into_parts();
        let body_bytes = body.collect().await?.to_bytes();
        Ok(TestResponse(http::Response::from_parts(parts, body_bytes)))
    }
}

impl<'a> IntoFuture for TestRequestBuilder<'a> {
    type Output = reqwest::Result<TestResponse>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

pub struct TestResponse(http::Response<Bytes>);

impl TestResponse {
    pub fn ensure(self, expected_status: StatusCode) -> anyhow::Result<Self> {
        if self.status() != expected_status {
            let actual_status = self.status();
            let msg_fragment_body = if self.body().is_empty() {
                ""
            } else {
                match self.headers().get(header::CONTENT_TYPE) {
                    Some(ty) if ty == APPLICATION_JSON => &format!(", body: {}", self.json()),
                    _ => &format!(", body: {}", String::from_utf8_lossy(self.body())),
                }
            };

            anyhow::bail!(
                "expected status: {expected_status}, \
                 actual status: {actual_status}{msg_fragment_body}",
            );
        }

        if expected_status == StatusCode::NO_CONTENT {
            assert_eq!(self.body(), b"".as_slice());
        }

        Ok(self)
    }

    pub fn ensure_not_found(self) -> anyhow::Result<()> {
        let response = self.ensure(StatusCode::BAD_REQUEST)?.json();
        anyhow::ensure!(response["code"] == "not_found");
        Ok(())
    }

    #[track_caller]
    pub fn expect(self, expected_status: StatusCode) -> Self {
        self.ensure(expected_status).unwrap()
    }

    #[must_use]
    #[track_caller]
    #[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
    pub fn json(&self) -> serde_json::Value {
        assert_eq!(
            self.headers().get(header::CONTENT_TYPE),
            Some(&APPLICATION_JSON),
            "unexpected non-JSON response: {:?}",
            self.0,
        );
        serde_json::from_slice(self.body()).unwrap()
    }
}

impl Deref for TestResponse {
    type Target = http::Response<Bytes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
