use http::StatusCode;
use opentelemetry::{
    KeyValue,
    metrics::{Counter, Gauge, Histogram, Meter},
};

use super::cluster::NodeId;

pub enum DbType {
    Persistent,
    Ephemeral,
}

impl From<DbType> for opentelemetry::Value {
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
    apply_operations: Gauge<u64>,
    apply_batch_size: Histogram<u64>,
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
            apply_operations: meter
                .u64_gauge("diom.raft.apply_count")
                .with_description("Raft apply operations")
                .build(),
            apply_batch_size: meter
                .u64_histogram("diom.raft.apply_batch_size")
                .with_description("Raft apply operation batch sizes")
                .build(),
            node_id_kv: node_id.into(),
        }
    }

    pub fn record_apply(&self, batch_size: usize) {
        self.apply_operations
            .record(1, std::slice::from_ref(&self.node_id_kv));
        self.apply_batch_size
            .record(batch_size as u64, std::slice::from_ref(&self.node_id_kv));
    }

    pub fn bytes_used(&self, bytes: u64, db_type: DbType) {
        self.bytes_used
            .record(bytes, &[self.node_id_kv.clone(), db_type.into()]);
    }
}

#[derive(Clone)]
pub struct LogMetrics {
    bytes_used: Gauge<u64>,
    entry_count: Gauge<u64>,

    append_operations: Gauge<u64>,
    append_batch_size: Histogram<u64>,

    read_operations: Gauge<u64>,
    read_batch_size: Histogram<u64>,

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
            append_operations: meter
                .u64_gauge("diom.raft.log.append_count")
                .with_description("Raft log append operations")
                .build(),
            append_batch_size: meter
                .u64_histogram("diom.raft.log.append_batch_size")
                .with_description("Raft log append operation batch sizes")
                .build(),
            read_operations: meter
                .u64_gauge("diom.raft.log.read_count")
                .with_description("Raft log read operations")
                .build(),
            read_batch_size: meter
                .u64_histogram("diom.raft.log.read_batch_size")
                .with_description("Raft log read operation batch sizes")
                .build(),
            context,
        }
    }

    pub fn record_append(&self, batch_size: usize) {
        self.append_operations.record(1, &self.context);
        self.append_batch_size
            .record(batch_size as u64, &self.context);
    }

    pub fn record_log_read(&self, batch_size: usize) {
        self.read_operations.record(1, &self.context);
        self.read_batch_size
            .record(batch_size as u64, &self.context);
    }

    pub fn bytes_used(&self, bytes: u64) {
        self.bytes_used.record(bytes, &self.context);
    }

    pub fn entry_count(&self, count: u64) {
        self.entry_count.record(count, &self.context);
    }
}

pub enum ConnectionType {
    Internal,
    Interserver,
    External,
    Unknown,
}

impl From<ConnectionType> for opentelemetry::Value {
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
                .with_unit("ms")
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
        duration: u64,
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

        self.latency.record(duration, attrs);

        if let Some(cl) = content_length {
            self.content_length.record(cl, attrs);
        }
    }
}
