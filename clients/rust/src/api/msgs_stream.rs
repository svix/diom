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

    /// Commits an offset for a consumer group on a specific partition.
    ///
    /// The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
    /// successfully processed offset; future receives will start after it.
    pub async fn commit(&self, stream_commit_in: StreamCommitIn) -> Result<StreamCommitOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/msgs/stream/commit")
            .with_body(stream_commit_in)
            .execute(self.cfg)
            .await
    }
}
