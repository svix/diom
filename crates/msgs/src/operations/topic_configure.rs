use diom_namespace::entities::NamespaceId;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{MAX_PARTITION_COUNT, RawTopic},
    tables::{TopicConfig, TopicConfigRow, topic_partition_count},
};

use super::{MsgsRaftState, MsgsRequest, TopicConfigureResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicConfigureOperation {
    namespace_id: NamespaceId,
    topic: RawTopic,
    partitions: u16,
}

impl TopicConfigureOperation {
    pub fn new(namespace_id: NamespaceId, topic: RawTopic, partitions: u16) -> Self {
        Self {
            namespace_id,
            topic,
            partitions,
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

        let current = topic_partition_count(state, self.namespace_id, &self.topic)?;

        if self.partitions < current {
            return Err(diom_error::Error::http(
                diom_error::HttpError::bad_request(
                    Some("cannot_decrease_partitions".to_owned()),
                    Some(format!(
                        "Cannot decrease partition count from {current} to {}. Only increases are allowed.",
                        self.partitions
                    )),
                ),
            )
            .into());
        }

        let config = TopicConfig {
            partition_count: self.partitions,
        };

        let mut batch = state.db.batch();
        TopicConfigRow::store(&mut batch, state, self.namespace_id, &self.topic, &config)?;
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
