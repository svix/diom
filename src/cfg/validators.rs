use diom_core::types::DurationMs;
use validator::ValidationError;

use crate::cfg::ClusterConfiguration;

pub(super) fn validate_admin_token(token: &str) -> Result<(), ValidationError> {
    if token.len() < 20 {
        return Err(ValidationError::new(
            "admin_token must be at least 20 characters long",
        ));
    }
    Ok(())
}

pub(super) fn validate_log_sync_interval_duration(d: &DurationMs) -> Result<(), ValidationError> {
    if *d == DurationMs::ZERO {
        return Err(ValidationError::new(
            "sync interval duration must be non-zero",
        ));
    }
    Ok(())
}

pub(super) fn validate_cluster_configuration(
    c: &ClusterConfiguration,
) -> Result<(), ValidationError> {
    if c.election_timeout_min >= c.election_timeout_max {
        return Err(ValidationError::new(
            "election_timeout_min must be < election_timeout_max",
        ));
    }
    if c.heartbeat_interval < c.replication_request_timeout {
        tracing::warn!(
            "setting replication_request_timeout greater than heartbeat_interval may cause election timeouts"
        );
    }
    Ok(())
}
