use diom_core::types::UnixTimestampMs;
use std::time::Duration;

use diom_core::Monotime;
use diom_error::{Error, Result, ResultExt};
use diom_id::NamespaceId;
use diom_namespace::{Namespace, entities::MsgsConfig};
use diom_operations::BackgroundResult;
use fjall::KeyspaceCreateOptions;

use entities::{ConsumerGroup, Partition, TopicName};
use fjall_utils::{ReadableKeyspace, SerializableKeyspaceCreateOptions, TableRow};
use storage::{
    MsgRow, QueueLeaseRow, StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow,
    delete_expired_partition,
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
}

impl State {
    pub fn init(
        db: fjall::Database,
        topic_publish_notifier: TopicPublishNotifier,
    ) -> Result<Self, Error> {
        let metadata_tables = db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let msg_table = SerializableKeyspaceCreateOptions::default()
            .expect_point_read_hits(true)
            .with_default_kv_separation()
            .create_and_record(&db, MSG_KEYSPACE)
            .or_internal_error()?;

        let meter = opentelemetry::global::meter("diom.svix.com");

        Ok(Self {
            db,
            metadata_tables,
            msg_table,
            metrics: metrics::MsgMetrics::new(&meter),
            topic_publish_notifier,
        })
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
    for partition_idx in 0..topic_row.partitions {
        let partition = Partition::new(partition_idx)?;

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
    for partition_idx in 0..topic_row.partitions {
        let partition = Partition::new(partition_idx)?;

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
fn delete_expired_messages(
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

            for partition_idx in 0..topic_row.partitions {
                let partition = Partition::new(partition_idx)?;
                let deleted = delete_expired_partition(
                    &state.db,
                    &state.msg_table,
                    topic_row.id,
                    partition,
                    cutoff,
                )?;
                total_deleted += deleted;
            }
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
    use fjall_utils::{Databases, WriteBatchExt};

    use super::*;
    use crate::{
        entities::{Partition, TopicName},
        storage::{MsgKey, MsgRow, TopicKey, TopicRow},
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
            let namespace_state =
                diom_namespace::State::init(Databases::new(persistent.clone(), ephemeral)).unwrap();
            let state = State::init(persistent, TopicPublishNotifier::new()).unwrap();
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

        let p0 = Partition::new(0).unwrap();
        let p1 = Partition::new(1).unwrap();

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
        let partition = Partition::new(0).unwrap();

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
        let p = Partition::new(0).unwrap();

        // Same old timestamp in both namespaces.
        fixture.insert_msg(topic_expiring, p, 0, ts(1_000));
        fixture.insert_msg(topic_permanent, p, 0, ts(1_000));

        delete_expired_messages(&fixture.state, &fixture.namespace_state, now).unwrap();

        assert_eq!(fixture.msg_count(topic_expiring, p), 0);
        assert_eq!(fixture.msg_count(topic_permanent, p), 1);
    }
}
