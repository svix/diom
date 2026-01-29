use crate::{
    State,
    entities::{ConsumerGroup, MsgId, StreamName},
    tables::{LeaseDiff, LeaseRow, NameToStreamRow},
};
use diom_error::{HttpError, Result};
use jiff::Timestamp;

pub struct Ack {
    lease_diff: LeaseDiff,
}

pub struct AckOutput {}

impl Ack {
    pub fn new(
        state: &State,
        name: StreamName,
        cg: ConsumerGroup,
        min_msg_id: MsgId,
        max_msg_id: MsgId,
    ) -> Result<Self> {
        let stream_id = NameToStreamRow::get_stream_id(state, &name)?;
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;
        validate_ack_bounds(&leases, max_msg_id)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);
        // This new lease is potentially redundant with an extant lease.
        // However, any redundancy will be removed by future calls to `cull_and_compact`.
        lease_diff.to_insert.push(LeaseRow {
            stream_id,
            cg,
            block_start: min_msg_id,
            block_end: max_msg_id,
            leased_at: now,
            expires_at: Timestamp::MAX,
            acked_at: Some(now),
        });

        Ok(Self { lease_diff })
    }

    pub fn apply_operation(self, state: &State) -> Result<AckOutput> {
        let mut batch = state.db.batch();
        self.lease_diff.apply_diff(state, &mut batch)?;
        batch.commit()?;
        Ok(AckOutput {})
    }
}

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
