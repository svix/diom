use std::time::Duration;

use http::StatusCode;
use opentelemetry::{
    KeyValue, Value,
    metrics::{Counter, Gauge, Histogram, Meter},
};

use super::cluster::NodeId;

#[derive(Debug, Clone, Copy)]
pub enum DbType {
    Persistent,
    Ephemeral,
}

impl From<DbType> for Value {
    fn from(db_type: DbType) -> Self {
        match db_type {
            DbType::Persistent => "persistent".into(),
            DbType::Ephemeral => "ephemeral".into(),
        }
    }
}

impl From<DbType> for KeyValue {
    fn from(db_type: DbType) -> Self {
        KeyValue::new("db_type", db_type)
    }
}

#[derive(Clone)]
pub struct DbMetrics {
    bytes_used: Gauge<u64>,
    cache_capacity: Gauge<u64>,
    cache_size: Gauge<u64>,
    compactions: Gauge<u64>,
    active_compactions: Gauge<u64>,
    compaction_time: Gauge<u64>,
    outstanding_flushes: Gauge<u64>,

    apply_latency: Histogram<u64>,
    snapshot_operations: Counter<u64>,
    snapshot_size: Histogram<u64>,
    snapshot_latency: Histogram<u64>,

    node_id_kv: KeyValue,
}

impl DbMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            bytes_used: meter
                .u64_gauge("diom.db.bytes_used")
                .with_description("DB size in bytes")
                .with_unit("By")
                .build(),
            cache_capacity: meter
                .u64_gauge("diom.db.cache_capacity")
                .with_description("DB cache capacity in bytes")
                .with_unit("By")
                .build(),
            cache_size: meter
                .u64_gauge("diom.db.cache_size")
                .with_description("DB used cache size in bytes")
                .with_unit("By")
                .build(),
            compactions: meter
                .u64_gauge("diom.db.compactions")
                .with_description("Number of compactions performed since boot")
                .build(),
            active_compactions: meter
                .u64_gauge("diom.db.active_compactions")
                .with_description("Number of compactions currently in progress")
                .build(),
            compaction_time: meter
                .u64_gauge("diom.db.compaction_time")
                .with_description("Total time spent on compaction since boot")
                .with_unit("ms")
                .build(),
            outstanding_flushes: meter
                .u64_gauge("diom.db.outstanding_flushes")
                .with_description("Number of outstanding flushes to the disk")
                .build(),
            apply_latency: meter
                .u64_histogram("diom.raft.apply_latency")
                .with_description("Raft apply operation latencies")
                .with_unit("us")
                .build(),
            snapshot_operations: meter
                .u64_counter("diom.raft.snapshot_count")
                .with_description("Raft snapshots built")
                .build(),
            snapshot_size: meter
                .u64_histogram("diom.raft.snapshot_size")
                .with_description("Raft snapshot sizes")
                .with_unit("By")
                .build(),
            snapshot_latency: meter
                .u64_histogram("diom.raft.snapshot_latency")
                .with_description("Raft snapshot build latency")
                .with_unit("ms")
                .build(),
            node_id_kv: node_id.into(),
        }
    }

    pub fn record_db(
        &self,
        db_type: DbType,
        database: &fjall::Database,
        fetch_size: bool,
    ) -> anyhow::Result<()> {
        let cache_capacity = database.cache_capacity();
        let cache_used = database.cache_size();
        let compactions = database.compactions_completed();
        let active_compactions = database.active_compactions();
        let outstanding_flushes = database.outstanding_flushes();

        let context = [self.node_id_kv.clone(), db_type.into()];
        self.cache_capacity.record(cache_capacity, &context);
        self.cache_size.record(cache_used, &context);
        self.compactions.record(compactions as _, &context);
        self.compaction_time
            .record(database.time_compacting().as_millis() as _, &context);
        self.active_compactions
            .record(active_compactions as _, &context);
        self.outstanding_flushes
            .record(outstanding_flushes as _, &context);

        if fetch_size {
            let bytes_used = database.disk_space()?;
            self.bytes_used.record(bytes_used, &context);
        }

        Ok(())
    }

    pub fn record_apply(&self, duration: Duration) {
        let context = std::slice::from_ref(&self.node_id_kv);
        self.apply_latency
            .record(duration.as_micros() as _, context);
    }

    pub fn record_snapshot(&self, bytes: u64, duration: Duration) {
        let context = std::slice::from_ref(&self.node_id_kv);
        self.snapshot_operations.add(1, context);
        self.snapshot_size.record(bytes, context);
        self.snapshot_latency
            .record(duration.as_millis() as _, context);
    }
}

