// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    http_sink_config::HttpSinkConfig, kafka_sink_config::KafkaSinkConfig,
    svix_sink_config::SvixSinkConfig,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SinkConfig {
    #[serde(rename = "http")]
    Http(HttpSinkConfig),
    #[serde(rename = "svix")]
    Svix(SvixSinkConfig),
    #[serde(rename = "kafka")]
    Kafka(KafkaSinkConfig),
}
