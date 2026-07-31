// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsSink<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsSink<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or update a sink for a topic. Overwrites any existing sink with the same id.
    pub async fn configure(&self, sink_configure_in: SinkConfigureIn) -> Result<SinkConfigureOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.sink.configure")
            .with_body(sink_configure_in)
            .execute(self.cfg)
            .await
    }

    /// Delete a sink.
    pub async fn delete(&self, sink_delete_in: SinkDeleteIn) -> Result<SinkDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.sink.delete")
            .with_body(sink_delete_in)
            .execute(self.cfg)
            .await
    }

    /// List sink configurations for a topic.
    pub async fn list(
        &self,
        topic: String,
        sink_list_in: SinkListIn,
    ) -> Result<ListResponseSinkOut> {
        let sink_list_in = SinkListIn_ {
            namespace: sink_list_in.namespace,
            topic,
            limit: sink_list_in.limit,
            iterator: sink_list_in.iterator,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.msgs.sink.list")
            .with_body(sink_list_in)
            .execute(self.cfg)
            .await
    }
}
