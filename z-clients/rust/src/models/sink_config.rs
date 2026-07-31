// this file is @generated
use serde::{Deserialize, Serialize};

use super::http_sink_config::HttpSinkConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum SinkConfig {
    #[serde(rename = "http")]
    Http(HttpSinkConfig),
}
