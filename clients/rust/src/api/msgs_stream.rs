// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct MsgsStream<'a> {
    cfg: &'a Configuration,
}

impl<'a> MsgsStream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Receives messages from a topic using a consumer group.
    ///
    /// Each consumer in the group reads from all partitions. Messages are locked by leases for the
    /// specified duration to prevent duplicate delivery within the same consumer group.
    pub async fn receive(&self, stream_receive_in: StreamReceiveIn) -> Result<StreamReceiveOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/stream/receive")
            .with_body(stream_receive_in)
            .execute(self.cfg)
            .await
    }
}
