use std::num::NonZeroU16;

use super::{FetchResponse, StreamRaftState, StreamRequest};
use coyote_configgroup::entities::ConfigGroupId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, MsgOut},
    tables::{LeaseDiff, LeaseRow, MsgRow},
};

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
            range_end = id;
        } else {
            ranges.push((range_start, range_end));
            range_start = id;
            range_end = id;
        }
    }
    ranges.push((range_start, range_end));

    ranges
}

/// Creates leases for each contiguous block of messages.
pub(crate) fn create_leases_for_msgs(
    msg_ids: &[MsgId],
    group_id: ConfigGroupId,
    cg: ConsumerGroup,
    now: Timestamp,
    visibility_timeout: std::time::Duration,
    lease_diff: &mut LeaseDiff,
) {
    let ranges = group_into_contiguous_ranges(msg_ids);
    for (block_start, block_end) in ranges {
        lease_diff.to_insert.push(LeaseRow {
            group_id,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOperation {
    pub(crate) group_id: ConfigGroupId,
    pub(crate) cg: ConsumerGroup,
    pub(crate) batch_size: NonZeroU16,
    pub(crate) visibility_timeout_secs: u64,
}

impl FetchOperation {
    pub fn new(
        group_id: ConfigGroupId,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        visibility_timeout_secs: u64,
    ) -> Self {
        Self {
            group_id,
            cg,
            batch_size,
            visibility_timeout_secs,
        }
    }

    fn apply_real(self, state: &State) -> coyote_operations::Result<FetchResponseData> {
        let now = Timestamp::now();
        let visibility_timeout = std::time::Duration::from_secs(self.visibility_timeout_secs);
        let leases = LeaseRow::fetch_all(state, self.group_id, &self.cg)?;

        // Unlike FetchLocking, we don't block on active leases.
        // Instead, we exclude acked, DLQ'd, and active leases from the fetch.
        let blocked_leases = leases
            .iter()
            .filter(|lease| lease.acked_at.is_some() || lease.is_dlq() || lease.is_active(now));

        let msgs =
            MsgRow::fetch_available(state, self.group_id, blocked_leases, self.batch_size.into())?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

        // Create separate leases for each contiguous block of messages.
        // This prevents covering gaps (e.g., DLQ'd messages) with a single range lease.
        let msg_ids: Vec<MsgId> = msgs.iter().map(|(id, _)| *id).collect();
        create_leases_for_msgs(
            &msg_ids,
            self.group_id,
            self.cg,
            now,
            visibility_timeout,
            &mut lease_diff,
        );

        let mut batch = state.db.batch();
        lease_diff.apply_diff(state, &mut batch)?;
        batch.commit().map_err(coyote_error::Error::from)?;

        let msgs = msgs
            .into_iter()
            .map(|(id, msg)| MsgOut {
                id,
                payload: msg.payload,
                headers: msg.headers,
                timestamp: msg.created_at,
            })
            .collect();

        Ok(FetchResponseData { msgs })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchResponseData {
    pub msgs: Vec<MsgOut>,
}

impl StreamRequest for FetchOperation {
    fn apply(self, state: StreamRaftState<'_>) -> FetchResponse {
        FetchResponse(self.apply_real(state.stream))
    }
}
