use super::{DlqResponse, StreamRaftState, StreamRequest};
use crate::{
    State,
    entities::{ConsumerGroup, MsgId},
    tables::LeaseRow,
};
use diom_configgroup::entities::ConfigGroupId;
use diom_error::HttpError;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

fn validate_dlq_bounds(leases: &[LeaseRow], msg_id: MsgId) -> diom_error::Result<()> {
    let highest_bound = leases.iter().map(|l| l.block_end).max().ok_or_else(|| {
        HttpError::bad_request(
            Some("invalid_dlq".to_owned()),
            Some("No leases exist for this consumer group".to_owned()),
        )
    })?;

    if msg_id > highest_bound {
        return Err(HttpError::bad_request(
            Some("invalid_dlq".to_owned()),
            Some(format!(
                "DLQ message id exceeds highest lease bound. msg_id={msg_id}, highest_bound={highest_bound}"
            )),
        )
        .into());
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlqOperation {
    group_id: ConfigGroupId,
    cg: ConsumerGroup,
    msg_id: MsgId,
}

impl DlqOperation {
    pub fn new(group_id: ConfigGroupId, cg: ConsumerGroup, msg_id: MsgId) -> Self {
        Self {
            group_id,
            cg,
            msg_id,
        }
    }

    fn apply_real(self, state: &State) -> diom_operations::Result<DlqResponseData> {
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, self.group_id, &self.cg)?;

        validate_dlq_bounds(&leases, self.msg_id)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases.clone(), now);

        // Shrink any active leases that cover this message
        LeaseRow::shrink_active_leases_for_range(
            &leases,
            self.msg_id,
            self.msg_id,
            now,
            &mut lease_diff,
        );

        lease_diff.to_insert.push(LeaseRow {
            group_id: self.group_id,
            cg: self.cg,
            block_start: self.msg_id,
            block_end: self.msg_id,
            leased_at: now,
            expires_at: Timestamp::MAX,
            acked_at: None,
            dlq_at: Some(now),
        });

        let mut batch = state.db.batch();
        lease_diff.apply_diff(state, &mut batch)?;
        batch.commit().map_err(diom_error::Error::from)?;

        Ok(DlqResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlqResponseData {}

impl StreamRequest for DlqOperation {
    fn apply(self, state: StreamRaftState<'_>) -> DlqResponse {
        DlqResponse(self.apply_real(state.stream))
    }
}
