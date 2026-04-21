use std::net::{Ipv6Addr, SocketAddr};

use diom_core::types::{DurationMs, NonZeroDurationMs};

use super::{DatabaseConfig, MemorySize};

pub(super) fn default_true() -> bool {
    true
}

pub(super) fn listen_address() -> SocketAddr {
    (Ipv6Addr::UNSPECIFIED, 8624).into()
}

pub(super) fn cluster_listen_address() -> SocketAddr {
    (Ipv6Addr::UNSPECIFIED, 8625).into()
}

pub(super) fn persistent_db() -> DatabaseConfig {
    DatabaseConfig {
        path: "./db".into(),
        filename: "fjall_persistent".into(),
        cache_size: default_database_size(),
    }
}

pub(super) fn ephemeral_db() -> DatabaseConfig {
    DatabaseConfig {
        path: "./db".into(),
        filename: "fjall_ephemeral".into(),
        cache_size: default_database_size(),
    }
}

pub(super) fn opentelemetry_service_name() -> String {
    "diom".into()
}

pub(super) const fn opentelemetry_metrics_period() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(10).unwrap()
}

pub(super) fn cluster_name() -> String {
    "diom".to_owned()
}

pub(super) const fn cluster_replication_request_timeout() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(5).unwrap()
}

pub(super) const fn cluster_discovery_request_timeout() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(10).unwrap()
}

pub(super) const fn cluster_discovery_timeout() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(30).unwrap()
}

pub(super) const fn cluster_startup_discovery_delay() -> DurationMs {
    DurationMs::from_millis(10)
}

pub(super) const fn cluster_connection_timeout() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(3100).unwrap()
}

pub(super) const fn cluster_heartbeat_interval() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(500).unwrap()
}

pub(super) const fn cluster_election_timeout_min() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(1500).unwrap()
}

pub(super) const fn cluster_election_timeout_max() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(3500).unwrap()
}

pub(super) fn cluster_auto_initialize() -> bool {
    true
}

pub(super) const fn log_index_interval() -> NonZeroDurationMs {
    NonZeroDurationMs::from_mins(10).unwrap()
}

pub(super) const fn cluster_snapshot_after_time() -> Option<NonZeroDurationMs> {
    NonZeroDurationMs::from_mins(15)
}

pub(super) fn cluster_log_sync_interval_commits() -> usize {
    0
}

pub(super) const fn cluster_log_sync_interval_duration() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(2).unwrap()
}

pub(super) const fn cluster_send_snapshot_timeout() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(30).unwrap()
}

pub(super) fn cluster_replication_lag_threshold() -> u64 {
    50_000
}

pub(super) const fn background_cleanup_interval() -> NonZeroDurationMs {
    NonZeroDurationMs::from_secs(10).unwrap()
}

pub(super) fn default_database_size() -> MemorySize {
    MemorySize::Percent(20)
}
