use aide::axum::{ApiRouter, routing::get_with};
use axum::extract::State;
use diom_core::types::UnixTimestampMs;
use diom_derive::aide_annotate;
use diom_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::{
    AppState,
    error::Result,
    metrics::{MetricType, SerializableMetric},
    v1::utils::openapi_tag,
};

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct MetricOut {
    /// Label for this series
    label: String,
    /// Human-readable description of this series
    description: String,
    /// Key/Value pairs attached to this sequence
    attributes: BTreeMap<String, String>,
    /// Most recent data point for this series
    ///
    /// All points (u64, i64, and f64) are squished into an f64, be careful
    /// of inexactness for values above 2**53.
    value: f64,
    /// Type of this metric
    ///
    /// Histograms are not currently exported through this API, and can
    /// only be accessed through OTLP.
    metric_type: MetricType,
    /// Timestamp this metric was collected
    timestamp: UnixTimestampMs,
    /// Optional unit, following UCUM unit conventions if possible
    ///
    /// See https://ucum.org/ for details
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct GetMetricsOut {
    metrics: Vec<MetricOut>,
}

impl GetMetricsOut {
    fn from_serialized<'a>(ms: Vec<SerializableMetric<'a>>) -> Self {
        let metrics = ms
            .into_iter()
            .map(|m| MetricOut {
                label: m.label.to_owned(),
                description: m.description.to_owned(),
                unit: m.unit.map(|s| s.to_owned()),
                attributes: m
                    .resources
                    .0
                    .iter()
                    .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
                    .collect(),
                value: m.datum.into(),
                metric_type: m.metric_type,
                timestamp: UnixTimestampMs::from(m.last_fetched_at),
            })
            .collect();
        Self { metrics }
    }
}

#[aide_annotate(op_id = "v1.admin.metrics.get")]
/// Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)
async fn get_metrics(State(state): State<AppState>) -> Result<MsgPackOrJson<GetMetricsOut>> {
    let metrics = if let Some(metrics) = state.metrics.as_ref() {
        metrics.serialize_with(GetMetricsOut::from_serialized)
    } else {
        GetMetricsOut {
            metrics: Vec::new(),
        }
    };
    Ok(MsgPackOrJson(metrics))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Admin");

    ApiRouter::new().api_route_with(
        get_metrics_path,
        get_with(get_metrics, get_metrics_operation),
        &tag,
    )
}
