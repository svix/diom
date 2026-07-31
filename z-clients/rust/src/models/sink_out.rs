// this file is @generated
use serde::{Deserialize, Serialize};

use super::{seek_position::SeekPosition, sink_config::SinkConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SinkOut {
    pub topic: String,

    pub consumer_group: String,

    /// Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<SeekPosition>,

    /// At most how many concurrent requests will be sent to the Sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_in_flight: Option<u32>,

    pub config: SinkConfig,
}

impl SinkOut {
    pub fn new(topic: String, consumer_group: String, config: SinkConfig) -> Self {
        Self {
            topic,
            consumer_group,
            default_starting_position: None,
            max_in_flight: None,
            config,
        }
    }

    pub fn with_default_starting_position(
        mut self,
        value: impl Into<Option<SeekPosition>>,
    ) -> Self {
        self.default_starting_position = value.into();
        self
    }

    pub fn with_max_in_flight(mut self, value: impl Into<Option<u32>>) -> Self {
        self.max_in_flight = value.into();
        self
    }
}
