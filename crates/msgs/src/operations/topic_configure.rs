use diom_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{MAX_PARTITION_COUNT, TopicName},
    tables::{TableRow, TopicRow},
};

use super::{MsgsRaftState, MsgsRequest, TopicConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureOperation {
    namespace_id: NamespaceId,
    topic: TopicName,
    partitions: u16,
    now: Timestamp,
}

impl TopicConfigureOperation {
    pub fn new(namespace_id: NamespaceId, topic: TopicName, partitions: u16) -> Self {
        Self {
            namespace_id,
            topic,
            partitions,
            now: Timestamp::now(),
        }
    }

    fn apply_real(self, state: &State) -> diom_operations::Result<TopicConfigureResponseData> {
        if self.partitions == 0 || self.partitions > MAX_PARTITION_COUNT {
            return Err(
                diom_error::Error::http(diom_error::HttpError::bad_request(
                    Some("invalid_partition_count".to_owned()),
                    Some(format!(
                        "Partition count must be between 1 and {MAX_PARTITION_COUNT}."
                    )),
                ))
                .into(),
            );
        }
        let mut topic_row =
            match TopicRow::fetch(&state.metadata_tables, self.namespace_id, &self.topic)? {
                Some(topic_row) => topic_row,
                None => TopicRow::new(self.topic.clone(), self.now)?,
            };

        if self.partitions < topic_row.partitions {
            return Err(
                diom_error::Error::http(diom_error::HttpError::bad_request(
                    Some("cannot_decrease_partitions".to_owned()),
                    Some("Cannot decrease partition count. Only increases are allowed.".to_owned()),
                ))
                .into(),
            );
        }

        topic_row.partitions = self.partitions;

        let mut batch = state.db.batch();
        batch.insert(
            &state.metadata_tables,
            TopicRow::construct_key(self.namespace_id, &self.topic),
            topic_row.to_fjall_value()?,
        );
        batch.commit().map_err(diom_error::Error::from)?;

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
