use coyote_error::Error;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgId, TopicName},
    tables::{QueueLeaseRow, TopicRow},
};

use super::super::{MsgsRaftState, MsgsRequest, QueueNackResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueNackOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    consumer_group: ConsumerGroup,
    msg_ids: Vec<MsgId>,
}

impl QueueNackOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        consumer_group: ConsumerGroup,
        msg_ids: Vec<MsgId>,
    ) -> Self {
        Self {
            namespace_id,
            topic,
            consumer_group,
            msg_ids,
        }
    }

    #[tracing::instrument(skip_all, level = "debug")]
    fn apply_real(self, state: &State) -> coyote_operations::Result<QueueNackResponseData> {
        let topic_row = TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )?
        .ok_or_else(|| Error::invalid_user_input("topic must exist"))?;

        let mut batch = state.db.batch();

        for msg_id in &self.msg_ids {
            batch.insert_row(
                &state.metadata_tables,
                QueueLeaseRow::key_for(topic_row.id, msg_id, &self.consumer_group),
                &QueueLeaseRow {
                    expiry: Timestamp::MAX,
                    dlq: true,
                },
            )?;
        }

        batch.commit().map_err(Error::from)?;

        Ok(QueueNackResponseData {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueNackResponseData {}

impl MsgsRequest for QueueNackOperation {
    fn apply(self, state: MsgsRaftState<'_>, _timestamp: Timestamp) -> QueueNackResponse {
        QueueNackResponse(self.apply_real(state.msgs))
    }
}
