use super::{InternalRequest, RecordLogTimestampResponse as Response};
use crate::core::cluster::{NodeId, state_machine::Store};
use coyote_error::Error;
use openraft::LogId;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordLogTimestampOperation {}

impl InternalRequest for RecordLogTimestampOperation {
    async fn apply(
        self,
        (state, log_id): (&mut Store, LogId<NodeId>),
        timestamp: jiff::Timestamp,
    ) -> Response {
        state
            .logs
            .record_log_timestamp(timestamp, log_id.index)
            .await
            .map_err(|e| Error::generic(e).into())
            .pipe(Response)
    }
}
