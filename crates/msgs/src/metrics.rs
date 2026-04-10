use opentelemetry::metrics::{Counter, Gauge, Meter};

use crate::{
    State,
    entities::{ConsumerGroup, Partition, TopicName},
    tables::{MsgRow, StreamLeaseRow, TopicRow},
};
use diom_error::Result;

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

impl From<&Partition> for opentelemetry::KeyValue {
    fn from(partition: &Partition) -> Self {
        opentelemetry::KeyValue::new("partition", partition.get() as i64)
    }
}

#[derive(Clone)]
pub struct MsgMetrics {
    pub(crate) published: Counter<u64>,
    pub(crate) published_bytes: Counter<u64>,
    pub(crate) topic_lag: Gauge<u64>,

    pub(crate) queue_received: Counter<u64>,
    pub(crate) queue_acked: Counter<u64>,
    pub(crate) queue_nacked: Counter<u64>,
    pub(crate) queue_nack_retried: Counter<u64>,
    pub(crate) queue_nack_dlq: Counter<u64>,
    pub(crate) queue_lease_extended: Counter<u64>,
    pub(crate) queue_redrive: Counter<u64>,

    pub(crate) stream_received: Counter<u64>,
    pub(crate) stream_committed: Counter<u64>,
    pub(crate) stream_no_lease: Counter<u64>,
    pub(crate) stream_seeks: Counter<u64>,
    pub(crate) topic_end_offset: Gauge<u64>,
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
            queue_lease_extended: meter
                .u64_counter("diom.msgs.queue.lease_extended")
                .with_description("Queue lease extensions")
                .with_unit("{event}")
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
            topic_lag: meter
                .u64_gauge("diom.msgs.topic.lag")
                .with_description("Estimated topic lag")
                .with_unit("{message}")
                .build(),
            topic_end_offset: meter
                .u64_gauge("diom.msgs.topic.end_offset")
                .with_description("End offset of topic partition")
                .with_unit("{message}")
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

    pub(crate) fn record_queue_lease_extended(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        count: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into()];
        self.queue_lease_extended.add(count, attrs);
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

    pub(crate) fn record_topic_end_offset(
        &self,
        topic: &TopicName,
        partition: &Partition,
        end_offset: u64,
    ) {
        let attrs = &[topic.into(), partition.into()];
        self.topic_end_offset.record(end_offset, attrs);
    }

    pub(crate) fn record_topic_lag(
        &self,
        topic: &TopicName,
        consumer_group: &ConsumerGroup,
        partition: &Partition,
        lag: u64,
    ) {
        let attrs = &[topic.into(), consumer_group.into(), partition.into()];
        self.topic_lag.record(lag, attrs);
    }
}

pub(crate) struct PartitionLag {
    pub topic_name: TopicName,
    pub consumer_group: ConsumerGroup,
    pub partition: Partition,
    pub lag: u64,
}

fn compute_topic_lag(
    metadata_tables: &impl fjall_utils::ReadableKeyspace,
    msg_table: &impl fjall_utils::ReadableKeyspace,
) -> Result<Vec<PartitionLag>> {
    use fjall_utils::TableRow as _;
    use std::collections::HashSet;

    let mut results = Vec::new();
    for guard in metadata_tables.prefix([TopicRow::ROW_TYPE]) {
        let (_, val) = guard.into_inner()?;
        let topic_row: TopicRow = TopicRow::from_fjall_value(val)?;

        let prefix = StreamLeaseRow::topic_scan_prefix(topic_row.id);

        let mut consumer_groups = HashSet::new();
        for guard in metadata_tables.prefix(&prefix) {
            let (key, _) = guard.into_inner()?;
            if let Some(cg_str) = StreamLeaseRow::consumer_group_from_key(&key)
                && let Ok(consumer_group) = ConsumerGroup::try_from(cg_str)
            {
                consumer_groups.insert(consumer_group);
            }
        }

        for cg in consumer_groups {
            for partition_idx in 0..topic_row.partitions {
                let Ok(partition) = Partition::new(partition_idx) else {
                    continue;
                };
                let cursor_offset = StreamLeaseRow::fetch(
                    metadata_tables,
                    StreamLeaseRow::key_for(topic_row.id, partition, &cg),
                )?
                .map(|c| c.offset)
                .unwrap_or(0);
                let tail = MsgRow::next_offset(msg_table, topic_row.id, partition)?;
                results.push(PartitionLag {
                    topic_name: topic_row.name.clone(),
                    consumer_group: cg.clone(),
                    partition,
                    lag: tail.saturating_sub(cursor_offset),
                });
            }
        }
    }
    Ok(results)
}

