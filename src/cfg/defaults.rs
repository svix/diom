use std::net::{Ipv4Addr, SocketAddr};

use coyote_core::types::DurationMs;

use super::DatabaseConfig;

pub(super) fn default_false() -> bool {
    false
}

pub(super) fn default_true() -> bool {
    true
}

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

pub(super) fn opentelemetry_metrics_period() -> DurationMs {
    DurationMs::from_secs(10)
}

pub(super) fn cluster_name() -> String {
    "coyote".to_owned()
}

pub(super) fn cluster_replication_request_timeout() -> DurationMs {
    DurationMs::from_secs(5)
}

pub(super) fn cluster_discovery_request_timeout() -> DurationMs {
    DurationMs::from_secs(10)
}

pub(super) fn cluster_discovery_timeout() -> DurationMs {
    DurationMs::from_secs(30)
}

pub(super) fn cluster_startup_discovery_delay() -> DurationMs {
    DurationMs::from(10)
}

pub(super) fn cluster_connection_timeout() -> DurationMs {
    DurationMs::from(3100)
}

pub(super) fn cluster_heartbeat_interval() -> DurationMs {
    DurationMs::from(500)
}

pub(super) fn cluster_election_timeout_min() -> DurationMs {
    DurationMs::from(1500)
}

pub(super) fn cluster_election_timeout_max() -> DurationMs {
    DurationMs::from(3500)
}

pub(super) fn cluster_auto_initialize() -> bool {
    true
}

pub(super) fn log_index_interval() -> DurationMs {
    DurationMs::from_mins(10)
}

pub(super) fn cluster_snapshot_after_time() -> Option<DurationMs> {
    Some(DurationMs::from_mins(15))
}

pub(super) fn cluster_log_sync_interval_commits() -> usize {
    0
}

pub(super) fn cluster_log_sync_interval_duration() -> DurationMs {
    DurationMs::from(10)
}

pub(super) fn cluster_send_snapshot_timeout() -> DurationMs {
    DurationMs::from_secs(30)
}

pub(super) fn cluster_replication_lag_threshold() -> u64 {
    50_000
}

pub(super) fn background_cleanup_interval() -> DurationMs {
    DurationMs::from_secs(10)
}
