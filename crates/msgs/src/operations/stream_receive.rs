use std::num::NonZeroU16;

use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{
        ConsumerGroup, DEFAULT_PARTITION_COUNT, Offset, PartitionIndex, partition_topic_name,
    },
    tables::{LeaseDiff, LeaseRow, MsgRow, OffsetRow},
};

use super::{MsgsRaftState, MsgsRequest, StreamReceiveResponse};

fn group_into_contiguous_ranges(offsets: &[Offset]) -> Vec<(Offset, Offset)> {
    if offsets.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut range_start = offsets[0];
    let mut range_end = offsets[0];

    for &offset in &offsets[1..] {
        if offset == range_end + 1 {
            range_end = offset;
        } else {
            ranges.push((range_start, range_end));
            range_start = offset;
            range_end = offset;
        }
    }
    ranges.push((range_start, range_end));

    ranges
}

fn create_leases_for_msgs(
    offsets: &[Offset],
    namespace_id: NamespaceId,
    partition: PartitionIndex,
    cg: &ConsumerGroup,
    now: Timestamp,
    lease_duration: std::time::Duration,
    lease_diff: &mut LeaseDiff,
) {
    let ranges = group_into_contiguous_ranges(offsets);
    for (block_start, block_end) in ranges {
        lease_diff.to_insert.push(LeaseRow {
            namespace_id,
            partition,
            cg: cg.clone(),
            block_start,
            block_end,
            leased_at: now,
            expires_at: now + lease_duration,
            acked_at: None,
            dlq_at: None,
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveOperation {
    namespace_id: NamespaceId,
    topic: String,
    cg: ConsumerGroup,
    batch_size: NonZeroU16,
    lease_duration_millis: u64,
}

impl StreamReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: String,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration_millis: u64,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            cg,
            batch_size,
            lease_duration_millis,
        }
    }

    fn apply_real(self, state: &State) -> coyote_operations::Result<StreamReceiveResponseData> {
        let now = Timestamp::now();
        let lease_duration = std::time::Duration::from_millis(self.lease_duration_millis);
        let mut all_msgs = Vec::new();
        let mut all_lease_diffs = Vec::new();
        let mut remaining = usize::from(self.batch_size.get());
        let mut any_partition_locked = false;

        for p in 0..DEFAULT_PARTITION_COUNT {
            if remaining == 0 {
                break;
            }
            let partition = PartitionIndex::new(p)
                .expect("already validated the partition number is in the valid range.");

            let start_offset = OffsetRow::fetch(state, self.namespace_id, partition, &self.cg)?
                .unwrap_or(Offset::MIN);

            let leases = LeaseRow::fetch_all(state, self.namespace_id, partition, &self.cg)?;

            // If any active lease exists on this partition, skip it entirely
            let has_active_lease = leases.iter().any(|l| l.is_active(now));
            if has_active_lease {
                any_partition_locked = true;
                all_lease_diffs.push(LeaseRow::cull_and_compact(leases, now));
                continue;
            }

            let blocked_leases = leases.iter().filter(|l| l.acked_at.is_some() || l.is_dlq());

            let batch_size = remaining;
            let msgs = MsgRow::fetch_available(
                state,
                self.namespace_id,
                partition,
                start_offset,
                blocked_leases,
                batch_size,
            )?;

            let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

            let offsets: Vec<Offset> = msgs.iter().map(|(offset, _)| *offset).collect();
            create_leases_for_msgs(
                &offsets,
                self.namespace_id,
                partition,
                &self.cg,
                now,
                lease_duration,
                &mut lease_diff,
            );

            remaining -= msgs.len();

            for (offset, msg) in msgs {
                all_msgs.push(StreamReceiveMsg {
                    offset,
                    topic: partition_topic_name(&self.topic, partition),
                    value: msg.value,
                    headers: msg.headers,
                    timestamp: msg.created_at,
                });
            }

            all_lease_diffs.push(lease_diff);
        }

        let mut batch = state.db.batch();
        for diff in all_lease_diffs {
            diff.apply_diff(state, &mut batch)?;
        }
        batch.commit().map_err(coyote_error::Error::from)?;

        // If we got no messages and at least one partition was locked, return an error
        // so the caller knows to retry after committing or waiting for lease expiry.
        if all_msgs.is_empty() && any_partition_locked {
            return Err(coyote_error::Error::http(coyote_error::HttpError::bad_request(
                Some("partition_locked".to_owned()),
                Some("All partitions with pending messages are locked by active leases. Commit offsets or wait for lease expiry.".to_owned()),
            )).into());
        }

        Ok(StreamReceiveResponseData { msgs: all_msgs })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveMsg {
    pub offset: Offset,
    pub topic: String,
    pub value: Vec<u8>,
    pub headers: std::collections::HashMap<String, String>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveResponseData {
    pub msgs: Vec<StreamReceiveMsg>,
}

impl MsgsRequest for StreamReceiveOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> StreamReceiveResponse {
        StreamReceiveResponse(self.apply_real(state.msgs))
    }
}
