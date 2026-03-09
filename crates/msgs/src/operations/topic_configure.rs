use coyote_namespace::entities::NamespaceId;
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use coyote_error::Error;

use crate::{
    State,
    entities::{MAX_PARTITION_COUNT, TopicId, TopicName, gen_topic_id},
    tables::TopicRow,
};

use super::{MsgsRaftState, MsgsRequest, TopicConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureOperation {
    namespace_id: NamespaceId,
    pub(crate) topic: TopicName,
    partitions: u16,
    now: Timestamp,
    topic_id: TopicId,
}

impl TopicConfigureOperation {
    pub fn new(
        namespace_id: NamespaceId,
        topic: TopicName,
        partitions: u16,
    ) -> Result<Self, Error> {
        if partitions == 0 || partitions > MAX_PARTITION_COUNT {
            return Err(Error::invalid_user_input(format!(
                "Partition count must be between 1 and {MAX_PARTITION_COUNT}."
            )));
        }
        Ok(Self {
            namespace_id,
            topic,
            partitions,
            now: Timestamp::now(),
            topic_id: gen_topic_id(),
        })
    }

    fn apply_real(self, state: &State) -> coyote_operations::Result<TopicConfigureResponseData> {
        let mut topic_row = match TopicRow::fetch(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
        )? {
            Some(topic_row) => topic_row,
            None => TopicRow::new(self.topic_id, self.topic.clone()),
        };

        if self.partitions < topic_row.partitions {
            return Err(Error::invalid_user_input(
                "Cannot decrease partition count. Only increases are allowed.",
            )
            .into());
        }

        topic_row.partitions = self.partitions;

        let mut batch = state.db.batch();
        batch.insert_row(
            &state.metadata_tables,
            TopicRow::key_for(self.namespace_id, &self.topic),
            &topic_row,
        )?;
        batch.commit().map_err(Error::from)?;

        Ok(TopicConfigureResponseData {
            partitions: self.partitions,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureResponseData {
    pub partitions: u16,
}

impl MsgsRequest for TopicConfigureOperation {
    fn apply(self, state: MsgsRaftState<'_>) -> TopicConfigureResponse {
        TopicConfigureResponse(self.apply_real(state.msgs))
    }
}
