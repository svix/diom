use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Offset, PartitionIndex},
    tables::{LeaseDiff, LeaseRow, OffsetRow},
};

use super::{MsgsRaftState, MsgsRequest, StreamCommitResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitOperation {
    namespace_id: NamespaceId,
    partition: PartitionIndex,
    cg: ConsumerGroup,
    offset: Offset,
    now: Timestamp,
}

impl StreamCommitOperation {
    pub fn new(
        namespace_id: NamespaceId,
        partition: PartitionIndex,
        cg: ConsumerGroup,
        offset: Offset,
    ) -> Self {
        Self {
            namespace_id,
            partition,
            cg,
            offset,
            now: Timestamp::now(),
        }
    }

    fn apply_real(self, state: &State) -> coyote_operations::Result<StreamCommitResponseData> {
        let now = self.now;
        let mut batch = state.db.batch();

        // Store next-to-read offset (committed offset + 1)
        OffsetRow::store(
            &mut batch,
            state,
            self.namespace_id,
            self.partition,
            &self.cg,
            self.offset.saturating_add(1),
        )?;

        // Shrink active leases at or below the committed offset, then cull expired ones
        let leases = LeaseRow::fetch_all(state, self.namespace_id, self.partition, &self.cg)?;
        let mut lease_diff = LeaseDiff::default();
        LeaseRow::shrink_active_leases_for_range(
            &leases,
            Offset::MIN,
            self.offset,
            now,
            &mut lease_diff,
        );
        lease_diff.extend(LeaseRow::cull_and_compact(leases, now));
        lease_diff.apply_diff(state, &mut batch)?;

        batch.commit().map_err(coyote_error::Error::from)?;

        Ok(StreamCommitResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamCommitResponseData {}

impl MsgsRequest for StreamCommitOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> StreamCommitResponse {
        StreamCommitResponse(self.apply_real(state.msgs))
    }
}
