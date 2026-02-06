use std::num::NonZeroU16;

use coyote_configgroup::entities::ConfigGroupId;
use coyote_error::Result;
use jiff::Timestamp;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, MsgOut, StreamId},
    tables::{LeaseDiff, LeaseRow, MsgRow},
};

pub struct Fetch {
    lease_diff: LeaseDiff,
    msgs: Vec<(MsgId, MsgRow)>,
}

pub struct FetchOutput {
    pub msgs: Vec<MsgOut>,
}

/// Groups message IDs into contiguous ranges.
/// Returns a vector of (start, end) tuples representing each contiguous block.
fn group_into_contiguous_ranges(msg_ids: &[MsgId]) -> Vec<(MsgId, MsgId)> {
    if msg_ids.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut range_start = msg_ids[0];
    let mut range_end = msg_ids[0];

    for &id in &msg_ids[1..] {
        if id == range_end + 1 {
            // Contiguous with current range
            range_end = id;
        } else {
            // Gap found, close current range and start new one
            ranges.push((range_start, range_end));
            range_start = id;
            range_end = id;
        }
    }
    // Don't forget the last range
    ranges.push((range_start, range_end));

    ranges
}

/// Creates leases for each contiguous block of messages.
pub(crate) fn create_leases_for_msgs(
    msg_ids: &[MsgId],
    stream_id: StreamId,
    cg: ConsumerGroup,
    now: Timestamp,
    visibility_timeout: std::time::Duration,
    lease_diff: &mut LeaseDiff,
) {
    let ranges = group_into_contiguous_ranges(msg_ids);
    for (block_start, block_end) in ranges {
        lease_diff.to_insert.push(LeaseRow {
            stream_id,
            cg: cg.clone(),
            block_start,
            block_end,
            leased_at: now,
            expires_at: now + visibility_timeout,
            acked_at: None,
            dlq_at: None,
        });
    }
}

impl Fetch {
    pub fn new(
        state: &State,
        stream_id: ConfigGroupId,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        visibility_timeout: std::time::Duration,
    ) -> Result<Self> {
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;

        // Unlike FetchLocking, we don't block on active leases.
        // Instead, we exclude acked, DLQ'd, and active leases from the fetch.
        let blocked_leases = leases
            .iter()
            .filter(|lease| lease.acked_at.is_some() || lease.is_dlq() || lease.is_active(now));

        let msgs = MsgRow::fetch_available(state, stream_id, blocked_leases, batch_size.into())?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

        // Create separate leases for each contiguous block of messages.
        // This prevents covering gaps (e.g., DLQ'd messages) with a single range lease.
        let msg_ids: Vec<MsgId> = msgs.iter().map(|(id, _)| *id).collect();
        create_leases_for_msgs(
            &msg_ids,
            stream_id,
            cg,
            now,
            visibility_timeout,
            &mut lease_diff,
        );

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
