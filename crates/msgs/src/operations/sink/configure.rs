use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::{NamespaceId, UuidV7RandomBytes};
use fjall_utils::WriteBatchExt;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{ConsumerGroup, SinkSettings, TopicName},
    operations::{MsgsRaftState, MsgsRequest, SinkConfigureResponse},
    storage::{SinkKey, SinkRow, TopicRow},
};

/// Creates or updates a sink in place. Overwrites any existing sink with the same consumer group,
/// and auto-creates the topic if it does not yet exist.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SinkConfigureOperation {
    pub namespace_id: NamespaceId,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub settings: SinkSettings,
    topic_id_random_bytes: UuidV7RandomBytes,
}

impl SinkConfigureOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        consumer_group: ConsumerGroup,
        settings: SinkSettings,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            settings,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    fn apply_real(
        self,
        state: &crate::State,
        now: UnixTimestampMs,
    ) -> Result<SinkConfigureResponseData> {
        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch_or_create(
            &state.metadata_tables,
            &mut batch,
            self.namespace_id,
            &self.topic,
            now,
            self.topic_id_random_bytes,
        )?;

        let key = SinkKey::build_key(&self.namespace_id, &topic_row.id, &self.consumer_group);
        let row = SinkRow {
            topic: self.topic.clone(),
            settings: self.settings,
        };
        batch.insert_row(&state.metadata_tables, key, &row)?;
        batch.commit()?;

        Ok(SinkConfigureResponseData {
            topic: self.topic,
            consumer_group: self.consumer_group,
        })
    }
}

impl MsgsRequest for SinkConfigureOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> SinkConfigureResponse {
        SinkConfigureResponse::new(self.apply_real(state.msgs, ctx.timestamp))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkConfigureResponseData {
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
}
