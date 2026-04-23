use std::num::NonZeroU16;

use diom_core::{
    PersistableValue,
    task::spawn_blocking_in_current_span,
    types::{ByteString, DurationMs, UnixTimestampMs},
};
use diom_error::{Error, Result};
use diom_id::{NamespaceId, UuidV7RandomBytes};
use fjall_utils::{TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{
        ConsumerGroup, Offset, Partition, SeekPosition, TopicIn, TopicName, TopicPartition,
    },
    storage::{MsgRow, StreamLeaseKey, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamReceiveResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct StreamReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    batch_size: NonZeroU16,
    #[serde(rename = "lease_duration_ms")]
    lease_duration: DurationMs,
    default_starting_position: SeekPosition,
    topic_id_random_bytes: UuidV7RandomBytes,
    retention_period: Option<DurationMs>,
}

impl StreamReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration: DurationMs,
        default_starting_position: SeekPosition,
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
            default_starting_position,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
            retention_period,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size))]
    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<StreamReceiveResponseData> {
        let state = state.clone();

        spawn_blocking_in_current_span(move || {
            let mut remaining = self.batch_size.get();
            let mut all_msgs: Vec<StreamReceiveMsg> = Vec::with_capacity(remaining as usize);
            let expiry = now + self.lease_duration;
            let expiry_cutoff = self
                .retention_period
                .map(|rp| now.saturating_sub(rp))
                .unwrap_or(UnixTimestampMs::UNIX_EPOCH);

            let mut batch = state.db.batch();

            let topic_row = TopicRow::fetch_or_create(
                &state.metadata_tables,
                &mut batch,
                self.namespace_id,
                &self.topic,
                now,
                self.topic_id_random_bytes,
            )?;

            Span::current().record("partition_count", topic_row.partitions);

            let partitions = match self.partition {
                Some(p) => vec![p],
                None => topic_row.partitions_shuffled(now.as_millisecond())?,
            };

            let mut no_lease_available = true;

            for partition in partitions {
                let topic = TopicPartition::new(self.topic.clone(), partition);
                let (mut lease, is_new) = match StreamLeaseRow::fetch(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(
                        &topic_row.id,
                        &topic.partition,
                        &self.consumer_group,
                    ),
                )? {
                    Some(lease) => (lease, false),
                    None => {
                        let mut lease = StreamLeaseRow::new()?;
                        lease.offset = match self.default_starting_position {
                            SeekPosition::Earliest => 0,
                            SeekPosition::Latest => MsgRow::next_offset(
                                &state.msg_table,
                                topic_row.id,
                                topic.partition,
                            )?,
                        };
                        (lease, true)
                    }
                };

                if lease.expiry > now {
                    continue;
                }
                no_lease_available = false;

                let msgs = MsgRow::fetch_range(
                    &state.msg_table,
                    topic_row.id,
                    topic.partition,
                    lease.offset,
                    remaining,
                    expiry_cutoff,
                )?;

                if msgs.is_empty() {
                    if is_new {
                        batch.insert_row(
                            &state.metadata_tables,
                            StreamLeaseKey::build_key(
                                &topic_row.id,
                                &topic.partition,
                                &self.consumer_group,
                            ),
                            &lease,
                        )?;
                    }
                    continue;
                }

                lease.expiry = expiry;

                lease.end_offset = msgs.last().expect("non-empty").0;
                remaining -= msgs.len() as u16;

                all_msgs.extend(msgs.into_iter().map(|(offset, msg)| StreamReceiveMsg {
                    value: msg.value,
                    timestamp: msg.timestamp,
                    headers: msg.headers,
                    offset,
                    topic: topic.clone(),
                    scheduled_at: msg.scheduled_at,
                }));

                batch.insert_row(
                    &state.metadata_tables,
                    StreamLeaseKey::build_key(
                        &topic_row.id,
                        &topic.partition,
                        &self.consumer_group,
                    ),
                    &lease,
                )?;

                if remaining == 0 {
                    break;
                }
            }

            if no_lease_available {
                state
                    .metrics
                    .record_stream_no_lease(&self.topic, &self.consumer_group);
                return Err(Error::bad_request(
                    "no_available_leases",
                    "no available leases",
                ));
            }

            batch.commit().map_err(Error::from)?;

            Span::current().record("msgs_returned", all_msgs.len());
            state.metrics.record_stream_received(
                &self.topic,
                &self.consumer_group,
                all_msgs.len() as u64,
            );
            Ok(StreamReceiveResponseData { msgs: all_msgs })
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveMsg {
    pub offset: Offset,
    pub topic: TopicPartition,
    pub value: ByteString,
    pub headers: std::collections::HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveResponseData {
    pub msgs: Vec<StreamReceiveMsg>,
}

impl MsgsRequest for StreamReceiveOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> StreamReceiveResponse {
        StreamReceiveResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
