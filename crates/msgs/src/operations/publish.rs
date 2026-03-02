use std::collections::BTreeMap;

use diom_namespace::entities::NamespaceId;
use fjall::OwnedWriteBatch;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tracing::Span;

use fjall_utils::ReadableDatabase;

use crate::{
    State,
    entities::{
        MsgIn, Offset, Partition, RawTopic, Topic, TopicIn, partition_for_key, random_partition,
    },
    tables::{MsgRow, msg_row_key},
};

use super::{MsgsRaftState, MsgsRequest, PublishResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOperation {
    namespace_id: NamespaceId,
    topic: RawTopic,
    partition_count: u16,
    /// The partition used for messages without a key.
    fallback_partition: Partition,
    msgs: Vec<MsgIn>,
    created_at: Timestamp,
}

impl PublishOperation {
    #[tracing::instrument(skip_all, level = "debug", fields(topic = %topic.raw_topic(), partition_count))]
    pub fn new(
        db: &impl ReadableDatabase,
        namespace_id: NamespaceId,
        topic: TopicIn,
        msgs: Vec<MsgIn>,
    ) -> diom_error::Result<Self> {
        let partition_count = crate::topic_partition_count(db, namespace_id, topic.raw_topic())?;
        Span::current().record("partition_count", partition_count);

        let (topic, fallback_partition) = match topic {
            TopicIn::WithPartition(t) => (t.raw, t.partition),
            TopicIn::Raw(raw) => (raw, random_partition(partition_count)),
        };

        if fallback_partition.get() >= partition_count {
            return Err(diom_error::Error::http(
                diom_error::HttpError::bad_request(
                    Some("partition_out_of_range".to_owned()),
                    Some(format!(
                        "Partition {} is out of range. Topic has {} partition(s). \
                         Use topic/configure to increase the partition count.",
                        fallback_partition.get(),
                        partition_count,
                    )),
                ),
            ));
        }

        Ok(Self {
            namespace_id,
            topic,
            partition_count,
            fallback_partition,
            msgs,
            created_at: Timestamp::now(),
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(topic = %self.topic, partition_count = self.partition_count, msg_count = self.msgs.len()))]
    fn apply_real(self, state: &State) -> diom_operations::Result<PublishResponseData> {
        let mut by_partition: BTreeMap<Partition, Vec<(usize, MsgIn)>> = BTreeMap::new();
        for (idx, msg) in self.msgs.into_iter().enumerate() {
            let partition = if let Some(key) = &msg.key {
                partition_for_key(key.as_bytes(), self.partition_count)
            } else {
                self.fallback_partition
            };
            by_partition.entry(partition).or_default().push((idx, msg));
        }

        let mut batch = state.db.batch();
        let created_at = self.created_at;
        let total_msgs = by_partition.values().map(|v| v.len()).sum();
        let mut results: Vec<(usize, PublishedMsg)> = Vec::with_capacity(total_msgs);

        for (partition, msgs) in by_partition {
            let partition_results = write_partition(
                state,
                self.namespace_id,
                &self.topic,
                partition,
                msgs,
                created_at,
                &mut batch,
            )?;
            results.extend(partition_results);
        }

        batch.commit().map_err(diom_error::Error::from)?;

        // Restore original message ordering.
        results.sort_by_key(|(idx, _)| *idx);
        let msgs = results.into_iter().map(|(_, msg)| msg).collect();

        Ok(PublishResponseData { msgs })
    }
}

#[tracing::instrument(skip_all, level = "debug", fields(partition = partition.get(), msg_count = msgs.len()))]
fn write_partition(
    state: &State,
    namespace_id: NamespaceId,
    topic: &RawTopic,
    partition: Partition,
    msgs: Vec<(usize, MsgIn)>,
    created_at: Timestamp,
    batch: &mut OwnedWriteBatch,
) -> diom_operations::Result<Vec<(usize, PublishedMsg)>> {
    let start_offset = MsgRow::next_offset(state, namespace_id, partition)?;
    let mut results = Vec::with_capacity(msgs.len());

    for (i, (original_idx, msg)) in msgs.into_iter().enumerate() {
        let offset = start_offset + i as Offset;
        let row = MsgRow {
            value: msg.value,
            headers: msg.headers,
            created_at,
        };
        let key = msg_row_key(namespace_id, partition, offset);
        batch.insert(&state.msg_table, key, row.to_fjall_value()?);
        results.push((
            original_idx,
            PublishedMsg {
                topic: Topic::new(topic.clone(), partition),
                offset,
            },
        ));
    }

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedMsg {
    pub topic: Topic,
    pub offset: Offset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResponseData {
    pub msgs: Vec<PublishedMsg>,
}

impl MsgsRequest for PublishOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> PublishResponse {
        PublishResponse(self.apply_real(state.msgs))
    }
}
