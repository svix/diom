use super::{RedriveResponse, StreamRaftState, StreamRequest};
use crate::{State, entities::ConsumerGroup, tables::LeaseRow};
use diom_configgroup::entities::ConfigGroupId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedriveOperation {
    group_id: ConfigGroupId,
    cg: ConsumerGroup,
}

impl RedriveOperation {
    pub fn new(group_id: ConfigGroupId, cg: ConsumerGroup) -> Self {
        Self { group_id, cg }
    }

    fn apply_real(self, state: &State) -> diom_operations::Result<RedriveResponseData> {
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, self.group_id, &self.cg)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases.clone(), now);

        // Delete all DLQ'd leases - this makes those messages available again
        for lease in leases {
            if lease.is_dlq() {
                lease_diff.to_delete.push(lease);
            }
        }

        let mut batch = state.db.batch();
        lease_diff.apply_diff(state, &mut batch)?;
        batch.commit().map_err(diom_error::Error::from)?;

        Ok(RedriveResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedriveResponseData {}

impl StreamRequest for RedriveOperation {
    fn apply(self, state: StreamRaftState<'_>) -> RedriveResponse {
        RedriveResponse(self.apply_real(state.stream))
    }
}