pub(crate) fn record_end_offsets(state: &State) -> Result<()> {
    use fjall_utils::TableRow as _;

    for guard in state.metadata_tables.prefix([TopicRow::ROW_TYPE]) {
        let (_, val) = guard.into_inner()?;
        let topic_row: TopicRow = TopicRow::from_fjall_value(val)?;
        for partition_idx in 0..topic_row.partitions {
            let Ok(partition) = Partition::new(partition_idx) else {
                continue;
            };
            let end_offset = MsgRow::next_offset(&state.msg_table, topic_row.id, partition)?;
            state
                .metrics
                .record_topic_end_offset(&topic_row.name, &partition, end_offset);
        }
    }
    Ok(())
}

pub(crate) fn record_topic_lag_metrics(state: &State) -> Result<()> {
    for entry in compute_topic_lag(&state.metadata_tables, &state.msg_table)? {
        state.metrics.record_topic_lag(
            &entry.topic_name,
            &entry.consumer_group,
            &entry.partition,
            entry.lag,
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use diom_id::UuidV7RandomBytes;
    use fjall_utils::WriteBatchExt as _;
    use jiff::Timestamp;

    use super::*;
    use crate::tables::{MsgKey, MsgRow, StreamLeaseRow, TopicRow};

    fn make_db() -> (fjall::Database, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db = fjall::Database::builder(dir.path())
            .temporary(true)
            .open()
            .unwrap();
        (db, dir)
    }

    fn insert_topic(db: &fjall::Database, meta: &fjall::Keyspace, topic_row: &TopicRow) {
        use diom_id::NamespaceId;
        let mut batch = db.batch();
        batch
            .insert_row(
                meta,
                TopicRow::key_for(NamespaceId::nil(), &topic_row.name),
                topic_row,
            )
            .unwrap();
        batch.commit().unwrap();
    }

    fn insert_msg(
        db: &fjall::Database,
        msgs: &fjall::Keyspace,
        topic_row: &TopicRow,
        partition: Partition,
        offset: u64,
    ) {
        let mut batch = db.batch();
        let row = MsgRow {
            value: b"".into(),
            headers: HashMap::new(),
            timestamp: Timestamp::UNIX_EPOCH,
            scheduled_at: None,
        };
        batch
            .insert_row(
                msgs,
                MsgKey {
                    topic_id: topic_row.id,
                    partition,
                    offset,
                },
                &row,
            )
            .unwrap();
        batch.commit().unwrap();
    }

    fn insert_cursor(
        db: &fjall::Database,
        meta: &fjall::Keyspace,
        topic_row: &TopicRow,
        partition: Partition,
        cg: &ConsumerGroup,
        offset: u64,
    ) {
        let mut batch = db.batch();
        let row = StreamLeaseRow {
            offset,
            expiry: Timestamp::UNIX_EPOCH,
            end_offset: offset,
        };
        batch
            .insert_row(
                meta,
                StreamLeaseRow::key_for(topic_row.id, partition, cg),
                &row,
            )
            .unwrap();
        batch.commit().unwrap();
    }

    #[test]
    fn no_topics_returns_empty() {
        let (db, _dir) = make_db();
        let meta = db
            .keyspace(crate::METADATA_KEYSPACE, Default::default)
            .unwrap();
        let msgs = db.keyspace(crate::MSG_KEYSPACE, Default::default).unwrap();
        let results = compute_topic_lag(&meta, &msgs).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn lag_is_tail_minus_cursor() {
        let (db, _dir) = make_db();
        let meta = db
            .keyspace(crate::METADATA_KEYSPACE, Default::default)
            .unwrap();
        let msgs = db.keyspace(crate::MSG_KEYSPACE, Default::default).unwrap();

        let topic_name = TopicName::new("test-topic".to_string()).unwrap();
        let topic_row = TopicRow::new(
            topic_name,
            Timestamp::UNIX_EPOCH,
            UuidV7RandomBytes::new_random(),
        );
        let cg = ConsumerGroup::try_from("my-group").unwrap();
        let partition = Partition::new(0).unwrap();

        insert_topic(&db, &meta, &topic_row);

        let results = compute_topic_lag(&meta, &msgs).unwrap();
        assert!(results.is_empty());

        // 5 messages, cursor at 0
        for offset in 0..5 {
            insert_msg(&db, &msgs, &topic_row, partition, offset);
        }
        insert_cursor(&db, &meta, &topic_row, partition, &cg, 0);

        let results = compute_topic_lag(&meta, &msgs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].lag, 5);

        // Advance cursor to 2
        insert_cursor(&db, &meta, &topic_row, partition, &cg, 2);

        let results = compute_topic_lag(&meta, &msgs).unwrap();
        assert_eq!(results[0].lag, 3);
    }
}
