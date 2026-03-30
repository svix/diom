use opentelemetry::metrics::{Counter, Meter};

use crate::entities::{ConsumerGroup, TopicName};

impl From<&TopicName> for opentelemetry::KeyValue {
    fn from(topic: &TopicName) -> Self {
        opentelemetry::KeyValue::new("topic", topic.to_string())
    }
}

impl From<&ConsumerGroup> for opentelemetry::KeyValue {
    fn from(group: &ConsumerGroup) -> Self {
        opentelemetry::KeyValue::new("consumer_group", group.to_string())
    }
}

#[derive(Clone)]
pub struct MsgMetrics {
    pub(crate) published: Counter<u64>,
    pub(crate) published_bytes: Counter<u64>,

    pub(crate) queue_received: Counter<u64>,
    pub(crate) queue_acked: Counter<u64>,
    pub(crate) queue_nacked: Counter<u64>,
    pub(crate) queue_nack_retried: Counter<u64>,
    pub(crate) queue_nack_dlq: Counter<u64>,
    pub(crate) queue_redrive: Counter<u64>,

    pub(crate) stream_received: Counter<u64>,
    pub(crate) stream_committed: Counter<u64>,
    pub(crate) stream_no_lease: Counter<u64>,
    pub(crate) stream_seeks: Counter<u64>,
}

impl MsgMetrics {
    pub fn new(meter: &Meter) -> Self {
        Self {
            published: meter
                .u64_counter("diom.msgs.published")
                .with_description("Messages published")
                .with_unit("{message}")
                .build(),
            published_bytes: meter
                .u64_counter("diom.msgs.published.bytes")
                .with_description("Total bytes published")
                .with_unit("By")
                .build(),
            queue_received: meter
                .u64_counter("diom.msgs.queue.received")
                .with_description("Messages delivered")
                .with_unit("{message}")
                .build(),
            queue_acked: meter
                .u64_counter("diom.msgs.queue.acked")
                .with_description("Messages acknowledged in queue")
                .with_unit("{message}")
                .build(),
            queue_nacked: meter
                .u64_counter("diom.msgs.queue.nacked")
                .with_description("Messages nacked")
                .with_unit("{message}")
                .build(),
            queue_nack_retried: meter
                .u64_counter("diom.msgs.queue.retried")
                .with_description("Messages scheduled for retry")
                .with_unit("{message}")
                .build(),
            queue_nack_dlq: meter
                .u64_counter("diom.msgs.queue.nack.dlq")
                .with_description("Messages sent to dead-letter queue")
                .with_unit("{message}")
                .build(),
            queue_redrive: meter
                .u64_counter("diom.msgs.queue.redrive")
                .with_description("Messages redriven from dead-letter queue")
                .with_unit("{message}")
                .build(),
            stream_received: meter
                .u64_counter("diom.msgs.stream.received")
                .with_description("Messages delivered")
                .with_unit("{message}")
                .build(),
            stream_committed: meter
                .u64_counter("diom.msgs.stream.committed")
                .with_description("Stream offset commits")
                .with_unit("{event}")
                .build(),
            stream_no_lease: meter
                .u64_counter("diom.msgs.stream.no_lease")
                .with_description("Times stream receive failed because all partitions were leased")
                .with_unit("{event}")
                .build(),
            stream_seeks: meter
                .u64_counter("diom.msgs.stream.seek")
                .with_description("Stream seek operations")
                .with_unit("{event}")
                .build(),
        }
    }
}

impl MsgMetrics {
    pub(crate) fn record_published(&self, topic: &TopicName, msg_count: u64, bytes: u64) {
        let attrs = &[opentelemetry::KeyValue::from(topic)];
        self.published.add(msg_count, attrs);
        self.published_bytes.add(bytes, attrs);
    }

    pub(crate) fn record_queue_received(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        count: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.queue_received.add(count, attrs);
    }

    pub(crate) fn record_queue_acked(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        count: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.queue_acked.add(count, attrs);
    }

    pub(crate) fn record_queue_nacked(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        nacked: u64,
        retried: u64,
        dlq: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.queue_nacked.add(nacked, attrs);
        self.queue_nack_retried.add(retried, attrs);
        self.queue_nack_dlq.add(dlq, attrs);
    }

    pub(crate) fn record_queue_redrive(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        count: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.queue_redrive.add(count, attrs);
    }

    pub(crate) fn record_stream_received(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        count: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.stream_received.add(count, attrs);
    }

    pub(crate) fn record_stream_no_lease(&self, topic: &TopicName, consumer_group: &ConsumerGroup) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.stream_no_lease.add(1, attrs);
    }

    pub(crate) fn record_stream_committed(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.stream_committed.add(1, attrs);
    }

    pub(crate) fn record_stream_seek(&self, topic: &TopicName, consumer_group: &ConsumerGroup) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.stream_seeks.add(1, attrs);
    }
}