#[derive(Clone)]
pub struct LogMetrics {
    bytes_used: Gauge<u64>,
    entry_count: Gauge<u64>,

    append_latency: Histogram<u64>,

    read_operations: Counter<u64>,
    read_batch_size: Histogram<u64>,

    sync_latency: Histogram<u64>,
    sync_entry_count: Histogram<u64>,

    context: Vec<KeyValue>,
}

impl LogMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        let context = vec![node_id.into()];
        Self {
            bytes_used: meter
                .u64_gauge("diom.raft.log.bytes_used")
                .with_description("Raft log DB size in bytes")
                .with_unit("By")
                .build(),
            entry_count: meter
                .u64_gauge("diom.raft.log.entry_count")
                .with_description("Raft log entry count")
                .build(),
            append_latency: meter
                .u64_histogram("diom.raft.log.append_latency")
                .with_description("Raft log append operation latency")
                .with_unit("us")
                .build(),
            read_operations: meter
                .u64_counter("diom.raft.log.read_count")
                .with_description("Raft log read operations")
                .build(),
            read_batch_size: meter
                .u64_histogram("diom.raft.log.read_batch_size")
                .with_description("Raft log read operation batch sizes")
                .build(),
            sync_latency: meter
                .u64_histogram("diom.raft.log.sync_latency")
                .with_description("Raft log sync latency")
                .with_unit("us")
                .build(),
            sync_entry_count: meter
                .u64_histogram("diom.raft.log.sync_entry_count")
                .with_description("Number of entries included in a log sync")
                .with_unit("message")
                .build(),
            context,
        }
    }

    pub fn record_append(&self, duration: Duration) {
        self.append_latency
            .record(duration.as_micros() as _, &self.context);
    }

    pub fn record_log_read(&self, batch_size: usize) {
        self.read_operations.add(1, &self.context);
        self.read_batch_size
            .record(batch_size as u64, &self.context);
    }

    pub fn bytes_used(&self, bytes: u64) {
        self.bytes_used.record(bytes, &self.context);
    }

    pub fn entry_count(&self, count: u64) {
        self.entry_count.record(count, &self.context);
    }

    pub fn record_fsync(&self, duration: Duration, entries: usize) {
        self.sync_latency
            .record(duration.as_micros() as _, &self.context);
        self.sync_entry_count.record(entries as _, &self.context);
    }
}

#[derive(Clone, Copy)]
pub enum WriteType {
    Local,
    Forwarded,
}

impl From<WriteType> for Value {
    fn from(value: WriteType) -> Self {
        match value {
            WriteType::Local => "local".into(),
            WriteType::Forwarded => "forwarded".into(),
        }
    }
}

impl From<WriteType> for KeyValue {
    fn from(value: WriteType) -> Self {
        KeyValue::new("write_type", value)
    }
}

#[derive(Clone)]
pub struct ClusterMetrics {
    writes: Counter<u64>,
    write_latency: Histogram<u64>,
    linearizable_reads: Counter<u64>,

    context: Vec<KeyValue>,
}

impl ClusterMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        let context = vec![node_id.into()];
        Self {
            writes: meter
                .u64_counter("diom.raft.writes")
                .with_description("The number of writes handled")
                .with_unit("message")
                .build(),
            write_latency: meter
                .u64_histogram("diom.raft.write_latency")
                .with_description("Latency of client_write calls")
                .with_unit("us")
                .build(),

            linearizable_reads: meter
                .u64_counter("diom.raft.linearizable_reads")
                .with_description("The number of linearizable reads performed")
                .with_unit("message")
                .build(),

            context,
        }
    }

    pub fn record_write(&self, write_type: WriteType, duration: Duration) {
        let mut context = self.context.clone();
        context.push(write_type.into());
        self.writes.add(1, &context);
        self.write_latency
            .record(duration.as_micros() as _, &context);
    }

    pub fn record_linearizable_read(&self) {
        self.linearizable_reads.add(1, &self.context);
    }
}

