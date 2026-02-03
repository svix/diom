use std::num::NonZeroU16;

use coyote_error::Result;
use jiff::Timestamp;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, MsgOut, StreamName},
    tables::{LeaseDiff, LeaseRow, MsgRow, NameToStreamRow},
};

pub struct Fetch {
    lease_diff: LeaseDiff,
    msgs: Vec<(MsgId, MsgRow)>,
}

pub struct FetchOutput {
    pub msgs: Vec<MsgOut>,
}

impl Fetch {
    pub fn new(
        state: &State,
        name: StreamName,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        visibility_timeout: std::time::Duration,
    ) -> Result<Self> {
        let stream_id = NameToStreamRow::get_stream_id(state, &name)?;
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;

        // Unlike FetchLocking, we don't block on active leases.
        // Instead, we exclude both acked and active leases from the fetch.
        let blocked_leases = leases
            .iter()
            .filter(|lease| lease.acked_at.is_some() || lease.is_active(now));

        let msgs = MsgRow::fetch_available(state, stream_id, blocked_leases, batch_size.into())?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

        if !msgs.is_empty() {
            lease_diff.to_insert.push(LeaseRow {
                stream_id,
                cg,
                block_start: msgs.first().unwrap().0,
                block_end: msgs.last().unwrap().0,
                leased_at: now,
                expires_at: now + visibility_timeout,
                acked_at: None,
            });
        }

        Ok(Self { lease_diff, msgs })
    }

    pub fn apply_operation(self, state: &State) -> Result<FetchOutput> {
        let mut batch = state.db.batch();
        self.lease_diff.apply_diff(state, &mut batch)?;
        batch.commit()?;

        let msgs = self
            .msgs
            .into_iter()
            .map(|(id, msg)| MsgOut {
                id,
                payload: msg.payload,
                headers: msg.headers,
                timestamp: msg.created_at,
            })
            .collect();

        Ok(FetchOutput { msgs })
    }
}
