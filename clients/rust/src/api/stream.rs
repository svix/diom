// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct StreamCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamAppendOptions {
    pub idempotency_key: Option<String>,
}

pub struct Stream<'a> {
    cfg: &'a Configuration,
}

impl<'a> Stream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Upserts a new Stream with the given name.
    pub async fn create(
        &self,
        create_stream_in: CreateStreamIn,
        options: Option<StreamCreateOptions>,
    ) -> Result<CreateStreamOut> {
        let StreamCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/create")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(create_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Appends messages to the stream.
    pub async fn append(
        &self,
        append_to_stream_in: AppendToStreamIn,
        options: Option<StreamAppendOptions>,
    ) -> Result<AppendToStreamOut> {
        let StreamAppendOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/append")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(append_to_stream_in)
            .execute(self.cfg)
            .await
    }
}
