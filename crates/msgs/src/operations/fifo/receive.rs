use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU16,
};

use diom_core::{
    PersistableValue,
    task::spawn_blocking_in_current_span,
    types::{ByteString, DurationMs, UnixTimestampMs},
};
use diom_error::{Error, Result};
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, Offset, Partition, TopicIn, TopicName},
    operations::queue::compact_cursor,
    storage::{MsgRow, QueueLeaseKey, QueueLeaseRow, StreamLeaseKey, StreamLeaseRow, TopicRow},
};

use super::super::{FifoReceiveResponse, MsgsRaftState, MsgsRequest};

/// How many messages, at most, a single `receive` call is allowed to scan
/// on a single partition before it gives up and returns whatever's collected.
///
/// With FIFO semantics, a message could be scanned but not delivered. This would
/// happen if a key is blocked because earlier messages with the same key
/// are already leased to another Consumer. In this situation, we might have to keep
/// scanning a partition for msgs, and we want to clamp how long that scanning can take.
pub(crate) fn fifo_scan_budget(batch_size: u16) -> usize {
    // Right now these values are mostly vibes based. They might be revisited later.

    const FIFO_SCAN_MULTIPLE: usize = 16;
    const FIFO_MIN_SCAN: usize = 1024;

    (batch_size as usize)
        .saturating_mul(FIFO_SCAN_MULTIPLE)
        .max(FIFO_MIN_SCAN)
}

/// What a FIFO scan does with a candidate whose key is not already blocked.
pub(crate) enum FifoDisposition {
    /// Terminal (acked or DLQ'd). Neither delivered nor locks the key.
    Skip,
    /// In flight or not yet due. Locks the key so nothing later of it is handed out.
    Block,
    /// Available now. Delivered, and the key stays unblocked so the rest of its run follows.
    Deliver,
}

pub(crate) fn classify_fifo_msg(
    existing_lease: Option<&QueueLeaseRow>,
    scheduled_at: Option<UnixTimestampMs>,
    now: UnixTimestampMs,
) -> FifoDisposition {
    if existing_lease.is_some_and(|l| l.is_acked() || l.is_dlq()) {
        return FifoDisposition::Skip;
    }

    let leased = existing_lease.is_some_and(|l| !l.is_available(now));
    let scheduled = scheduled_at.is_some_and(|at| at > now);
    if leased || scheduled {
        return FifoDisposition::Block;
    }

    FifoDisposition::Deliver
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct FifoReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    batch_size: NonZeroU16,
    #[serde(rename = "lease_duration_ms")]
    lease_duration: DurationMs,
    topic_id_random_bytes: UuidV7RandomBytes,
    retention_period: Option<DurationMs>,
}

impl FifoReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration: DurationMs,
        retention_period: Option<DurationMs>,
    ) -> Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.topic, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };
        Ok(Self {
            namespace_id,
            topic,
            partition,
            consumer_group,
            batch_size,
            lease_duration,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
            retention_period,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size, msgs_returned = tracing::field::Empty))]
    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<FifoReceiveResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let mut remaining = self.batch_size.get();
            let mut all_msgs: Vec<FifoReceiveMsg> = Vec::with_capacity(remaining.into());

            let expiry = now + self.lease_duration;
            let expiry_cutoff = self
                .retention_period
                .map(|rp| now.saturating_sub(rp))
                .unwrap_or(UnixTimestampMs::UNIX_EPOCH);
            let scan_budget = fifo_scan_budget(self.batch_size.get());

            let mut batch = state.db.batch();

            let topic_row = TopicRow::fetch_or_create(
                &state.metadata_tables,
                &mut batch,
                self.namespace_id,
                &self.topic,
                now,
                self.topic_id_random_bytes,
            )?;

            let partitions = match self.partition {
                Some(p) => vec![p],
                None => topic_row.partitions_shuffled(now.as_millisecond())?,
            };

            for partition in partitions {
                let mut cursor = match StreamLeaseRow::fetch(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                )? {
                    Some(cursor) => cursor,
                    None => StreamLeaseRow::new()?,
                };

                let mut scan_offset = cursor.offset;
                let mut blocked_keys: HashSet<String> = HashSet::new();
                let mut scanned = 0usize;

                loop {
                    if remaining == 0 || scanned >= scan_budget {
                        break;
                    }

                    let msgs = MsgRow::fetch_range(
                        &state.msg_table,
                        topic_row.id,
                        partition,
                        scan_offset,
                        remaining,
                        expiry_cutoff,
                    )?;

                    if msgs.is_empty() {
                        break;
                    }

                    let last_offset = msgs.last().expect("non-empty").0;
                    scanned += msgs.len();

                    let n = fifo_lease_available_msgs(
                        &state,
                        &mut batch,
                        &mut all_msgs,
                        msgs,
                        partition,
                        topic_row.id,
                        &self.consumer_group,
                        now,
                        expiry,
                        &mut blocked_keys,
                    )?;
                    remaining = remaining.saturating_sub(n);

                    scan_offset = last_offset + 1;
                }

                compact_cursor(
                    &mut cursor,
                    &mut batch,
                    &state,
                    topic_row.id,
                    partition,
                    &self.consumer_group,
                )?;

                batch.insert_row(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(&topic_row.id, &partition, &self.consumer_group),
                    &cursor,
                )?;

                if remaining == 0 {
                    break;
                }
            }

            batch.commit().map_err(Error::from)?;

            Span::current().record("msgs_returned", all_msgs.len());
            state.metrics.record_fifo_received(
                &self.topic,
                &self.consumer_group,
                all_msgs.len() as u64,
            );
            Ok(FifoReceiveResponseData { msgs: all_msgs })
        })
        .await?
    }
}

