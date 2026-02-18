use super::{AckResponse, StreamRaftState, StreamRequest};
use diom_configgroup::entities::ConfigGroupId;
use diom_error::{HttpError, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId},
    tables::LeaseRow,
};

fn validate_ack_bounds(leases: &[LeaseRow], max_msg_id: MsgId) -> Result<()> {
    let highest_bound = leases.iter().map(|l| l.block_end).max().ok_or_else(|| {
        HttpError::bad_request(
            Some("invalid_ack".to_owned()),
            Some("No leases exist for this consumer group".to_owned()),
        )
    })?;

    if max_msg_id > highest_bound {
        return Err(HttpError::bad_request(
            Some("invalid_ack".to_owned()),
            Some(format!(
                "Ack range exceeds highest lease bound. max_msg_id={max_msg_id}, highest_bound={highest_bound}"
            )),
        )
        .into());
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckOperation {
    group_id: ConfigGroupId,
    cg: ConsumerGroup,
    min_msg_id: MsgId,
    max_msg_id: MsgId,
}

impl AckOperation {
    pub fn new(
        group_id: ConfigGroupId,
        cg: ConsumerGroup,
        min_msg_id: MsgId,
        max_msg_id: MsgId,
    ) -> Self {
        Self {
            group_id,
            cg,
            min_msg_id,
            max_msg_id,
        }
    }

    fn apply_real(self, state: &State) -> diom_operations::Result<AckResponseData> {
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, self.group_id, &self.cg)?;
        validate_ack_bounds(&leases, self.max_msg_id)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases.clone(), now);

        // Shrink any active leases that overlap with the acked range
        LeaseRow::shrink_active_leases_for_range(
            &leases,
            self.min_msg_id,
            self.max_msg_id,
            now,
            &mut lease_diff,
        );

        // This new lease is potentially redundant with an extant lease.
        // However, any redundancy will be removed by future calls to `cull_and_compact`.
        lease_diff.to_insert.push(LeaseRow {
            group_id: self.group_id,
            cg: self.cg,
            block_start: self.min_msg_id,
            block_end: self.max_msg_id,
            leased_at: now,
            expires_at: Timestamp::MAX,
            acked_at: Some(now),
            dlq_at: None,
        });

        let mut batch = state.db.batch();
        lease_diff.apply_diff(state, &mut batch)?;
        batch.commit().map_err(diom_error::Error::from)?;

        Ok(AckResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AckResponseData {}

impl StreamRequest for AckOperation {
    fn apply(self, state: StreamRaftState<'_>) -> AckResponse {
        AckResponse(self.apply_real(state.stream))
    }
}
