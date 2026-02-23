// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct StreamCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamGetOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamAppendOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamFetchOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamFetchLockingOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamAckRangeOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamAckOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamDlqOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct StreamRedriveOptions {
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

    /// Get stream with given name.
    pub async fn get(
        &self,
        get_stream_in: GetStreamIn,
        options: Option<StreamGetOptions>,
    ) -> Result<GetStreamOut> {
        let StreamGetOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/get-group")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(get_stream_in)
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

    /// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
    ///
    /// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
    /// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
    /// until the visibility timeout expires, or the messages are acked.
    pub async fn fetch(
        &self,
        fetch_from_stream_in: FetchFromStreamIn,
        options: Option<StreamFetchOptions>,
    ) -> Result<FetchFromStreamOut> {
        let StreamFetchOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/fetch")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(fetch_from_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Fetches messages from the stream, locking over the consumer group.
    ///
    /// This call prevents other consumers within the same consumer group from reading from the stream
    /// until either the visibility timeout expires, or the last message in the batch is acknowledged.
    pub async fn fetch_locking(
        &self,
        fetch_from_stream_in: FetchFromStreamIn,
        options: Option<StreamFetchLockingOptions>,
    ) -> Result<FetchFromStreamOut> {
        let StreamFetchLockingOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/fetch-locking")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(fetch_from_stream_in)
            .execute(self.cfg)
            .await
    }

    /// Acks the messages for the consumer group, allowing more messages to be consumed.
    pub async fn ack_range(
        &self,
        ack_msg_range_in: AckMsgRangeIn,
        options: Option<StreamAckRangeOptions>,
    ) -> Result<AckMsgRangeOut> {
        let StreamAckRangeOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/ack-range")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(ack_msg_range_in)
            .execute(self.cfg)
            .await
    }

    /// Acks a single message.
    pub async fn ack(&self, ack: Ack, options: Option<StreamAckOptions>) -> Result<AckOut> {
        let StreamAckOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/ack")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(ack)
            .execute(self.cfg)
            .await
    }

    /// Moves a message to the dead letter queue.
    pub async fn dlq(&self, dlq_in: DlqIn, options: Option<StreamDlqOptions>) -> Result<DlqOut> {
        let StreamDlqOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/dlq")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(dlq_in)
            .execute(self.cfg)
            .await
    }

    /// Redrives messages from the dead letter queue back to the stream.
    pub async fn redrive(
        &self,
        redrive_in: RedriveIn,
        options: Option<StreamRedriveOptions>,
    ) -> Result<RedriveOut> {
        let StreamRedriveOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/stream/redrive-dlq")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(redrive_in)
            .execute(self.cfg)
            .await
    }
}
