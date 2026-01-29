use std::num::NonZeroU16;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, MsgOut, StreamId},
    tables::{LeaseDiff, LeaseRow, MsgRow},
};
use diom_error::{HttpError, Result};
use jiff::Timestamp;

pub struct FetchLocking {
    lease_diff: LeaseDiff,
    msgs: Vec<(MsgId, MsgRow)>,
}

pub struct FetchLockingOutput {
    pub msgs: Vec<MsgOut>,
}

impl FetchLocking {
    pub fn new(
        state: &State,
        stream_id: StreamId,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        visibility_timeout: std::time::Duration,
    ) -> Result<Self> {
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;

        let has_active_lease = leases.iter().any(|lease| lease.is_active(now));

        if has_active_lease {
            return Err(HttpError::bad_request(
                Some("consumer_group_locked".to_owned()),
                Some("Concurrent reads from the same consumer group".to_string()),
            )
            .into());
        }

        let acked_leases = leases.iter().filter(|lease| lease.acked_at.is_some());
        let msgs = MsgRow::fetch_available(state, stream_id, acked_leases, batch_size.into())?;

        if msgs.is_empty() {
            // FIXME(@svix-gabriel) this isn't really an error, but we need to go back
            // and change any HttpErrors anyway, so this is simpler for now.
            return Err(HttpError::bad_request(
                Some("empty_stream".to_owned()),
                Some("no messages available".to_string()),
            )
            .into());
        }

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

        lease_diff.to_insert.push(LeaseRow {
            stream_id,
            cg,
            block_start: msgs.first().unwrap().0,
            block_end: msgs.last().unwrap().0,
            leased_at: now,
            expires_at: now + visibility_timeout,
            acked_at: None,
        });

        Ok(Self { lease_diff, msgs })
    }

    pub fn apply_operation(self, state: &State) -> Result<FetchLockingOutput> {
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

        Ok(FetchLockingOutput { msgs })
    }
}
