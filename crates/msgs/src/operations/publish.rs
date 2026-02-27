use std::collections::BTreeMap;

use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{MsgIn, Offset, PartitionIndex, partition_for_key},
    tables::{MsgRow, msg_row_key},
};

use super::{MsgsRaftState, MsgsRequest, PublishResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOperation {
    namespace_id: NamespaceId,
    topic: String,
    msgs: Vec<MsgIn>,
    /// Partition assigned to messages without a key.
    ///
    /// Chosen randomly by the caller so the Raft state machine stays deterministic.
    keyless_partition: PartitionIndex,
    created_at: Timestamp,
}

impl PublishOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: String,
        msgs: Vec<MsgIn>,
        keyless_partition: PartitionIndex,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            msgs,
            keyless_partition,
            created_at: Timestamp::now(),
        }
    }

    fn apply_real(self, state: &State) -> coyote_operations::Result<PublishResponseData> {
        let mut by_partition: BTreeMap<PartitionIndex, Vec<(usize, MsgIn)>> = BTreeMap::new();

        for (idx, msg) in self.msgs.into_iter().enumerate() {
            let partition = if let Some(key) = &msg.key {
                partition_for_key(key.as_bytes())
            } else {
                self.keyless_partition
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
                results.push((original_idx, PublishedMsg { partition, offset }));
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
    pub partition: PartitionIndex,
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
