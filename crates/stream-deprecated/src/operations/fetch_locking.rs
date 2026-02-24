use std::num::NonZeroU16;

use super::{FetchLockingResponse, StreamRaftState, StreamRequest, fetch::create_leases_for_msgs};
use diom_configgroup::entities::ConfigGroupId;
use diom_error::HttpError;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, MsgOut},
    tables::{LeaseRow, MsgRow},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchLockingOperation {
    group_id: ConfigGroupId,
    cg: ConsumerGroup,
    batch_size: NonZeroU16,
    visibility_timeout_secs: u64,
}

impl FetchLockingOperation {
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

    fn apply_real(self, state: &State) -> diom_operations::Result<FetchLockingResponseData> {
        let now = Timestamp::now();
        let visibility_timeout = std::time::Duration::from_secs(self.visibility_timeout_secs);
        let leases = LeaseRow::fetch_all(state, self.group_id, &self.cg)?;

        let has_active_lease = leases.iter().any(|lease| lease.is_active(now));

        if has_active_lease {
            return Err(diom_error::Error::from(HttpError::bad_request(
                Some("consumer_group_locked".to_owned()),
                Some("Concurrent reads from the same consumer group".to_string()),
            ))
            .into());
        }

        let blocked_leases = leases
            .iter()
            .filter(|lease| lease.acked_at.is_some() || lease.is_dlq());
        let msgs =
            MsgRow::fetch_available(state, self.group_id, blocked_leases, self.batch_size.into())?;

        if msgs.is_empty() {
            // FIXME(@svix-gabriel) this isn't really an error, but we need to go back
            // and change any HttpErrors anyway, so this is simpler for now.
            return Err(diom_error::Error::from(HttpError::bad_request(
                Some("empty_stream".to_owned()),
                Some("no messages available".to_string()),
            ))
            .into());
        }

        let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

        // Create separate leases for each contiguous block of messages.
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
        batch.commit().map_err(diom_error::Error::from)?;

        let msgs = msgs
            .into_iter()
            .map(|(id, msg)| MsgOut {
                id,
                payload: msg.payload,
                headers: msg.headers,
                timestamp: msg.created_at,
            })
            .collect();

        Ok(FetchLockingResponseData { msgs })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchLockingResponseData {
    pub msgs: Vec<MsgOut>,
}

impl StreamRequest for FetchLockingOperation {
    fn apply(self, state: StreamRaftState<'_>) -> FetchLockingResponse {
        FetchLockingResponse(self.apply_real(state.stream))
    }
}
