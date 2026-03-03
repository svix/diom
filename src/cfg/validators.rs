use std::time::Duration;

use validator::ValidationError;

use crate::cfg::ClusterConfiguration;

pub(super) fn validate_log_sync_interval_duration(d: &Duration) -> Result<(), ValidationError> {
    if d.is_zero() {
        return Err(ValidationError::new(
            "sync interval duration must be non-zero",
        ));
    }
    Ok(())
}

pub(super) fn validate_cluster_configuration(
    c: &ClusterConfiguration,
) -> Result<(), ValidationError> {
    if !(c.log_ack_immediately || c.log_sync_interval_commits == 1) {
        return Err(ValidationError::new(
            "If log_sync_interval_commits != 1, log_ack_immediately must be true (until openraft 0.10)",
        ));
    }
    if c.election_timeout_min >= c.election_timeout_max {
        return Err(ValidationError::new(
            "election_timeout_min must be < election_timeout_max",
        ));
    }
    if c.heartbeat_interval < c.replication_request_timeout {
        return Err(ValidationError::new(
            "heartbeat_interval must be >= replication_request_teimout",
        ));
    }
    Ok(())
}
