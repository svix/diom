use std::collections::BTreeMap;

use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

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
    pub fn new(
        db: &impl ReadableDatabase,
        namespace_id: NamespaceId,
        topic: TopicIn,
        msgs: Vec<MsgIn>,
    ) -> coyote_error::Result<Self> {
        let partition_count = crate::topic_partition_count(db, namespace_id, topic.raw_topic())?;

        let (topic, fallback_partition) = match topic {
            TopicIn::WithPartition(t) => (t.raw, t.partition),
            TopicIn::Raw(raw) => (raw, random_partition(partition_count)),
        };

        if fallback_partition.get() >= partition_count {
            return Err(coyote_error::Error::http(
                coyote_error::HttpError::bad_request(
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

    fn apply_real(self, state: &State) -> coyote_operations::Result<PublishResponseData> {
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
            let start_offset = MsgRow::next_offset(state, self.namespace_id, partition)?;

            for (i, (original_idx, msg)) in msgs.into_iter().enumerate() {
                let offset = start_offset + i as Offset;
                let row = MsgRow {
                    value: msg.value,
                    headers: msg.headers,
                    created_at,
                };
                let key = msg_row_key(self.namespace_id, partition, offset);
                batch.insert(&state.msg_table, key, row.to_fjall_value()?);
                results.push((
                    original_idx,
                    PublishedMsg {
                        topic: Topic::new(self.topic.clone(), partition),
                        offset,
                    },
                ));
            }
        }

        batch.commit().map_err(coyote_error::Error::from)?;

        // Restore original message ordering.
        results.sort_by_key(|(idx, _)| *idx);
        let msgs = results.into_iter().map(|(_, msg)| msg).collect();

        Ok(PublishResponseData { msgs })
    }
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