#[derive(Clone, Copy)]
pub enum ConnectionType {
    Internal,
    Interserver,
    External,
    Unknown,
}

impl From<ConnectionType> for Value {
    fn from(value: ConnectionType) -> Self {
        match value {
            ConnectionType::Internal => "internal",
            ConnectionType::Interserver => "interserver",
            ConnectionType::External => "external",
            ConnectionType::Unknown => "unknown",
        }
        .into()
    }
}

impl From<ConnectionType> for KeyValue {
    fn from(value: ConnectionType) -> Self {
        KeyValue::new("connection_type", value)
    }
}

#[derive(Clone)]
pub struct ConnectionMetrics {
    pub total: Counter<u64>,
    node_id_kv: KeyValue,
}

impl ConnectionMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            total: meter
                .u64_counter("diom.connections.total")
                .with_description("Total number of accepted connections")
                .build(),
            node_id_kv: node_id.into(),
        }
    }

    pub fn accepted(&self, t: ConnectionType) {
        self.total.add(1, &[self.node_id_kv.clone(), t.into()]);
    }
}

#[derive(Clone)]
pub struct RequestMetrics {
    success: Counter<u64>,
    client_error: Counter<u64>,
    server_error: Counter<u64>,
    latency: Histogram<u64>,
    content_length: Histogram<u64>,
    node_id_kv: KeyValue,
    connection_type_kv: KeyValue,
}

impl RequestMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            success: meter
                .u64_counter("diom.request.success")
                .with_description("Count of successful requests")
                .build(),
            client_error: meter
                .u64_counter("diom.request.client_error")
                .with_description("Count of client errors")
                .build(),
            server_error: meter
                .u64_counter("diom.request.server_error")
                .with_description("Count of server errors")
                .build(),
            latency: meter
                .u64_histogram("diom.request.duration")
                .with_description("Request latency")
                .with_unit("us")
                .build(),
            content_length: meter
                .u64_histogram("diom.request.content_length")
                .with_description("Content length")
                .with_unit("By")
                .build(),
            node_id_kv: node_id.into(),
            connection_type_kv: ConnectionType::Unknown.into(),
        }
    }

    /// Make a clone of this RequestMetrics (sharing the same meters) with the given
    /// ConnectionType as context
    pub fn with_connection_type(&self, connection_type: ConnectionType) -> Self {
        let mut cloned = self.clone();
        cloned.connection_type_kv = connection_type.into();
        cloned
    }

    pub fn record(
        &self,
        route: &str,
        status: StatusCode,
        duration: Duration,
        content_length: Option<u64>,
    ) {
        let attrs = &[
            KeyValue::new("route", route.to_owned()),
            self.node_id_kv.clone(),
        ];

        if status.is_success() {
            self.success.add(1, attrs);
        } else if status.is_server_error() {
            self.server_error.add(1, attrs);
        } else {
            self.client_error.add(1, attrs);
        }

        self.latency.record(duration.as_micros() as _, attrs);

        if let Some(cl) = content_length {
            self.content_length.record(cl, attrs);
        }
    }
}

#[derive(Clone, Debug)]
pub struct OpenraftMetrics {
    apply_batch_calls: Counter<u64>,
    apply_batch_entry_count: Histogram<u64>,

    append_batch_calls: Counter<u64>,
    append_batch_entry_count: Histogram<u64>,

    write_batch_calls: Counter<u64>,
    write_batch_entry_count: Histogram<u64>,

    term: Gauge<u64>,
    last_log_index: Gauge<u64>,
    last_applied_log_index: Gauge<u64>,
    last_committed_log_index: Gauge<u64>,
    snapshot_index: Gauge<u64>,
    purged_log_index: Gauge<u64>,
    server_state: Gauge<u64>,

