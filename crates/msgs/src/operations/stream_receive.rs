use std::num::NonZeroU16;

use coyote_namespace::entities::NamespaceId;
use fjall_utils::ReadableDatabase;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, Offset, Partition, RawTopic, Topic, TopicIn},
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
    partition: Partition,
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
    topic: RawTopic,
    partitions: Vec<Partition>,
    cg: ConsumerGroup,
    batch_size: NonZeroU16,
    lease_duration_millis: u64,
    now: Timestamp,
}

impl StreamReceiveOperation {
    #[tracing::instrument(skip_all, level = "debug", fields(topic = %topic.raw_topic(), partition_count, partitions_eligible))]
    pub fn new(
        db: &impl ReadableDatabase,
        namespace_id: NamespaceId,
        topic: TopicIn,
        cg: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration_millis: u64,
    ) -> coyote_error::Result<Self> {
        let now = Timestamp::now();
        let raw_topic = topic.raw_topic().clone();
        let partition_count = crate::topic_partition_count(db, namespace_id, &raw_topic)?;

        let all_partitions: Vec<Partition> = match topic {
            TopicIn::WithPartition(t) => vec![t.partition],
            TopicIn::Raw(_) => (0..partition_count)
                .map(|p| Partition::new(p).expect("partition index is within MAX_PARTITION_COUNT"))
                .collect(),
        };

        let mut partitions = Vec::with_capacity(all_partitions.len());
        for p in all_partitions {
            if !crate::partition_has_active_lease(db, namespace_id, p, &cg, now)? {
                partitions.push(p);
            }
        }

        let span = Span::current();
        span.record("partition_count", partition_count);
        span.record("partitions_eligible", partitions.len());

        Ok(Self {
            namespace_id,
            topic: raw_topic,
            partitions,
            cg,
            batch_size,
            lease_duration_millis,
            now,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(topic = %self.topic, partitions = self.partitions.len(), batch_size = self.batch_size.get(), msgs_returned))]
    fn apply_real(self, state: &State) -> coyote_operations::Result<StreamReceiveResponseData> {
        let now = self.now;
        let lease_duration = std::time::Duration::from_millis(self.lease_duration_millis);
        let mut all_msgs = Vec::new();
        let mut all_lease_diffs = Vec::new();
        let mut remaining = usize::from(self.batch_size.get());

        for partition in self.partitions {
            if remaining == 0 {
                break;
            }

            let (msgs, lease_diff) = receive_partition(
                state,
                self.namespace_id,
                &self.topic,
                partition,
                &self.cg,
                now,
                lease_duration,
                remaining,
            )?;

            remaining -= msgs.len();
            all_msgs.extend(msgs);
            all_lease_diffs.push(lease_diff);
        }

        let mut batch = state.db.batch();
        for diff in all_lease_diffs {
            diff.apply_diff(state, &mut batch)?;
        }
        batch.commit().map_err(coyote_error::Error::from)?;

        Span::current().record("msgs_returned", all_msgs.len());
        Ok(StreamReceiveResponseData { msgs: all_msgs })
    }
}

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(skip_all, level = "debug", fields(partition = partition.get(), has_active_lease, msgs_fetched))]
fn receive_partition(
    state: &State,
    namespace_id: NamespaceId,
    topic: &RawTopic,
    partition: Partition,
    cg: &ConsumerGroup,
    now: Timestamp,
    lease_duration: std::time::Duration,
    remaining: usize,
) -> coyote_operations::Result<(Vec<StreamReceiveMsg>, LeaseDiff)> {
    let start_offset = OffsetRow::fetch(state, namespace_id, partition, cg)?.unwrap_or(Offset::MIN);

    let leases = LeaseRow::fetch_all(state, namespace_id, partition, cg)?;

    // Check for active leases inside apply_real (Raft state machine) to
    // prevent races — the pre-Raft check in the constructor is only an
    // optimistic fast-path.
    let has_active_lease = leases.iter().any(|l| l.is_active(now));
    Span::current().record("has_active_lease", has_active_lease);
    if has_active_lease {
        return Ok((Vec::new(), LeaseRow::cull_and_compact(leases, now)));
    }

    let blocked_leases = leases.iter().filter(|l| l.acked_at.is_some() || l.is_dlq());

    let msgs = MsgRow::fetch_available(
        state,
        namespace_id,
        partition,
        start_offset,
        blocked_leases,
        remaining,
    )?;

    let mut lease_diff = LeaseRow::cull_and_compact(leases, now);

    let offsets: Vec<Offset> = msgs.iter().map(|(offset, _)| *offset).collect();
    create_leases_for_msgs(
        &offsets,
        namespace_id,
        partition,
        cg,
        now,
        lease_duration,
        &mut lease_diff,
    );

    let receive_msgs: Vec<StreamReceiveMsg> = msgs
        .into_iter()
        .map(|(offset, msg)| StreamReceiveMsg {
            offset,
            topic: Topic::new(topic.clone(), partition),
            value: msg.value,
            headers: msg.headers,
            timestamp: msg.created_at,
        })
        .collect();

    Span::current().record("msgs_fetched", receive_msgs.len());
    Ok((receive_msgs, lease_diff))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveMsg {
    pub offset: Offset,
    pub topic: Topic,
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
