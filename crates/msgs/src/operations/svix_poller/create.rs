use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::{NamespaceId, UuidV7RandomBytes};
use fjall_utils::WriteBatchExt;
use serde::{Deserialize, Serialize};

use crate::{
    entities::TopicName,
    operations::{MsgsRaftState, MsgsRequest, SvixPollerCreateResponse},
    storage::{SvixPollerKey, SvixPollerRow, TopicRow},
};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SvixPollerCreateOperation {
    pub namespace_id: NamespaceId,
    pub topic: TopicName,
    pub poller_id: String,
    pub token: String,
    topic_id_random_bytes: UuidV7RandomBytes,
}

impl SvixPollerCreateOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        poller_id: String,
        token: String,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            poller_id,
            token,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    fn apply_real(
        self,
        state: &crate::State,
        now: UnixTimestampMs,
    ) -> Result<SvixPollerCreateResponseData> {
        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch_or_create(
            &state.metadata_tables,
            &mut batch,
            self.namespace_id,
            &self.topic,
            now,
            self.topic_id_random_bytes,
        )?;

        let key = SvixPollerKey::build_key(&self.namespace_id, &topic_row.id, &self.poller_id);
        let row = SvixPollerRow {
            topic: self.topic.clone(),
            token: self.token,
        };
        batch.insert_row(&state.metadata_tables, key, &row)?;
        batch.commit()?;

        Ok(SvixPollerCreateResponseData {
            topic: self.topic,
            poller_id: self.poller_id,
        })
    }
}

impl MsgsRequest for SvixPollerCreateOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> SvixPollerCreateResponse {
        SvixPollerCreateResponse::new(self.apply_real(state.msgs, ctx.timestamp))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvixPollerCreateResponseData {
    pub topic: TopicName,
    pub poller_id: String,
}
