use std::collections::BTreeMap;

use crate::{
    State,
    entities::{MsgIn, Offset, Partition, TopicIn, TopicName, TopicPartition, partition_for_key},
    tables::{MsgRow, TopicRow},
};
use diom_core::task::spawn_blocking_in_current_span;
use diom_error::Error;
use diom_id::{NamespaceId, TopicId};
use fjall::OwnedWriteBatch;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use super::{MsgsRaftState, MsgsRequest, PublishResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partition: Option<Partition>,
    msgs: Vec<MsgIn>,
}

impl PublishOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        msgs: Vec<MsgIn>,
    ) -> diom_error::Result<Self> {
        let (topic, partition) = match topic {
            TopicIn::TopicPartition(tp) => {
                if msgs.iter().any(|m| m.key.is_some()) {
                    return Err(Error::invalid_user_input(
                        "msg key cannot be specified alongside a specific partition",
                    ));
                }
                (tp.raw, Some(tp.partition))
            }
            TopicIn::TopicName(tn) => (tn, None),
        };

        Ok(Self {
            namespace_id,
            topic,
            partition,
            msgs,
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(msg_count = self.msgs.len()))]
    async fn apply_real(
        self,
        state: &State,
        now: Timestamp,
    ) -> diom_operations::Result<PublishResponseData> {
        let state = state.clone();

        let results = spawn_blocking_in_current_span(move || {
            let topic_row = TopicRow::fetch(
                &state.metadata_tables,
                TopicRow::key_for(self.namespace_id, &self.topic),
            )?;
            let mut batch = state.db.batch();

            let topic_row = match (topic_row, self.partition) {
                (Some(row), Some(partition)) if (0..row.partitions).contains(&partition.get()) => {
                    row
                }
                (Some(_), Some(_)) => {
                    return Err(Error::invalid_user_input("partition out of range"));
                }
                (Some(row), None) => row,
                (None, Some(_)) => {
                    return Err(Error::invalid_user_input("topic does not exist"));
                }
                (None, None) => {
                    let row = TopicRow::new(self.topic.clone(), now);
                    batch.insert_row(
                        &state.metadata_tables,
                        TopicRow::key_for(self.namespace_id, &self.topic),
                        &row,
                    )?;
                    row
                }
            };

            Span::current().record("partition_count", topic_row.partitions);

            let msgs_by_partition =
                group_msgs_by_partition(self.msgs, self.partition, topic_row.partitions);

            let results = write_msg_batch(
                &mut batch,
                &state.msg_table,
                &self.topic,
                topic_row.id,
                msgs_by_partition,
                now,
            )?;

            batch.commit().map_err(Error::from)?;
            Ok::<_, Error>(results)
        })
        .await??;

        Ok(PublishResponseData { topics: results })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedTopic {
    pub topic: TopicPartition,
    pub start_offset: Offset,
    pub offset: Offset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponseData {
    // FIXME: should be a HashSet, not a Vec...
    /// The list of topics published and their offsets.
    pub topics: Vec<PublishedTopic>,
}

impl MsgsRequest for PublishOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> PublishResponse {
        PublishResponse(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}

fn group_msgs_by_partition(
    msgs: Vec<MsgIn>,
    partition: Option<Partition>,
    num_partitions: u16,
) -> BTreeMap<Partition, Vec<MsgIn>> {
    let mut msgs_by_partition: BTreeMap<Partition, Vec<MsgIn>> = BTreeMap::new();
    for msg in msgs {
        let p = match partition {
            Some(p) => p,
            None => partition_for_key(msg.key.as_deref(), num_partitions),
        };
        msgs_by_partition.entry(p).or_default().push(msg);
    }
    msgs_by_partition
}

fn write_msg_batch(
    batch: &mut OwnedWriteBatch,
    msg_table: &fjall::Keyspace,
    topic_name: &TopicName,
    topic_id: TopicId,
    msgs_by_partition: BTreeMap<Partition, Vec<MsgIn>>,
    now: Timestamp,
) -> diom_operations::Result<Vec<PublishedTopic>> {
    let mut results: Vec<PublishedTopic> = Vec::with_capacity(msgs_by_partition.len());

    for (partition, msgs) in msgs_by_partition {
        let topic = TopicPartition::new(topic_name.clone(), partition);
        let mut offset = MsgRow::next_offset(msg_table, topic_id, partition)?;
        let start_offset = offset;

        for msg in msgs {
            let msg = MsgRow {
                value: msg.value,
                headers: msg.headers,
                timestamp: now,
            };
            batch.insert_row(
                msg_table,
                MsgRow::key_for(topic_id, partition, offset),
                &msg,
            )?;
            offset += 1;
        }

        results.push(PublishedTopic {
            start_offset,
            topic,
            offset,
        });
    }

    Ok(results)
}
