use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use super::DatabaseConfig;

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

pub(super) fn opentelemetry_metrics_period() -> Duration {
    Duration::from_secs(10)
}

pub(super) fn cluster_name() -> String {
    "coyote".to_owned()
}

pub(super) fn cluster_replication_request_timeout() -> Duration {
    Duration::from_secs(5)
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

pub(super) fn cluster_heartbeat_interval() -> Duration {
    Duration::from_millis(500)
}

pub(super) fn cluster_election_timeout_min() -> Duration {
    Duration::from_millis(1500)
}

pub(super) fn cluster_election_timeout_max() -> Duration {
    Duration::from_millis(3500)
}

pub(super) fn cluster_auto_initialize() -> bool {
    true
}

pub(super) fn log_index_interval() -> Duration {
    Duration::from_mins(10)
}

pub(super) fn cluster_snapshot_after_time() -> Option<Duration> {
    Some(Duration::from_mins(15))
}

pub(super) fn cluster_send_snapshot_timeout() -> Duration {
    Duration::from_secs(30)
}

pub(super) fn cluster_replication_lag_threshold() -> u64 {
    50_000
}

pub(super) fn background_cleanup_interval() -> Duration {
    Duration::from_secs(10)
}
