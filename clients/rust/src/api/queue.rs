// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct QueueSendOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueueReceiveOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueueAckOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueueNackOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueueRejectOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueuePurgeOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct QueueStatsOptions {
    pub idempotency_key: Option<String>,
}

pub struct Queue<'a> {
    cfg: &'a Configuration,
}

impl<'a> Queue<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Send messages to the queue
    pub async fn send(
        &self,
        queue_send_in: QueueSendIn,
        options: Option<QueueSendOptions>,
    ) -> Result<QueueSendOut> {
        let QueueSendOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/send")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_send_in)
            .execute(self.cfg)
            .await
    }

    /// Receive messages from the queue
    pub async fn receive(
        &self,
        queue_receive_in: QueueReceiveIn,
        options: Option<QueueReceiveOptions>,
    ) -> Result<QueueReceiveOut> {
        let QueueReceiveOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/receive")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_receive_in)
            .execute(self.cfg)
            .await
    }

    /// Acknowledge successful message processing
    pub async fn ack(
        &self,
        queue_ack_in: QueueAckIn,
        options: Option<QueueAckOptions>,
    ) -> Result<QueueAckOut> {
        let QueueAckOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/ack")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_ack_in)
            .execute(self.cfg)
            .await
    }

    /// Negative acknowledge - return message to queue or move to DLQ
    pub async fn nack(
        &self,
        queue_nack_in: QueueNackIn,
        options: Option<QueueNackOptions>,
    ) -> Result<QueueNackOut> {
        let QueueNackOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/nack")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_nack_in)
            .execute(self.cfg)
            .await
    }

    /// Reject a message - remove from processing and send to DLQ without retry
    pub async fn reject(
        &self,
        queue_reject_in: QueueRejectIn,
        options: Option<QueueRejectOptions>,
    ) -> Result<QueueRejectOut> {
        let QueueRejectOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/reject")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_reject_in)
            .execute(self.cfg)
            .await
    }

    /// Purge all messages from a queue
    pub async fn purge(
        &self,
        queue_purge_in: QueuePurgeIn,
        options: Option<QueuePurgeOptions>,
    ) -> Result<QueuePurgeOut> {
        let QueuePurgeOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/purge")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_purge_in)
            .execute(self.cfg)
            .await
    }

    /// Get queue statistics
    pub async fn stats(
        &self,
        queue_stats_in: QueueStatsIn,
        options: Option<QueueStatsOptions>,
    ) -> Result<QueueStatsOut> {
        let QueueStatsOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/queue/stats")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(queue_stats_in)
            .execute(self.cfg)
            .await
    }
}
