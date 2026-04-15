// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Configures a msgs namespace with the given name.
    pub async fn configure(
        &self,
        name: String,
        msg_namespace_configure_in: MsgNamespaceConfigureIn,
    ) -> Result<MsgNamespaceConfigureOut> {
        let msg_namespace_configure_in = MsgNamespaceConfigureIn_ {
            name,
            retention: msg_namespace_configure_in.retention,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.namespace.configure")
            .with_body(msg_namespace_configure_in)
            .execute(self.cfg)
            .await
    }

    /// Gets a msgs namespace by name.
    pub async fn get(
        &self,
        name: String,
        msg_namespace_get_in: MsgNamespaceGetIn,
    ) -> Result<MsgNamespaceGetOut> {
        let _unused = msg_namespace_get_in;
        let msg_namespace_get_in = MsgNamespaceGetIn_ { name };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.namespace.get")
            .with_body(msg_namespace_get_in)
            .execute(self.cfg)
            .await
    }
}
