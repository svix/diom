// this file is @generated
use serde::{Deserialize, Serialize};

use super::{seek_position::SeekPosition, sink_config::SinkConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SinkConfigureIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// The topic whose messages are forwarded to the sink. Created automatically if it does not
    /// exist.
    pub topic: String,

    /// The consumer group that identifies the sink and tracks its progress through the topic.
    pub consumer_group: String,

    /// Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_starting_position: Option<SeekPosition>,

    /// At most how many concurrent requests will be sent to the Sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_in_flight: Option<u32>,

    pub config: SinkConfig,
}

impl SinkConfigureIn {
    pub fn new(topic: String, consumer_group: String, config: SinkConfig) -> Self {
        Self {
            namespace: None,
            topic,
            consumer_group,
            default_starting_position: None,
            max_in_flight: None,
            config,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
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
