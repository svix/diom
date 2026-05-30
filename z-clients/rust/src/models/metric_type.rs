// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "counter")]
    Counter,
    #[serde(rename = "gauge")]
    Gauge,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
        };
        f.write_str(value)
    }
}
