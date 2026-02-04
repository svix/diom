use crate::{
    State,
    entities::{ConsumerGroup, MsgId, StreamName},
    tables::{LeaseDiff, LeaseRow, NameToStreamRow},
};
use coyote_error::{HttpError, Result};
use jiff::Timestamp;

pub struct Dlq {
    lease_diff: LeaseDiff,
}

pub struct DlqOutput {}

impl Dlq {
    pub fn new(state: &State, name: StreamName, cg: ConsumerGroup, msg_id: MsgId) -> Result<Self> {
        let stream_id = NameToStreamRow::get_stream_id(state, &name)?;
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;

        validate_dlq_bounds(&leases, msg_id)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases.clone(), now);

        // Shrink any active leases that cover this message
        LeaseRow::shrink_active_leases_for_range(&leases, msg_id, msg_id, now, &mut lease_diff);

        lease_diff.to_insert.push(LeaseRow {
            stream_id,
            cg,
            block_start: msg_id,
            block_end: msg_id,
            leased_at: now,
            expires_at: Timestamp::MAX,
            acked_at: None,
            dlq_at: Some(now),
        });

        Ok(Self { lease_diff })
    }

    pub fn apply_operation(self, state: &State) -> Result<DlqOutput> {
        let mut batch = state.db.batch();
        self.lease_diff.apply_diff(state, &mut batch)?;
        batch.commit()?;
        Ok(DlqOutput {})
    }
}

fn validate_dlq_bounds(leases: &[LeaseRow], msg_id: MsgId) -> Result<()> {
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
