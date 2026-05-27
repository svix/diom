use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use diom_core::{Monotime, types::UnixTimestampMs};
use diom_error::{Error, Result, ResultExt};
use diom_id::{NamespaceId, TopicId};
use diom_namespace::{Namespace, entities::MsgsConfig};
use diom_operations::BackgroundResult;
use fjall::KeyspaceCreateOptions;
use opentelemetry::metrics::Meter;

use entities::{ConsumerGroup, Offset, Partition, TopicName};
use fjall_utils::{ReadableKeyspace, SerializableKeyspaceCreateOptions, TableRow};
use storage::{
    MsgRow, QueueLeaseRow, StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow,
    delete_expired_partition, delete_stale_queue_leases, delete_stale_stream_leases,
    earliest_offset,
};

use crate::metrics::{record_end_offsets, record_topic_lag_metrics};

pub mod compaction;
pub mod entities;
pub mod metrics;
pub mod operations;
pub(crate) mod storage;
mod topic_publish_notifier;

pub use topic_publish_notifier::*;

pub const MSG_KEYSPACE: &str = "mod_msgs";
pub const METADATA_KEYSPACE: &str = "mod_msgs_metadata";

pub type MsgsNamespace = Namespace<MsgsConfig>;

#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    pub(crate) metadata_tables: fjall::Keyspace,
    pub(crate) msg_table: fjall::Keyspace,
    pub(crate) metrics: metrics::MsgMetrics,
    pub(crate) topic_publish_notifier: TopicPublishNotifier,
    /// Caches the highest offset within a topic/partition.
    hwm_cache: Arc<DashMap<(TopicId, Partition), Offset>>,
}

impl State {
    pub fn init(
        db: fjall::Database,
        topic_publish_notifier: TopicPublishNotifier,
        meter: &Meter,
    ) -> Result<Self, Error> {
        let metadata_tables = db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let msg_table = SerializableKeyspaceCreateOptions::default()
            .expect_point_read_hits(true)
            .with_default_kv_separation()
            .create_and_record(&db, MSG_KEYSPACE)
            .or_internal_error()?;

        Ok(Self {
            db,
            metadata_tables,
            msg_table,
            metrics: metrics::MsgMetrics::new(meter),
            topic_publish_notifier,
            hwm_cache: Arc::new(DashMap::new()),
        })
    }

    /// Returns the next offset to assign for a partition.
    ///
    /// Checks the in-memory cache first. On a miss, falls through to disk
    /// (backward scan, then persisted HWM row) and populates the cache.
    pub(crate) fn next_offset(&self, topic_id: TopicId, partition: Partition) -> Result<Offset> {
        if let Some(cached) = self.hwm_cache.get(&(topic_id, partition)) {
            return Ok(*cached);
        }
        let offset = MsgRow::next_offset(&self.msg_table, topic_id, partition)?;
        self.hwm_cache.insert((topic_id, partition), offset);
        Ok(offset)
    }

    /// Updates the cached high-water mark after a successful write.
    pub(crate) fn set_hwm(&self, topic_id: TopicId, partition: Partition, next_offset: Offset) {
        self.hwm_cache.insert((topic_id, partition), next_offset);
    }
}

/// Counts available queue messages across all partitions.
///
/// For each partition, scans all `QueueLeaseRow` entries and counts messages that
/// are available (no lease, or lease expired and not in DLQ).
pub fn estimate_available_queue_messages(
    metadata_tables: &impl ReadableKeyspace,
    msg_table: &impl ReadableKeyspace,
    namespace_id: NamespaceId,
    topic: &TopicName,
    consumer_group: &ConsumerGroup,
    now: UnixTimestampMs,
) -> Result<u64> {
    let Some(topic_row) =
        TopicRow::fetch(metadata_tables, TopicKey::build_key(&namespace_id, topic))?
    else {
        return Ok(0);
    };

    let mut total = 0u64;
    for partition in topic_row.partitions() {
        let partition = partition?;

        // SMH should probably rename StreamLeaseRow to CursorRow or something,
        // the name is misleading here.
        let cursor_offset = StreamLeaseRow::fetch(
            metadata_tables,
            StreamLeaseKey::build_key(&topic_row.id, &partition, consumer_group),
        )?
        .map(|c| c.offset)
        .unwrap_or(0);

        let next_offset = MsgRow::next_offset(msg_table, topic_row.id, partition)?;
        let total_msgs = next_offset.saturating_sub(cursor_offset);

        let leases = QueueLeaseRow::scan_partition(
            metadata_tables,
            topic_row.id,
            partition,
            consumer_group,
        )?;
        let unavailable = leases.iter().filter(|(_, l)| !l.is_available(now)).count() as u64;

        total += total_msgs.saturating_sub(unavailable);
    }

    Ok(total)
}

