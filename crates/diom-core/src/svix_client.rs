use std::{future::Future, sync::Arc};

use svix::{
    api::{PollerV2PollOut, SinkInCommon},
    autoconfig::decode_autoconfig_token_v1,
    autoconfig_consumer::{AutoConfigConsumer, MessagePollerv2ConsumerPollOptions},
};

// We can't use `async fn ...` here, because we need to add the `+ Send` bound.
/// Abstraction over the Svix SDK for testability.
pub trait SvixAutoConfigClient: Send + Sync + Clone + 'static {
    fn new(token: String) -> Result<Self, SvixClientError>
    where
        Self: Sized;

    fn subscribe(&self) -> impl Future<Output = Result<(), SvixClientError>> + Send;

    fn receive(
        &self,
        consumer_id: &str,
        limit: Option<i32>,
        lease_duration_ms: Option<i32>,
    ) -> impl Future<Output = Result<PollerV2PollOut, SvixClientError>> + Send;

    fn commit(
        &self,
        consumer_id: &str,
        offset: i32,
    ) -> impl Future<Output = Result<(), SvixClientError>> + Send;
}

/// Validates that the token is a well-formed autoconfig token without
/// constructing a full client. Used by endpoint handlers for fast feedback.
pub fn validate_autoconfig_token(token: &str) -> Result<(), SvixClientError> {
    decode_autoconfig_token_v1(token)
        .map_err(|_| SvixClientError::new("invalid autoconfig token"))?;
    Ok(())
}

#[derive(Debug)]
pub struct SvixClientError(String);

impl SvixClientError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for SvixClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for SvixClientError {}

#[derive(Clone)]
pub struct RealSvixAutoConfigClient {
    consumer: Arc<AutoConfigConsumer>,
}

impl SvixAutoConfigClient for RealSvixAutoConfigClient {
    fn new(token: String) -> Result<Self, SvixClientError> {
        let consumer = AutoConfigConsumer::new(token, SinkInCommon::default())
            .map_err(|e| SvixClientError::new(e.to_string()))?;
        Ok(Self {
            consumer: Arc::new(consumer),
        })
    }

    async fn subscribe(&self) -> Result<(), SvixClientError> {
        self.consumer
            .subscribe()
            .await
            .map_err(|e| SvixClientError::new(e.to_string()))?;
        Ok(())
    }

    async fn receive(
        &self,
        consumer_id: &str,
        limit: Option<i32>,
        lease_duration_ms: Option<i32>,
    ) -> Result<PollerV2PollOut, SvixClientError> {
        let options = MessagePollerv2ConsumerPollOptions {
            limit,
            lease_duration_ms,
            ..Default::default()
        };
        self.consumer
            .receive(consumer_id.to_owned(), Some(options))
            .await
            .map_err(|e| SvixClientError::new(e.to_string()))
    }

    async fn commit(&self, consumer_id: &str, offset: i32) -> Result<(), SvixClientError> {
        self.consumer
            .commit(consumer_id.to_owned(), offset, None)
            .await
            .map_err(|e| SvixClientError::new(e.to_string()))
    }
}
