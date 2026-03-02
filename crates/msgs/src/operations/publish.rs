use std::{collections::BTreeMap, time::Instant};

use diom_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{MsgIn, Offset, Partition, TopicIn, TopicPartition, partition_for_key},
    tables::{MsgRow, TableRow, TopicRow},
};

use super::{MsgsRaftState, MsgsRequest, PublishResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOperation {
    namespace_id: NamespaceId,
    topic: TopicIn,
    msgs: Vec<MsgIn>,
    now: Timestamp,
}

impl PublishOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicIn,
        msgs: Vec<MsgIn>,
    ) -> diom_error::Result<Self> {
        Ok(Self {
            namespace_id,
            topic,
            msgs,
            now: Timestamp::now(),
        })
    }

    #[tracing::instrument(skip_all, level = "debug", fields(msg_count = self.msgs.len()))]
    fn apply_real(self, state: &State) -> diom_operations::Result<PublishResponseData> {
        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            self.namespace_id,
            self.topic.topic_name(),
        )?;
        let mut batch = state.db.batch();

        let topic_row = match (topic_row, &self.topic) {
            (Some(topic_row), TopicIn::TopicPartition(topic)) => {
                if !(0..topic_row.partitions).contains(&topic.partition.get()) {
                    return Err(
                        diom_error::Error::http(diom_error::HttpError::bad_request(
                            Some("partition_out_of_range".to_owned()),
                            None,
                        ))
                        .into(),
                    );
                }

                topic_row
            }
            (Some(topic_row), TopicIn::TopicName(_)) => {
                // If there isn't a partition, need to use the key or round-robin choose the right one.
                topic_row
            }
            (None, TopicIn::TopicPartition(_)) => {
                // Topic has to exist if passing a specific partition
                return Err(
                    diom_error::Error::http(diom_error::HttpError::bad_request(
                        Some("partition_must_exist".to_owned()),
                        None,
                    ))
                    .into(),
                );
            }
            (None, TopicIn::TopicName(topic)) => {
                // Need to create the topic with default settings
                let topic_row = TopicRow::new(topic.clone(), self.now)?;
                batch.insert(
                    &state.metadata_tables,
                    TopicRow::construct_key(self.namespace_id, topic),
                    topic_row.to_fjall_value()?,
                );
                topic_row
            }
        };

        tracing::Span::current().record("partition_count", topic_row.partitions);

        // Group the messages by partitions
        let mut by_partition: BTreeMap<Partition, Vec<MsgIn>> = BTreeMap::new();
        for msg in self.msgs.into_iter() {
            let partition =
                if let TopicIn::TopicPartition(TopicPartition { partition, .. }) = self.topic {
                    if msg.key.is_some() {
                        // We currently don't allow setting a key + sending to a specific partition
                        // We should probably allow it, just need to think about the right way of doing it.
                        return Err(diom_error::Error::http(
                            diom_error::HttpError::bad_request(
                                Some("both_key_and_partition_are_set".to_owned()),
                                None,
                            ),
                        )
                        .into());
                    }
                    partition
                } else {
                    partition_for_key(msg.key.as_deref(), topic_row.partitions)
                };
            by_partition.entry(partition).or_default().push(msg);
        }

        // Write the messages to the db
        let t = Instant::now();
        let mut results: Vec<PublishedTopic> = Vec::with_capacity(by_partition.keys().len());
        for (partition, msgs) in by_partition {
            let topic = TopicPartition::new(self.topic.topic_name().clone(), partition);
            let mut offset = MsgRow::next_offset(&state.msg_table, topic_row.id, partition)?;
            let start_offset = offset;

            for msg in msgs.into_iter() {
                let msg = MsgRow {
                    value: msg.value,
                    headers: msg.headers,
                    timestamp: self.now + t.elapsed(),
                };
                batch.insert(
                    &state.msg_table,
                    MsgRow::construct_key(topic_row.id, partition, offset),
                    msg.to_fjall_value()?,
                );

                offset += 1;
            }

            results.push(PublishedTopic {
                start_offset,
                topic,
                offset,
            });
        }

        batch.commit().map_err(diom_error::Error::from)?;

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
    fn apply(self, state: MsgsRaftState<'_>) -> PublishResponse {
        PublishResponse(self.apply_real(state.msgs))
    }
}
