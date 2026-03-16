// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Admin<'a> {
    cfg: &'a Configuration,
}

impl<'a> Admin<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Get information about the current cluster
    pub async fn cluster_status(&self) -> Result<ClusterStatusOut> {
        crate::request::Request::new(http::Method::GET, "/api/v1/admin/cluster-status")
            .execute(self.cfg)
            .await
    }
}