/// Leases messages under strict per-key ordering: a key is delivered to at most one caller at a
/// time. The first un-acked message of each key is its head. If the head is in-flight (leased,
/// retry-scheduled, or not-yet-due) the key is locked and none of its later messages are handed
/// out. A key whose head is available is delivered along with its subsequent messages,
/// which is why a delivered key is deliberately left unblocked. Keyless messages are ungrouped.
#[allow(clippy::too_many_arguments)]
fn fifo_lease_available_msgs(
    state: &State,
    batch: &mut fjall::OwnedWriteBatch,
    all_msgs: &mut Vec<FifoReceiveMsg>,
    msgs: Vec<(Offset, MsgRow)>,
    partition: Partition,
    topic_id: TopicId,
    consumer_group: &ConsumerGroup,
    now: UnixTimestampMs,
    expiry: UnixTimestampMs,
    blocked_keys: &mut HashSet<String>,
) -> Result<u16> {
    let mut count = 0;

    for (offset, msg) in msgs {
        if let Some(key) = &msg.key
            && blocked_keys.contains(key)
        {
            continue;
        }

        let msg_id = MsgId::new(partition, offset);
        let existing_lease = QueueLeaseRow::fetch(
            &state.metadata_tables,
            QueueLeaseKey::build_key(&topic_id, &msg_id.partition, &msg_id.offset, consumer_group),
        )?;

        match classify_fifo_msg(existing_lease.as_ref(), msg.scheduled_at, now) {
            FifoDisposition::Skip => {}
            FifoDisposition::Block => {
                // A not-yet-due head that has no lease gets a synthetic one (expiry = scheduled_at) so
                // later scans skip it via the lease table without re-reading the message body.
                if msg.scheduled_at.is_some_and(|at| at > now) && existing_lease.is_none() {
                    let scheduled_at = msg.scheduled_at.expect("scheduled implies Some");
                    batch.insert_row(
                        &state.metadata_tables,
                        QueueLeaseKey::build_key(
                            &topic_id,
                            &msg_id.partition,
                            &msg_id.offset,
                            consumer_group,
                        ),
                        &QueueLeaseRow {
                            expiry: scheduled_at,
                            dlq: false,
                            attempt_count: 0,
                        },
                    )?;
                }
                if let Some(key) = msg.key {
                    blocked_keys.insert(key);
                }
            }
            FifoDisposition::Deliver => {
                let attempt_count = existing_lease.map(|l| l.attempt_count).unwrap_or(0);
                batch.insert_row(
                    &state.metadata_tables,
                    QueueLeaseKey::build_key(
                        &topic_id,
                        &msg_id.partition,
                        &msg_id.offset,
                        consumer_group,
                    ),
                    &QueueLeaseRow {
                        expiry,
                        dlq: false,
                        attempt_count,
                    },
                )?;

                // The key is left unblocked so the rest of the messages with the same key
                // are delivered to this same caller.
                all_msgs.push(FifoReceiveMsg {
                    msg_id,
                    key: msg.key,
                    value: msg.value,
                    headers: msg.headers,
                    timestamp: msg.timestamp,
                    scheduled_at: msg.scheduled_at,
                });
                count += 1;
            }
        }
    }

    Ok(count)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FifoReceiveMsg {
    pub msg_id: MsgId,
    pub key: Option<String>,
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FifoReceiveResponseData {
    pub msgs: Vec<FifoReceiveMsg>,
}

impl MsgsRequest for FifoReceiveOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> FifoReceiveResponse {
        FifoReceiveResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