    votes: Counter<u64>,
    heartbeats: Counter<u64>,
    appends: Counter<u64>,

    context: Vec<KeyValue>,
}

impl OpenraftMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            apply_batch_calls: meter
                .u64_counter("diom.raft.apply_count")
                .with_description("apply_batch operations")
                .build(),
            apply_batch_entry_count: meter
                .u64_histogram("diom.raft.apply_batch_size")
                .with_description("apply_batch entry counts")
                .with_unit("message")
                .build(),
            append_batch_calls: meter
                .u64_counter("diom.raft.log.append_count")
                .with_description("append_batch operations")
                .build(),
            append_batch_entry_count: meter
                .u64_histogram("diom.raft.log.append_batch_size")
                .with_description("append_batch entry counts")
                .with_unit("message")
                .build(),
            write_batch_calls: meter
                .u64_counter("diom.raft.write_batch")
                .with_description("write_batch operations")
                .build(),
            write_batch_entry_count: meter
                .u64_histogram("diom.raft.write_batch_entry_count")
                .with_description("write_batch entry counts")
                .with_unit("message")
                .build(),

            term: meter
                .u64_gauge("diom.raft.term")
                .with_description("Current term")
                .build(),
            last_log_index: meter
                .u64_gauge("diom.raft.last_log_index")
                .with_description("Current log index")
                .build(),
            last_applied_log_index: meter
                .u64_gauge("diom.raft.last_applied_log_index")
                .with_description("Most-recently-applied log index")
                .build(),
            last_committed_log_index: meter
                .u64_gauge("diom.raft.last_committed_log_index")
                .with_description("Most-recently-committed log index")
                .build(),
            snapshot_index: meter
                .u64_gauge("diom.raft.snapshot_index")
                .with_description("Current snapshot index")
                .build(),
            purged_log_index: meter
                .u64_gauge("diom.raft.purged_log_index")
                .with_description("Purged log index")
                .build(),
            server_state: meter
                .u64_gauge("diom.raft.server_state")
                .with_description("Current server state")
                .build(),
            votes: meter
                .u64_counter("diom.raft.votes")
                .with_description("vote operations")
                .build(),
            heartbeats: meter
                .u64_counter("diom.raft.heartbeats")
                .with_description("heartbeat operations")
                .build(),
            appends: meter
                .u64_counter("diom.raft.appends")
                .with_description("append operations")
                .build(),

            context: vec![node_id.into()],
        }
    }
}

impl openraft::metrics::MetricsRecorder for OpenraftMetrics {
    fn record_apply_batch(&self, entry_count: u64) {
        self.apply_batch_calls.add(1, &self.context);
        self.apply_batch_entry_count
            .record(entry_count, &self.context);
    }

    fn record_append_batch(&self, entry_count: u64) {
        self.append_batch_calls.add(1, &self.context);
        self.append_batch_entry_count
            .record(entry_count, &self.context);
    }

    fn record_write_batch(&self, entry_count: u64) {
        self.write_batch_calls.add(1, &self.context);
        self.write_batch_entry_count
            .record(entry_count, &self.context);
    }

    fn set_current_term(&self, term: u64) {
        self.term.record(term, &self.context);
    }

    fn set_last_log_index(&self, index: u64) {
        self.last_log_index.record(index, &self.context);
    }

    fn set_applied_index(&self, index: u64) {
        self.last_applied_log_index.record(index, &self.context);
    }

    fn set_committed_index(&self, index: u64) {
        self.last_committed_log_index.record(index, &self.context);
    }

    fn set_snapshot_index(&self, index: u64) {
        self.snapshot_index.record(index, &self.context);
    }

    fn set_purged_index(&self, index: u64) {
        self.purged_log_index.record(index, &self.context);
    }

    fn set_server_state(&self, state: u8) {
        self.server_state.record(state.into(), &self.context);
    }

    fn increment_vote(&self) {
        self.votes.add(1, &self.context);
    }

    fn increment_heartbeat(&self) {
        self.heartbeats.add(1, &self.context);
    }

    fn increment_append(&self) {
        self.appends.add(1, &self.context);
    }
}
