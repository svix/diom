// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsSvixPoller<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsSvixPoller<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create a Svix poller configuration for a topic.
    pub async fn create(
        &self,
        svix_poller_create_in: SvixPollerCreateIn,
    ) -> Result<SvixPollerCreateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.svix-poller.create")
            .with_body(svix_poller_create_in)
            .execute(self.cfg)
            .await
    }

    /// Delete a Svix poller configuration.
    pub async fn delete(
        &self,
        svix_poller_delete_in: SvixPollerDeleteIn,
    ) -> Result<SvixPollerDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.svix-poller.delete")
            .with_body(svix_poller_delete_in)
            .execute(self.cfg)
            .await
    }

    /// List Svix poller configurations for a topic.
    pub async fn list(
        &self,
        topic: String,
        svix_poller_list_in: SvixPollerListIn,
    ) -> Result<ListResponseSvixPollerOut> {
        let svix_poller_list_in = SvixPollerListIn_ {
            namespace: svix_poller_list_in.namespace,
            topic,
            limit: svix_poller_list_in.limit,
            iterator: svix_poller_list_in.iterator,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.svix-poller.list")
            .with_body(svix_poller_list_in)
            .execute(self.cfg)
            .await
    }
}
