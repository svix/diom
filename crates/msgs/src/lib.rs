use diom_error::{Error, Result};
use diom_id::NamespaceId;
use diom_namespace::{Namespace, entities::MsgsConfig};
use diom_operations::BackgroundResult;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use jiff::Timestamp;

use entities::{ConsumerGroup, Partition, TopicName};
use fjall_utils::{ReadableKeyspace, TableRow};
use tables::{MsgRow, QueueLeaseRow, StreamLeaseRow, TopicRow};

use crate::metrics::record_topic_lag_metrics;

pub mod entities;
pub mod metrics;
pub mod operations;
pub(crate) mod tables;
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
        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        let msg_table = {
            let opts = KeyspaceCreateOptions::default()
                .expect_point_read_hits(true)
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(MSG_KEYSPACE, || opts)?
        };

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
    now: Timestamp,
) -> Result<u64> {
    let Some(topic_row) = TopicRow::fetch(metadata_tables, TopicRow::key_for(namespace_id, topic))?
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
            StreamLeaseRow::key_for(topic_row.id, partition, consumer_group),
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
#[derive(Default)]
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
    now: Timestamp,
) -> Result<StreamEstimate> {
    let Some(topic_row) = TopicRow::fetch(metadata_tables, TopicRow::key_for(namespace_id, topic))?
    else {
        return Ok(StreamEstimate::default());
    };

    let mut total = 0u64;
    let mut available_partitions = Vec::new();
    for partition_idx in 0..topic_row.partitions {
        let partition = Partition::new(partition_idx)?;

        let cursor = StreamLeaseRow::fetch(
            metadata_tables,
            StreamLeaseRow::key_for(topic_row.id, partition, consumer_group),
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
}

impl AllNodesWorker {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    async fn worker_loop(&self) -> BackgroundResult<()> {
        let state = self.state.clone();
        match diom_core::task::spawn_blocking_in_current_span(move || {
            record_topic_lag_metrics(&state)
        })
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(err)) => tracing::warn!(?err, "failed to collect stream lag metrics"),
            Err(err) => tracing::warn!(?err, "stream lag metrics task panicked"),
        }
        Ok(())
    }
}

impl diom_operations::workers::BackgroundWorker for AllNodesWorker {
    const NAME: &'static str = "bg-worker:msgs";

    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(std::time::Duration::from_secs(60));
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
