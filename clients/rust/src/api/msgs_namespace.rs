// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Creates or updates a msgs namespace with the given name.
    pub async fn create(
        &self,
        msg_namespace_create_in: MsgNamespaceCreateIn,
    ) -> Result<MsgNamespaceCreateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/namespace/create")
            .with_body(msg_namespace_create_in)
            .execute(self.cfg)
            .await
    }

    /// Gets a msgs namespace by name.
    pub async fn get(&self, msg_namespace_get_in: MsgNamespaceGetIn) -> Result<MsgNamespaceGetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/namespace/get")
            .with_body(msg_namespace_get_in)
            .execute(self.cfg)
            .await
    }
}
