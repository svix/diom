use validator::ValidationError;

use crate::cfg::ClusterConfiguration;
use diom_core::types::NonZeroDurationMs;

pub(super) fn validate_admin_token(token: &str) -> Result<(), ValidationError> {
    if token.len() < 20 {
        return Err(ValidationError::new(
            "admin_token must be at least 20 characters long",
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

    if c.log_sync_interval_commits > 1
        && c.log_sync_interval_duration > NonZeroDurationMs::from_millis(100).unwrap()
    {
        tracing::warn!(
            "setting log_sync_interval_commits > 1 and a long log_sync_interval_duration will make bootstrap extremely slow!"
        )
    }

    Ok(())
}
