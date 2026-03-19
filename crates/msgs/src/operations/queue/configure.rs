use diom_error::Error;
use diom_id::NamespaceId;
use fjall_utils::WriteBatchExt;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, TopicName},
    tables::{QueueConfigRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigureOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    retry_schedule: Vec<u64>,
    dlq_topic: Option<TopicName>,
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
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(
        self,
        state: &State,
        now: Timestamp,
    ) -> diom_operations::Result<QueueConfigureResponseData> {
        let mut batch = state.db.batch();

        let topic_row = TopicRow::fetch_or_create(
            &state.metadata_tables,
            &mut batch,
            self.namespace_id,
            &self.topic,
            now,
        )?;

        let config = QueueConfigRow {
            retry_schedule: self.retry_schedule,
            dlq_topic: self.dlq_topic,
        };

        batch.insert_row(
            &state.metadata_tables,
            QueueConfigRow::key_for(topic_row.id, &self.consumer_group),
            &config,
        )?;
        batch.commit().map_err(Error::from)?;

        Ok(QueueConfigureResponseData {
            retry_schedule: config.retry_schedule,
            dlq_topic: config.dlq_topic,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigureResponseData {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

impl MsgsRequest for QueueConfigureOperation {
    fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> QueueConfigureResponse {
        QueueConfigureResponse(self.apply_real(state.msgs, ctx.timestamp))
    }
}
