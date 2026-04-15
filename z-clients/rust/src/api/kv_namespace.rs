// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct KvNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> KvNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Configure KV namespace
    pub async fn configure(
        &self,
        kv_configure_namespace_in: KvConfigureNamespaceIn,
    ) -> Result<KvConfigureNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.kv.namespace.configure")
            .with_body(kv_configure_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get KV namespace
    pub async fn get(&self, kv_get_namespace_in: KvGetNamespaceIn) -> Result<KvGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.kv.namespace.get")
            .with_body(kv_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
