use std::{
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use super::DatabaseConfig;

pub(super) fn listen_address() -> SocketAddr {
    (Ipv4Addr::UNSPECIFIED, 8050).into()
}

pub(super) fn persistent_db() -> DatabaseConfig {
    DatabaseConfig {
        path: "./db".into(),
        filename: None,
    }
}

pub(super) fn ephemeral_db() -> DatabaseConfig {
    DatabaseConfig {
        path: "./db".into(),
        filename: None,
    }
}

pub(super) fn opentelemetry_service_name() -> String {
    "coyote".into()
}

pub(super) fn cluster_listen_address() -> SocketAddr {
    (Ipv4Addr::UNSPECIFIED, 18050).into()
}

pub(super) fn opentelemetry_metrics_period() -> u64 {
    60
}

pub(super) fn cluster_name() -> String {
    "coyote".to_owned()
}

pub(super) fn cluster_snapshot_path() -> PathBuf {
    "./snapshots".into()
}

pub(super) fn cluster_log_path() -> PathBuf {
    "./logs".into()
}

pub(super) fn cluster_replication_request_timeout() -> Duration {
    Duration::from_secs(30)
}

pub(super) fn cluster_discovery_request_timeout() -> Duration {
    Duration::from_secs(10)
}

pub(super) fn cluster_discovery_timeout() -> Duration {
    Duration::from_secs(30)
}

pub(super) fn cluster_startup_discovery_delay() -> Duration {
    Duration::from_millis(10)
}

pub(super) fn cluster_connection_timeout() -> Duration {
    Duration::from_millis(3100)
}

pub(super) fn cluster_heartbeat_interval_ms() -> u64 {
    500
}

pub(super) fn cluster_election_timeout_min_ms() -> u64 {
    1500
}

pub(super) fn cluster_election_timeout_max_ms() -> u64 {
    3000
}

pub(super) fn cluster_auto_initialize() -> bool {
    true
}