/// Result of estimating available stream messages.
#[derive(Default, Debug)]
pub struct StreamEstimate {
    /// Estimated number of available messages across all unlocked partitions.
    pub count: u64,
    /// Partitions that are not currently leased.
    pub available_partitions: Vec<Partition>,
}

// NOTE - I'm not thrilled about the location of this method, but I didn't want to expose the
// tables module outside the msgs crate, and I wasn't sure where else to put this. 🤷
/// Cheap offset-based estimate of available stream messages across all partitions.
///
/// Partitions with active leases are skipped — stream semantics lock at the partition level.
pub fn estimate_available_stream_messages(
    metadata_tables: &impl ReadableKeyspace,
    msg_table: &impl ReadableKeyspace,
    namespace_id: NamespaceId,
    topic: &TopicName,
    consumer_group: &ConsumerGroup,
    now: UnixTimestampMs,
) -> Result<StreamEstimate> {
    let Some(topic_row) =
        TopicRow::fetch(metadata_tables, TopicKey::build_key(&namespace_id, topic))?
    else {
        return Ok(StreamEstimate::default());
    };

    let mut total = 0u64;
    let mut available_partitions = Vec::new();
    for partition in topic_row.partitions() {
        let partition = partition?;

        let cursor = StreamLeaseRow::fetch(
            metadata_tables,
            StreamLeaseKey::build_key(&topic_row.id, &partition, consumer_group),
        )?;

        // Skip partitions with active leases
        if cursor.as_ref().is_some_and(|c| c.expiry > now) {
            continue;
        }

        available_partitions.push(partition);

        let cursor_offset = cursor.map(|c| c.offset).unwrap_or(0);
        let next_offset = MsgRow::next_offset(msg_table, topic_row.id, partition)?;
        total += next_offset.saturating_sub(cursor_offset);
    }

    Ok(StreamEstimate {
        count: total,
        available_partitions,
    })
}

#[derive(Clone)]
pub struct AllNodesWorker {
    state: State,
    namespace_state: diom_namespace::State,
    time: Monotime,
}

impl AllNodesWorker {
    pub fn new(state: State, namespace_state: diom_namespace::State, time: Monotime) -> Self {
        Self {
            state,
            namespace_state,
            time,
        }
    }

    async fn worker_loop(&self) -> BackgroundResult<()> {
        // Periodically clear the cache, to prevent it from growing unbounded.
        // There's potentially a more complex scheme we can do, but this seemed
        // the most straightforward for now.
        self.state.hwm_cache.clear();

        let mut tasks = tokio::task::JoinSet::new();
        tasks.spawn_blocking({
            let state = self.state.clone();
            move || record_topic_lag_metrics(&state)
        });
        tasks.spawn_blocking({
            let state = self.state.clone();
            move || record_end_offsets(&state)
        });
        tasks.spawn_blocking({
            let state = self.state.clone();
            let namespace_state = self.namespace_state.clone();
            let now = self.time.now_utm();
            move || delete_expired_messages(&state, &namespace_state, now)
        });
        for result in tasks.join_all().await {
            if let Err(e) = result {
                tracing::warn!(error = %e, "msgs background worker task failed");
            }
        }
        Ok(())
    }
}

