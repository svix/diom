// this file is @generated
use serde::{Deserialize, Serialize};

use super::metric_type::MetricType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetricOut {
    /// Label for this series
    pub label: String,

    /// Human-readable description of this series
    pub description: String,

    /// Key/Value pairs attached to this sequence
    pub attributes: std::collections::HashMap<String, String>,

    /// Most recent data point for this series
    ///
    /// All points (u64, i64, and f64) are squished into an f64, be careful
    /// of inexactness for values above 2**53.
    pub value: f64,

    /// Type of this metric
    ///
    /// Histograms are not currently exported through this API, and can
    /// only be accessed through OTLP.
    pub metric_type: MetricType,

    /// Timestamp this metric was collected
    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub timestamp: jiff::Timestamp,

    /// Optional unit, following UCUM unit conventions if possible
    ///
    /// See https://ucum.org/ for details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl MetricOut {
    pub fn new(
        label: String,
        description: String,
        attributes: std::collections::HashMap<String, String>,
        value: f64,
        metric_type: MetricType,
        timestamp: jiff::Timestamp,
    ) -> Self {
        Self {
            label,
            description,
            attributes,
            value,
            metric_type,
            timestamp,
            unit: None,
        }
    }

    pub fn with_unit(mut self, value: impl Into<Option<String>>) -> Self {
        self.unit = value.into();
        self
    }
}
