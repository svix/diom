// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsQueue<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsQueue<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Receives messages from a topic as competing consumers.
    ///
    /// Messages are individually leased for the specified duration. Multiple consumers can receive
    /// different messages from the same topic concurrently. Leased messages are skipped until they
    /// are acked or their lease expires.
    pub async fn receive(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_receive_in: MsgQueueReceiveIn,
    ) -> Result<MsgQueueReceiveOut> {
        let msg_queue_receive_in = MsgQueueReceiveIn_ {
            topic,
            consumer_group,
            batch_size: msg_queue_receive_in.batch_size,
            lease_duration_millis: msg_queue_receive_in.lease_duration_millis,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/queue/receive")
            .with_body(msg_queue_receive_in)
            .execute(self.cfg)
            .await
    }

    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    pub async fn ack(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: MsgQueueAckIn,
    ) -> Result<MsgQueueAckOut> {
        let msg_queue_ack_in = MsgQueueAckIn_ {
            topic,
            consumer_group,
            msg_ids: msg_queue_ack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/queue/ack")
            .with_body(msg_queue_ack_in)
            .execute(self.cfg)
            .await
    }

    /// Rejects messages, sending them to the dead-letter queue.
    ///
    /// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
    /// move them back to the queue for reprocessing.
    pub async fn nack(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_nack_in: MsgQueueNackIn,
    ) -> Result<MsgQueueNackOut> {
        let msg_queue_nack_in = MsgQueueNackIn_ {
            topic,
            consumer_group,
            msg_ids: msg_queue_nack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/queue/nack")
            .with_body(msg_queue_nack_in)
            .execute(self.cfg)
            .await
    }

    /// Moves all dead-letter queue messages back to the main queue for reprocessing.
    pub async fn redrive_dlq(
        &self,
        topic: String,
        consumer_group: String,
        msg_queue_redrive_dlq_in: MsgQueueRedriveDlqIn,
    ) -> Result<MsgQueueRedriveDlqOut> {
        let _unused = msg_queue_redrive_dlq_in;
        let msg_queue_redrive_dlq_in = MsgQueueRedriveDlqIn_ {
            topic,
            consumer_group,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/queue/redrive-dlq")
            .with_body(msg_queue_redrive_dlq_in)
            .execute(self.cfg)
            .await
    }
}
