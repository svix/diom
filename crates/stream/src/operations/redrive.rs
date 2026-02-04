use crate::{
    State,
    entities::{ConsumerGroup, StreamName},
    tables::{LeaseDiff, LeaseRow, NameToStreamRow},
};
use diom_error::Result;
use jiff::Timestamp;

pub struct Redrive {
    lease_diff: LeaseDiff,
}

pub struct RedriveOutput {}

impl Redrive {
    pub fn new(state: &State, name: StreamName, cg: ConsumerGroup) -> Result<Self> {
        let stream_id = NameToStreamRow::get_stream_id(state, &name)?;
        let now = Timestamp::now();
        let leases = LeaseRow::fetch_all(state, stream_id, &cg)?;

        let mut lease_diff = LeaseRow::cull_and_compact(leases.clone(), now);

        // Delete all DLQ'd leases - this makes those messages available again
        for lease in leases {
            if lease.is_dlq() {
                lease_diff.to_delete.push(lease);
            }
        }

        Ok(Self { lease_diff })
    }

    pub fn apply_operation(self, state: &State) -> Result<RedriveOutput> {
        let mut batch = state.db.batch();
        self.lease_diff.apply_diff(state, &mut batch)?;
        batch.commit()?;
        Ok(RedriveOutput {})
    }
}