/// Iterates all namespaces with a retention period and deletes expired messages.
#[tracing::instrument(skip_all, fields(total_deleted))]
pub fn delete_expired_messages(
    state: &State,
    namespace_state: &diom_namespace::State,
    now: UnixTimestampMs,
) -> Result<()> {
    let namespaces = namespace_state.fetch_all_namespaces::<MsgsConfig>()?;
    let mut total_deleted: usize = 0;

    for ns in namespaces {
        let Some(retention_period) = ns.config.retention_period else {
            continue;
        };

        let cutoff = now.saturating_sub(retention_period);

        let topic_prefix = TopicKey::prefix_namespace_id(&ns.id);
        for entry in state.metadata_tables.prefix(topic_prefix) {
            let val = entry.value()?;
            let topic_row = TopicRow::from_fjall_value(val)?;

            for partition in topic_row.partitions() {
                let partition = partition?;
                let deleted = delete_expired_partition(
                    &state.db,
                    &state.msg_table,
                    topic_row.id,
                    partition,
                    cutoff,
                )?;
                total_deleted += deleted;

                if deleted > 0 {
                    let cutoff_offset = earliest_offset(&state.msg_table, topic_row.id, partition)?
                        .unwrap_or(u64::MAX);
                    delete_stale_queue_leases(
                        &state.db,
                        &state.metadata_tables,
                        topic_row.id,
                        partition,
                        cutoff_offset,
                    )?;
                }
            }

            delete_stale_stream_leases(&state.metadata_tables, &state.msg_table, topic_row.id)?;
        }
    }

    if total_deleted > 0 {
        tracing::debug!(total_deleted, "deleted expired messages");
    }
    tracing::Span::current().record("total_deleted", total_deleted);

    Ok(())
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:msgs";

    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(Duration::from_secs(60));
        let shutting_down = diom_core::shutdown::shutting_down_token();
        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            self.worker_loop().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod delete_expired_tests {
    use std::collections::HashMap;

    use diom_core::types::DurationMs;
    use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
    use diom_namespace::{entities::NamespaceName, operations::create_namespace::CreateNamespace};
    use diom_operations::OpContext;
    use fjall_utils::{Databases, WriteBatchExt};
    use opentelemetry::metrics::MeterProvider as _;

    use super::*;
    use std::num::NonZeroU16;

    use crate::{
        entities::{
            ConsumerGroup, MsgIn, Partition, SeekPosition, TopicIn, TopicName, TopicPartition,
        },
        operations::{
            MsgsRaftState, PublishOperation, PublishResponseData, QueueReceiveOperation, Response,
            StreamReceiveOperation,
        },
        storage::{MsgKey, MsgRow, QueueLeaseKey, StreamLeaseKey, TopicKey, TopicRow},
    };

    struct Fixture {
        _workdir: tempfile::TempDir,
        state: State,
        namespace_state: diom_namespace::State,
    }

    impl Fixture {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let persistent = fjall::Database::builder(workdir.path().join("persistent"))
                .temporary(true)
                .open()
                .unwrap();
            let ephemeral = fjall::Database::builder(workdir.path().join("ephemeral"))
                .temporary(true)
                .open()
                .unwrap();
            let meter = opentelemetry::metrics::NoopMeterProvider::default().meter("diom.testing");
            let namespace_state =
                diom_namespace::State::init(Databases::new(persistent.clone(), ephemeral)).unwrap();
            let state = State::init(persistent, TopicPublishNotifier::new(), &meter).unwrap();
            Self {
                _workdir: workdir,
                state,
                namespace_state,
            }
        }

        async fn create_namespace(
            &self,
            name: &str,
            retention: Option<DurationMs>,
            now: UnixTimestampMs,
        ) -> NamespaceId {
            CreateNamespace::<MsgsConfig>::new(
                NamespaceName(name.to_owned()),
                MsgsConfig {
                    retention_period: retention,
                    retention_bytes: None,
                },
                UuidV7RandomBytes::new_random(),
            )
            .apply_operation(&self.namespace_state, now)
            .await
            .unwrap();
            self.namespace_state
                .fetch_namespace_admin::<MsgsConfig>(name)
                .unwrap()
                .expect("namespace was just created")
                .id
        }

        fn create_topic(
            &self,
            namespace_id: NamespaceId,
            topic_name: &str,
            partitions: u16,
            now: UnixTimestampMs,
        ) -> TopicId {
            let topic = TopicName::new(topic_name.to_owned()).unwrap();
            let topic_row = TopicRow {
                id: TopicId::new(now, UuidV7RandomBytes::new_random()),
                name: topic.clone(),
                partitions,
            };
            let mut batch = self.state.db.batch();
            batch
                .insert_row(
                    &self.state.metadata_tables,
                    TopicKey::build_key(&namespace_id, &topic),
                    &topic_row,
                )
                .unwrap();
            batch.commit().unwrap();
            topic_row.id
        }

        fn insert_msg(
            &self,
            topic_id: TopicId,
            partition: Partition,
            offset: u64,
            timestamp: UnixTimestampMs,
        ) {
            let mut batch = self.state.db.batch();
            batch
                .insert_row(
                    &self.state.msg_table,
                    MsgKey {
                        topic_id,
                        partition,
                        offset,
                        timestamp,
                    },
                    &MsgRow {
                        value: b"hello".into(),
                        headers: HashMap::new(),
                        timestamp,
                        scheduled_at: None,
                    },
                )
                .unwrap();
            batch.commit().unwrap();
        }

        fn msg_count(&self, topic_id: TopicId, partition: Partition) -> usize {
            self.state
                .msg_table
                .prefix(MsgKey::prefix_partition(&topic_id, &partition))
                .count()
        }

        async fn publish(
            &self,
            namespace_id: NamespaceId,
            topic: &str,
            msgs: Vec<MsgIn>,
            now: UnixTimestampMs,
        ) -> PublishResponseData {
            let op = PublishOperation::new(
                namespace_id,
                TopicIn::TopicName(TopicName::new(topic.to_owned()).unwrap()),
                msgs,
                None,
            )
            .unwrap();
            let raft_state = MsgsRaftState {
                msgs: &self.state,
                namespace: &self.namespace_state,
            };
            let ctx = OpContext {
                timestamp: now,
                log_index: 0,
                term: 0,
            };
            let op: operations::MsgsOperation = op.into();
            let response = op.apply(raft_state, &ctx).await;
            match response {
                Response::Publish(r) => r.0.unwrap(),
                _ => panic!("unexpected response variant"),
            }
        }
    }

    fn ts(millis: i64) -> UnixTimestampMs {
        UnixTimestampMs::try_from_millisecond(millis).unwrap()
    }

    #[tokio::test]
    async fn deletes_expired_messages_across_partitions() {
        let fixture = Fixture::new();

        let now = ts(100_000);
        // Cutoff = now - 10s = 90_000. Anything with timestamp < 90_000 is expired.
        let retention = DurationMs::from_secs(10);
        let ns_id = fixture
            .create_namespace("with-retention", Some(retention), now)
            .await;
        let topic_id = fixture.create_topic(ns_id, "topic-a", 2, now);

        let p0 = Partition::ZERO;
        let p1 = Partition::ONE;

        // p0: two expired, one fresh (exactly at the cutoff).
        fixture.insert_msg(topic_id, p0, 0, ts(50_000));
        fixture.insert_msg(topic_id, p0, 1, ts(80_000));
        fixture.insert_msg(topic_id, p0, 2, ts(90_000));
        // p1: one expired, one fresh.
        fixture.insert_msg(topic_id, p1, 0, ts(80_000));
        fixture.insert_msg(topic_id, p1, 1, ts(95_000));

        assert_eq!(fixture.msg_count(topic_id, p0), 3);
        assert_eq!(fixture.msg_count(topic_id, p1), 2);

        delete_expired_messages(&fixture.state, &fixture.namespace_state, now).unwrap();

        assert_eq!(
            fixture.msg_count(topic_id, p0),
            1,
            "p0 should keep the one at the cutoff"
        );
        assert_eq!(
            fixture.msg_count(topic_id, p1),
            1,
            "p1 should keep the fresh row"
        );
    }

    #[tokio::test]
    async fn namespace_without_retention_is_skipped() {
        let fixture = Fixture::new();

        let now = ts(1_000_000);
        let ns_id = fixture.create_namespace("no-retention", None, now).await;
        let topic_id = fixture.create_topic(ns_id, "topic", 1, now);
        let partition = Partition::ZERO;

        // Very old message — would be expired if retention were set.
        fixture.insert_msg(topic_id, partition, 0, ts(1));

        delete_expired_messages(&fixture.state, &fixture.namespace_state, now).unwrap();

        assert_eq!(fixture.msg_count(topic_id, partition), 1);
    }

    #[tokio::test]
    async fn retention_is_scoped_per_namespace() {
        let fixture = Fixture::new();

        let now = ts(100_000);
        let ns_expiring = fixture
            .create_namespace("expiring", Some(DurationMs::from_secs(10)), now)
            .await;
        let ns_permanent = fixture.create_namespace("permanent", None, now).await;

        let topic_expiring = fixture.create_topic(ns_expiring, "t", 1, now);
        let topic_permanent = fixture.create_topic(ns_permanent, "t", 1, now);
        let p = Partition::ZERO;

        // Same old timestamp in both namespaces.
        fixture.insert_msg(topic_expiring, p, 0, ts(1_000));
        fixture.insert_msg(topic_permanent, p, 0, ts(1_000));

        delete_expired_messages(&fixture.state, &fixture.namespace_state, now).unwrap();

        assert_eq!(fixture.msg_count(topic_expiring, p), 0);
        assert_eq!(fixture.msg_count(topic_permanent, p), 1);
    }

    #[tokio::test]
    async fn offsets_are_monotonic_after_full_partition_deletion() {
        let fixture = Fixture::new();
        let retention = DurationMs::from_secs(10);
        let ns_id = fixture
            .create_namespace("ns", Some(retention), ts(1_000))
            .await;

        fn msg(value: &[u8]) -> MsgIn {
            MsgIn {
                value: value.into(),
                headers: HashMap::new(),
                key: None,
                delay: None,
            }
        }

        // Publish first batch at t=1000
        let batch1 = fixture
            .publish(
                ns_id,
                "topic",
                vec![msg(b"a"), msg(b"b"), msg(b"c")],
                ts(1_000),
            )
            .await;
        assert_eq!(batch1.topics.len(), 1);
        let batch1_start = batch1.topics[0].start_offset;
        let batch1_end = batch1.topics[0].offset;
        assert_eq!(batch1_start, 0);
        assert_eq!(batch1_end, 3);

        // Delete all messages (now = t=20_000, retention = 10s, cutoff = t=10_000)
        delete_expired_messages(&fixture.state, &fixture.namespace_state, ts(20_000)).unwrap();

        // Publish second batch — offsets must continue from 3, not reset to 0
        let batch2 = fixture
            .publish(ns_id, "topic", vec![msg(b"d"), msg(b"e")], ts(20_000))
            .await;
        assert_eq!(batch2.topics.len(), 1);
        let batch2_start = batch2.topics[0].start_offset;
        let batch2_end = batch2.topics[0].offset;
        assert_eq!(batch2_start, 3, "offsets must not reset after deletion");
        assert_eq!(batch2_end, 5);
    }

    impl Fixture {
        async fn queue_receive(
            &self,
            namespace_id: NamespaceId,
            topic: &str,
            consumer_group: &str,
            retention: Option<DurationMs>,
            now: UnixTimestampMs,
        ) {
            let op = QueueReceiveOperation::new(
                namespace_id,
                TopicIn::TopicName(TopicName::new(topic.to_owned()).unwrap()),
                ConsumerGroup::try_from(consumer_group).unwrap(),
                NonZeroU16::new(10).unwrap(),
                DurationMs::from_secs(30),
                retention,
            )
            .unwrap();
            let raft_state = MsgsRaftState {
                msgs: &self.state,
                namespace: &self.namespace_state,
            };
            let ctx = OpContext {
                timestamp: now,
                log_index: 0,
                term: 0,
            };
            let op: operations::MsgsOperation = op.into();
            let _ = op.apply(raft_state, &ctx).await;
        }

        async fn stream_receive(
            &self,
            namespace_id: NamespaceId,
            topic: &str,
            consumer_group: &str,
            retention: Option<DurationMs>,
            now: UnixTimestampMs,
        ) {
            let op = StreamReceiveOperation::new(
                namespace_id,
                TopicIn::TopicName(TopicName::new(topic.to_owned()).unwrap()),
                ConsumerGroup::try_from(consumer_group).unwrap(),
                NonZeroU16::new(10).unwrap(),
                DurationMs::from_secs(30),
                SeekPosition::Earliest,
                retention,
            )
            .unwrap();
            let raft_state = MsgsRaftState {
                msgs: &self.state,
                namespace: &self.namespace_state,
            };
            let ctx = OpContext {
                timestamp: now,
                log_index: 0,
                term: 0,
            };
            let op: operations::MsgsOperation = op.into();
            let _ = op.apply(raft_state, &ctx).await;
        }

        fn queue_lease_count(&self, topic_id: TopicId, partition: Partition) -> usize {
            self.state
                .metadata_tables
                .prefix(QueueLeaseKey::prefix_partition(&topic_id, &partition))
                .count()
        }

        fn stream_lease_count(&self, topic_id: TopicId) -> usize {
            self.state
                .metadata_tables
                .prefix(StreamLeaseKey::prefix_topic_id(&topic_id))
                .count()
        }
    }

    #[tokio::test]
    async fn stale_leases_cleaned_up_after_retention() {
        let fixture = Fixture::new();
        let retention = DurationMs::from_secs(10);
        let ns_id = fixture
            .create_namespace("ns-leases", Some(retention), ts(1_000))
            .await;

        fn msg(value: &[u8]) -> MsgIn {
            MsgIn {
                value: value.into(),
                headers: HashMap::new(),
                key: None,
                delay: None,
            }
        }

        // Publish messages at t=1000
        fixture
            .publish(
                ns_id,
                "topic",
                vec![msg(b"a"), msg(b"b"), msg(b"c")],
                ts(1_000),
            )
            .await;
        let topic_id = TopicRow::fetch(
            &fixture.state.metadata_tables,
            TopicKey::build_key(&ns_id, &TopicName::new("topic".to_owned()).unwrap()),
        )
        .unwrap()
        .unwrap()
        .id;
        let p = Partition::ZERO;

        // Consume via queue (creates QueueLeaseRows) and stream (creates StreamLeaseRow)
        fixture
            .queue_receive(ns_id, "topic", "queue-cg", Some(retention), ts(1_000))
            .await;
        fixture
            .stream_receive(ns_id, "topic", "stream-cg", Some(retention), ts(1_000))
            .await;

        // Verify leases exist (both queue and stream operations create StreamLeaseRows)
        assert!(
            fixture.queue_lease_count(topic_id, p) > 0,
            "queue leases should exist"
        );
        assert!(
            fixture.stream_lease_count(topic_id) > 0,
            "stream leases should exist"
        );

        // Delete all messages and clean up leases (now = t=20_000, cutoff = t=10_000)
        delete_expired_messages(&fixture.state, &fixture.namespace_state, ts(20_000)).unwrap();

        // Verify stale leases were cleaned up
        assert_eq!(
            fixture.queue_lease_count(topic_id, p),
            0,
            "stale queue leases should be deleted"
        );
        assert_eq!(
            fixture.stream_lease_count(topic_id),
            0,
            "stale stream lease should be deleted"
        );
    }

    impl Fixture {
        async fn publish_to_partition(
            &self,
            namespace_id: NamespaceId,
            topic: &str,
            partition: Partition,
            msgs: Vec<MsgIn>,
            now: UnixTimestampMs,
        ) -> PublishResponseData {
            let tp = TopicPartition::new(TopicName::new(topic.to_owned()).unwrap(), partition);
            let op = PublishOperation::new(namespace_id, TopicIn::TopicPartition(tp), msgs, None)
                .unwrap();
            let raft_state = MsgsRaftState {
                msgs: &self.state,
                namespace: &self.namespace_state,
            };
            let ctx = OpContext {
                timestamp: now,
                log_index: 0,
                term: 0,
            };
            let op: operations::MsgsOperation = op.into();
            let response = op.apply(raft_state, &ctx).await;
            match response {
                Response::Publish(r) => r.0.unwrap(),
                _ => panic!("unexpected response variant"),
            }
        }
    }

    #[tokio::test]
    async fn lease_cleanup_only_affects_stale_partitions() {
        let fixture = Fixture::new();
        let retention = DurationMs::from_secs(10);
        let ns_id = fixture
            .create_namespace("ns-partial", Some(retention), ts(1_000))
            .await;

        // Create a 2-partition topic
        let topic_id = fixture.create_topic(ns_id, "multi", 2, ts(1_000));
        let p0 = Partition::ZERO;
        let p1 = Partition::ONE;

        fn msg(value: &[u8]) -> MsgIn {
            MsgIn {
                value: value.into(),
                headers: HashMap::new(),
                key: None,
                delay: None,
            }
        }

        // Publish messages to both partitions at t=1000
        fixture
            .publish_to_partition(ns_id, "multi", p0, vec![msg(b"a0"), msg(b"b0")], ts(1_000))
            .await;
        fixture
            .publish_to_partition(ns_id, "multi", p1, vec![msg(b"a1"), msg(b"b1")], ts(1_000))
            .await;

        // Consume from both partitions at t=2_000 (all messages are within retention)
        fixture
            .queue_receive(ns_id, "multi", "cg", Some(retention), ts(2_000))
            .await;

        // Verify leases exist on both partitions
        assert!(
            fixture.queue_lease_count(topic_id, p0) > 0,
            "p0 queue leases should exist"
        );
        assert!(
            fixture.queue_lease_count(topic_id, p1) > 0,
            "p1 queue leases should exist"
        );
        assert!(
            fixture.stream_lease_count(topic_id) > 0,
            "stream leases should exist"
        );

        // Publish fresh messages to p1 only, so p1 stays alive after retention
        fixture
            .publish_to_partition(ns_id, "multi", p1, vec![msg(b"c1")], ts(15_000))
            .await;

        // Consume again — picks up the fresh p1 message, creating a queue lease at offset 2
        fixture
            .queue_receive(ns_id, "multi", "cg", Some(retention), ts(15_000))
            .await;

        let p1_leases_before = fixture.queue_lease_count(topic_id, p1);
        assert!(
            p1_leases_before > 0,
            "p1 should have queue leases for fresh messages"
        );

        // Run retention at t=20_000 (cutoff = t=10_000).
        // p0: all messages expire. p1: old messages expire, fresh one survives.
        delete_expired_messages(&fixture.state, &fixture.namespace_state, ts(20_000)).unwrap();

        // p0: all messages deleted, all leases should be cleaned up
        assert_eq!(
            fixture.msg_count(topic_id, p0),
            0,
            "p0 messages should be deleted"
        );
        assert_eq!(
            fixture.queue_lease_count(topic_id, p0),
            0,
            "p0 stale queue leases should be deleted"
        );

        // p1: fresh message survives, queue leases for it must be preserved
        assert_eq!(
            fixture.msg_count(topic_id, p1),
            1,
            "p1 fresh message should survive"
        );
        assert!(
            fixture.queue_lease_count(topic_id, p1) > 0,
            "p1 queue leases for surviving messages must be preserved"
        );

        // Stream lease for p0 should be deleted (cursor behind retention),
        // but p1's stream lease should be preserved (cursor is at the fresh message)
        assert_eq!(
            fixture.stream_lease_count(topic_id),
            1,
            "only p0's stream lease should be deleted, p1's must be preserved"
        );
    }
}
