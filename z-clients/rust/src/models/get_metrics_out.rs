// this file is @generated
use serde::{Deserialize, Serialize};

use super::metric_out::MetricOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetMetricsOut {
    pub metrics: Vec<MetricOut>,
}

impl GetMetricsOut {
    pub fn new(metrics: Vec<MetricOut>) -> Self {
        Self { metrics }
    }
}
