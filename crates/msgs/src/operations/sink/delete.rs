use diom_core::PersistableValue;
use diom_error::Result;
use diom_id::NamespaceId;
use fjall_utils::{KeyspaceExt, TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::{
    entities::{ConsumerGroup, TopicName},
    operations::{MsgsRaftState, MsgsRequest, SinkDeleteResponse},
    storage::{SinkKey, SinkRow, StreamLeaseKey, StreamLeaseRow, TopicKey, TopicRow},
};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SinkDeleteOperation {
    pub namespace_id: NamespaceId,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
}

impl SinkDeleteOperation {
    pub fn new(namespace_id: NamespaceId, topic: TopicName, consumer_group: ConsumerGroup) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
        }
    }

    fn apply_real(self, state: &crate::State) -> Result<SinkDeleteResponseData> {
        let success = self.remove_sink(state)?;

        Ok(SinkDeleteResponseData {
            topic: self.topic,
            consumer_group: self.consumer_group,
            success,
        })
    }

    /// Removes the sink config along with the stream lease/cursor rows it accumulated under its
    /// consumer group, so the group's progress doesn't linger (and can't be inherited by a sink or
    /// stream consumer that later reuses the same consumer group). Returns false if the sink
    /// doesn't actually exist.
    fn remove_sink(&self, state: &crate::State) -> Result<bool> {
        let Some(topic_row) = TopicRow::fetch(
            &state.metadata_tables,
            TopicKey::build_key(&self.namespace_id, &self.topic),
        )?
        else {
            return Ok(false);
        };

        let sink_key = SinkKey::build_key(&self.namespace_id, &topic_row.id, &self.consumer_group);
        if state
            .metadata_tables
            .get_row::<SinkRow, _>(sink_key.clone())?
            .is_none()
        {
            return Ok(false);
        }

        let mut batch = state.db.batch();
        batch.remove_row::<SinkRow, _>(&state.metadata_tables, sink_key)?;

        // The sink consumes each partition as a stream consumer, leaving one lease/cursor row per
        // partition keyed by its consumer group.
        for partition in topic_row.partitions() {
            let lease_key =
                StreamLeaseKey::build_key(&topic_row.id, &partition?, &self.consumer_group);
            batch.remove_row::<StreamLeaseRow, _>(&state.metadata_tables, lease_key)?;
        }

        batch.commit()?;
        Ok(true)
    }
}

impl MsgsRequest for SinkDeleteOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> SinkDeleteResponse {
        SinkDeleteResponse::new(self.apply_real(state.msgs))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkDeleteResponseData {
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub success: bool,
}
