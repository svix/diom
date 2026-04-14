use diom_core::task::spawn_blocking_in_current_span;
use diom_error::{Error, Result};
use diom_id::{NamespaceId, UuidV7RandomBytes};
use fjall_utils::WriteBatchExt;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, TopicName},
    tables::{QueueConfigKey, QueueConfigRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigureOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    retry_schedule: Vec<u64>,
    dlq_topic: Option<TopicName>,
    topic_id_random_bytes: UuidV7RandomBytes,
}

impl QueueConfigureOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        consumer_group: ConsumerGroup,
        retry_schedule: Vec<u64>,
        dlq_topic: Option<TopicName>,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            retry_schedule,
            dlq_topic,
            topic_id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    async fn apply_real(self, state: &State, now: Timestamp) -> Result<QueueConfigureResponseData> {
        let state = state.clone();
        spawn_blocking_in_current_span(move || {
            let mut batch = state.db.batch();

            let topic_row = TopicRow::fetch_or_create(
                &state.metadata_tables,
                &mut batch,
                self.namespace_id,
                &self.topic,
                now,
                self.topic_id_random_bytes,
            )?;

            let config = QueueConfigRow {
                retry_schedule: self.retry_schedule,
                dlq_topic: self.dlq_topic,
            };

            batch.insert_row(
                &state.metadata_tables,
                QueueConfigKey::build_key(&topic_row.id, &self.consumer_group),
                &config,
            )?;
            batch.commit().map_err(Error::from)?;

            Ok(QueueConfigureResponseData {
                retry_schedule: config.retry_schedule,
                dlq_topic: config.dlq_topic,
            })
        })
        .await?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigureResponseData {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

impl MsgsRequest for QueueConfigureOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> QueueConfigureResponse {
        QueueConfigureResponse::new(self.apply_real(state.msgs, ctx.timestamp).await)
    }
}
