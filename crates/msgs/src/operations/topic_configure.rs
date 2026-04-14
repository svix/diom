use diom_core::task::spawn_blocking_in_current_span;
use diom_error::{Error, Result};
use diom_id::{NamespaceId, UuidV7RandomBytes};
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{MAX_PARTITION_COUNT, TopicName},
    tables::{TopicKey, TopicRow},
};

use super::{MsgsRaftState, MsgsRequest, TopicConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partitions: u16,
    topic_id_random_bytes: UuidV7RandomBytes,
}

impl TopicConfigureOperation {
    pub fn new(namespace_id: NamespaceId, topic: TopicName, partitions: u16) -> Result<Self> {
        if partitions == 0 || partitions > MAX_PARTITION_COUNT {
            return Err(Error::invalid_user_input(format!(
                "Partition count must be between 1 and {MAX_PARTITION_COUNT}."
            )));
        }
        Ok(Self {
            namespace_id,
            topic,
            partitions,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        })
    }

    async fn apply_real(self, state: &State, now: Timestamp) -> Result<TopicConfigureResponseData> {
        let state = state.clone();
        spawn_blocking_in_current_span(move || {
            let mut topic_row = match TopicRow::fetch(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
            )? {
                Some(topic_row) => topic_row,
                None => TopicRow::new(self.topic.clone(), now, self.topic_id_random_bytes),
            };

            if self.partitions < topic_row.partitions {
                return Err(Error::invalid_user_input(
                    "Cannot decrease partition count. Only increases are allowed.",
                ));
            }

            topic_row.partitions = self.partitions;

            let mut batch = state.db.batch();
            batch.insert_row(
                &state.metadata_tables,
                TopicKey::build_key(&self.namespace_id, &self.topic),
                &topic_row,
            )?;
            batch.commit().map_err(Error::from)?;

            Ok(TopicConfigureResponseData {
                partitions: self.partitions,
            })
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureResponseData {
    pub partitions: u16,
}

impl MsgsRequest for TopicConfigureOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> TopicConfigureResponse {
        TopicConfigureResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
