// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsFifo<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsFifo<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Receives messages from a topic with strict per-key ordering.
    ///
    /// Like `queue/receive`, but a key is leased exclusively: once a consumer holds an in-flight
    /// message for a key, no other consumer receives that key's messages until it is acked (or its
    /// lease expires). A single call may return several messages of the same key, in order. Keyless
    /// messages are unordered. Note: increasing a topic's partition count re-hashes keys and can
    /// split a key across partitions, breaking its order at that boundary.
    pub async fn receive(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_receive_in: MsgFifoReceiveIn,
    ) -> Result<MsgFifoReceiveOut> {
        let msg_fifo_receive_in = MsgFifoReceiveIn_ {
            namespace: msg_fifo_receive_in.namespace,
            topic,
            consumer_group,
            batch_size: msg_fifo_receive_in.batch_size,
            lease_duration: msg_fifo_receive_in.lease_duration,
            batch_wait: msg_fifo_receive_in.batch_wait,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.receive")
            .with_body(msg_fifo_receive_in)
            .execute(self.cfg)
            .await
    }

    /// Acknowledges fifo messages by their opaque msg_ids, releasing each key for its next message.
    pub async fn ack(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_ack_in: MsgFifoAckIn,
    ) -> Result<MsgFifoAckOut> {
        let msg_fifo_ack_in = MsgFifoAckIn_ {
            namespace: msg_fifo_ack_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_fifo_ack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.ack")
            .with_body(msg_fifo_ack_in)
            .execute(self.cfg)
            .await
    }

    /// Extends the lease on in-flight fifo messages.
    pub async fn extend_lease(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_extend_lease_in: MsgFifoExtendLeaseIn,
    ) -> Result<MsgFifoExtendLeaseOut> {
        let msg_fifo_extend_lease_in = MsgFifoExtendLeaseIn_ {
            namespace: msg_fifo_extend_lease_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_fifo_extend_lease_in.msg_ids,
            lease_duration: msg_fifo_extend_lease_in.lease_duration,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.extend-lease")
            .with_body(msg_fifo_extend_lease_in)
            .execute(self.cfg)
            .await
    }

    /// Configures retry and DLQ behavior for a fifo consumer group on a topic.
    pub async fn configure(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_configure_in: MsgFifoConfigureIn,
    ) -> Result<MsgFifoConfigureOut> {
        let msg_fifo_configure_in = MsgFifoConfigureIn_ {
            namespace: msg_fifo_configure_in.namespace,
            topic,
            consumer_group,
            retry_schedule: msg_fifo_configure_in.retry_schedule,
            dlq_topic: msg_fifo_configure_in.dlq_topic,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.configure")
            .with_body(msg_fifo_configure_in)
            .execute(self.cfg)
            .await
    }

    /// Rejects fifo messages, retrying per the configured schedule then sending them to the DLQ.
    pub async fn nack(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_nack_in: MsgFifoNackIn,
    ) -> Result<MsgFifoNackOut> {
        let msg_fifo_nack_in = MsgFifoNackIn_ {
            namespace: msg_fifo_nack_in.namespace,
            topic,
            consumer_group,
            msg_ids: msg_fifo_nack_in.msg_ids,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.nack")
            .with_body(msg_fifo_nack_in)
            .execute(self.cfg)
            .await
    }

    /// Moves all dead-letter queue messages for a fifo consumer group back for reprocessing.
    pub async fn redrive_dlq(
        &self,
        topic: String,
        consumer_group: String,
        msg_fifo_redrive_dlq_in: MsgFifoRedriveDlqIn,
    ) -> Result<MsgFifoRedriveDlqOut> {
        let msg_fifo_redrive_dlq_in = MsgFifoRedriveDlqIn_ {
            namespace: msg_fifo_redrive_dlq_in.namespace,
            topic,
            consumer_group,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.fifo.redrive-dlq")
            .with_body(msg_fifo_redrive_dlq_in)
            .execute(self.cfg)
            .await
    }
}
