use opentelemetry::metrics::{Gauge, Meter};

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

pub struct DbMetrics {
    bytes_used: Gauge<u64>,
    node_id: NodeId,
}

impl DbMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
        Self {
            bytes_used: meter
                .u64_gauge("diom.db.bytes_used")
                .with_description("DB size in bytes")
                .with_unit("By")
                .build(),
            node_id,
        }
    }

    pub fn bytes_used(&self, bytes: u64, db_type: DbType) {
        self.bytes_used.record(
            bytes,
            &[
                opentelemetry::KeyValue::new("node_id", self.node_id.to_string()),
                opentelemetry::KeyValue::new("db_type", db_type),
            ],
        );
    }
}

pub struct LogMetrics {
    bytes_used: Gauge<u64>,
    entry_count: Gauge<u64>,
    node_id: NodeId,
}

impl LogMetrics {
    pub fn new(meter: &Meter, node_id: NodeId) -> Self {
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
            node_id,
        }
    }

    pub fn bytes_used(&self, bytes: u64) {
        self.bytes_used.record(
            bytes,
            &[opentelemetry::KeyValue::new(
                "node_id",
                self.node_id.to_string(),
            )],
        );
    }

    pub fn entry_count(&self, count: u64) {
        self.entry_count.record(
            count,
            &[opentelemetry::KeyValue::new(
                "node_id",
                self.node_id.to_string(),
            )],
        );
    }
}
