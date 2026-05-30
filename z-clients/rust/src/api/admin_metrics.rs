// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AdminMetrics<'a> {
    cfg: &'a Configuration,
}

impl<'a> AdminMetrics<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)
    pub async fn get(&self) -> Result<GetMetricsOut> {
        crate::request::Request::new(http::Method::GET, "/api/v1.admin.metrics.get")
            .execute(self.cfg)
            .await
    }
}
