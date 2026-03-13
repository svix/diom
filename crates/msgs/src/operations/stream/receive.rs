use std::{num::NonZeroU16, time::Duration};

use coyote_error::Error;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use crate::{
    State,
    entities::{ConsumerGroup, Offset, Partition, TopicIn, TopicName, TopicPartition},
    tables::{MsgRow, StreamLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, StreamReceiveResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    consumer_group: ConsumerGroup,
    batch_size: NonZeroU16,
    lease_duration_millis: u64,
}

impl StreamReceiveOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        consumer_group: ConsumerGroup,
        batch_size: NonZeroU16,
        lease_duration_millis: u64,
    ) -> coyote_error::Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => (tp.raw, Some(tp.partition)),
            TopicIn::TopicName(tn) => (tn, None),
        };
        Ok(Self {
            namespace_id,
            topic,
            partition,
            consumer_group,
            batch_size,
            lease_duration_millis,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size = self.batch_size))]
    fn apply_real(
        self,
        state: &State,
        now: Timestamp,
    ) -> coyote_operations::Result<StreamReceiveResponseData> {
        let lease_duration = Duration::from_millis(self.lease_duration_millis);
        let mut remaining = self.batch_size.get();
        let mut all_msgs: Vec<StreamReceiveMsg> = Vec::with_capacity(remaining as usize);
        let expiry = now + lease_duration;

        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch_or_create(
            &state.metadata_tables,
            &mut batch,
            self.namespace_id,
            &self.topic,
            now,
        )?;

        Span::current().record("partition_count", topic_row.partitions);

        let partitions = self
            .partition
            .map(|p| vec![p.get()])
            .unwrap_or_else(|| topic_row.partitions_shuffled());

        let mut no_lease_available = true;

        for partition in partitions {
            let topic = TopicPartition::new(self.topic.clone(), Partition::new(partition)?);
            let (mut lease, is_new) = match StreamLeaseRow::fetch(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, topic.partition, &self.consumer_group),
            )? {
                Some(lease) => (lease, false),
                None => {
                    let mut lease = StreamLeaseRow::new()?;
                    lease.offset =
                        MsgRow::next_offset(&state.msg_table, topic_row.id, topic.partition)?;
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
            )?;

            if msgs.is_empty() {
                if is_new {
                    batch.insert_row(
                        &state.metadata_tables,
                        StreamLeaseRow::key_for(
                            topic_row.id,
                            topic.partition,
                            &self.consumer_group,
                        ),
                        &lease,
                    )?;
                }
                continue;
            }

            lease.expiry = expiry;

            // FIXME(@svix-gabriel) - I should just be able to reference msgs.last.offset.
            // this'll require a larger change though.
            lease.end_offset = lease.offset + msgs.len() as u64 - 1;
            remaining -= msgs.len() as u16;

            all_msgs.extend(
                msgs.into_iter()
                    .enumerate()
                    .map(|(i, msg)| StreamReceiveMsg {
                        value: msg.value,
                        timestamp: msg.timestamp,
                        headers: msg.headers,
                        offset: lease.offset + i as u64,
                        topic: topic.clone(),
                    }),
            );

            batch.insert_row(
                &state.metadata_tables,
                StreamLeaseRow::key_for(topic_row.id, topic.partition, &self.consumer_group),
                &lease,
            )?;

            if remaining == 0 {
                break;
            }
        }

        if no_lease_available {
            return Err(Error::invalid_user_input("no available leases").into());
        }

        batch.commit().map_err(Error::from)?;

        Span::current().record("msgs_returned", all_msgs.len());

        Ok(StreamReceiveResponseData { msgs: all_msgs })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveMsg {
    pub offset: Offset,
    pub topic: TopicPartition,
    pub value: Vec<u8>,
    pub headers: std::collections::HashMap<String, String>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamReceiveResponseData {
    pub msgs: Vec<StreamReceiveMsg>,
}

impl MsgsRequest for StreamReceiveOperation {
    fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &coyote_operations::OpContext,
    ) -> StreamReceiveResponse {
        StreamReceiveResponse(self.apply_real(state.msgs, ctx.timestamp))
    }
}
