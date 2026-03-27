use std::fmt::{self, Display, Formatter};

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

#[derive(Clone)]
pub struct DbMetrics {
    bytes_used: Gauge<u64>,
    apply_operations: Gauge<u64>,
    apply_batch_size: Histogram<u64>,
    node_id: NodeId,
}

impl DbMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            bytes_used: meter
                .u64_gauge("coyote.db.bytes_used")
                .with_description("DB size in bytes")
                .with_unit("By")
                .build(),
            apply_operations: meter
                .u64_gauge("coyote.raft.apply_count")
                .with_description("Raft apply operations")
                .build(),
            apply_batch_size: meter
                .u64_histogram("coyote.raft.apply_batch_size")
                .with_description("Raft apply operation batch sizes")
                .build(),
            node_id,
        }
    }

    pub fn record_apply(&self, batch_size: usize) {
        self.apply_operations
            .record(1, &[KeyValue::new("node_id", self.node_id.to_string())]);
        self.apply_batch_size.record(
            batch_size as u64,
            &[KeyValue::new("node_id", self.node_id.to_string())],
        );
    }

    pub fn bytes_used(&self, bytes: u64, db_type: DbType) {
        self.bytes_used.record(
            bytes,
            &[
                KeyValue::new("node_id", self.node_id.to_string()),
                KeyValue::new("db_type", db_type),
            ],
        );
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
        let context = vec![KeyValue::new("node_id", node_id.to_string())];
        Self {
            bytes_used: meter
                .u64_gauge("coyote.raft.log.bytes_used")
                .with_description("Raft log DB size in bytes")
                .with_unit("By")
                .build(),
            entry_count: meter
                .u64_gauge("coyote.raft.log.entry_count")
                .with_description("Raft log entry count")
                .build(),
            append_operations: meter
                .u64_gauge("coyote.raft.log.append_count")
                .with_description("Raft log append operations")
                .build(),
            append_batch_size: meter
                .u64_histogram("coyote.raft.log.append_batch_size")
                .with_description("Raft log append operation batch sizes")
                .build(),
            read_operations: meter
                .u64_gauge("coyote.raft.log.read_count")
                .with_description("Raft log read operations")
                .build(),
            read_batch_size: meter
                .u64_histogram("coyote.raft.log.read_batch_size")
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
    External,
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = match self {
            ConnectionType::Internal => "internal",
            ConnectionType::External => "external",
        };
        write!(f, "{value}")
    }
}
pub struct ConnectionMetrics {
    pub total: Counter<u64>,
}

impl ConnectionMetrics {
    pub fn new(meter: &Meter) -> Self {
        Self {
            total: meter
                .u64_counter("coyote.connections.total")
                .with_description("Total number of accepted connections")
                .build(),
        }
    }

    pub fn accepted(&self, node_id: NodeId, t: ConnectionType) {
        self.total.add(
            1,
            &[
                KeyValue::new("node_id", node_id.to_string()),
                KeyValue::new("connection_type", t.to_string()),
            ],
        );
    }
}

pub struct RequestMetrics {
    success: Counter<u64>,
    client_error: Counter<u64>,
    server_error: Counter<u64>,
    latency: Histogram<u64>,
    content_length: Histogram<u64>,
}

impl RequestMetrics {
    pub fn new(meter: &Meter) -> Self {
        Self {
            success: meter
                .u64_counter("coyote.request.success")
                .with_description("Count of successful requests")
                .build(),
            client_error: meter
                .u64_counter("coyote.request.client_error")
                .with_description("Count of client errors")
                .build(),
            server_error: meter
                .u64_counter("coyote.request.server_error")
                .with_description("Count of server errors")
                .build(),
            latency: meter
                .u64_histogram("coyote.request.duration")
                .with_description("Request latency")
                .with_unit("ms")
                .build(),
            content_length: meter
                .u64_histogram("coyote.request.content_length")
                .with_description("Content length")
                .with_unit("By")
                .build(),
        }
    }

    pub fn record(
        &self,
        route: &str,
        node_id: NodeId,
        status: StatusCode,
        duration: u64,
        content_length: Option<u64>,
    ) {
        let attrs = &[
            KeyValue::new("route", route.to_owned()),
            KeyValue::new("node_id", node_id.to_string()),
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
