use diom_core::PersistableValue;
use diom_error::Result;
use diom_id::NamespaceId;
use fjall_utils::{KeyspaceExt, TableRow};
use serde::{Deserialize, Serialize};

use crate::{
    entities::TopicName,
    operations::{MsgsRaftState, MsgsRequest, SvixPollerDeleteResponse},
    storage::{SvixPollerKey, SvixPollerRow, TopicKey, TopicRow},
};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SvixPollerDeleteOperation {
    pub namespace_id: NamespaceId,
    pub topic: TopicName,
    pub poller_id: String,
}

impl SvixPollerDeleteOperation {
    pub fn new(namespace_id: NamespaceId, topic: TopicName, poller_id: String) -> Self {
        Self {
            namespace_id,
            topic,
            poller_id,
        }
    }

    fn apply_real(self, state: &crate::State) -> Result<SvixPollerDeleteResponseData> {
        let success = self.remove_poller(state)?;

        Ok(SvixPollerDeleteResponseData {
            topic: self.topic,
            poller_id: self.poller_id,
            success,
        })
    }

    /// Returns false if the poller doesn't actually exist.
    fn remove_poller(&self, state: &crate::State) -> Result<bool> {
        let Some(topic_row) = TopicRow::fetch(
            &state.metadata_tables,
            TopicKey::build_key(&self.namespace_id, &self.topic),
        )?
        else {
            return Ok(false);
        };

        let key = SvixPollerKey::build_key(&self.namespace_id, &topic_row.id, &self.poller_id);
        if state
            .metadata_tables
            .get_row::<SvixPollerRow, _>(key)?
            .is_none()
        {
            return Ok(false);
        }

        let key = SvixPollerKey::build_key(&self.namespace_id, &topic_row.id, &self.poller_id);
        state.metadata_tables.remove_row::<SvixPollerRow, _>(key)?;
        Ok(true)
    }
}

impl MsgsRequest for SvixPollerDeleteOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> SvixPollerDeleteResponse {
        SvixPollerDeleteResponse::new(self.apply_real(state.msgs))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvixPollerDeleteResponseData {
    pub topic: TopicName,
    pub poller_id: String,
    pub success: bool,
}
