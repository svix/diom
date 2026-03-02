use super::{InternalRequest, RecordLogTimestampResponse as Response};
use crate::core::cluster::{NodeId, state_machine::Store};
use coyote_error::Error;
use jiff::Timestamp;
use openraft::LogId;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordLogTimestampOperation {
    pub timestamp: Timestamp,
}

impl InternalRequest for RecordLogTimestampOperation {
    async fn apply(self, (state, log_id): (&mut Store, LogId<NodeId>)) -> Response {
        state
            .logs
            .record_log_timestamp(self.timestamp, log_id.index)
            .await
            .map_err(|e| Error::generic(e).into())
            .pipe(Response)
    }
}
