use super::{InternalRequest, RecordLogTimestampResponse as Response};
use crate::core::cluster::state_machine::Store;
use coyote_error::Error;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordLogTimestampOperation {}

impl InternalRequest for RecordLogTimestampOperation {
    async fn apply(self, state: &mut Store, context: &coyote_operations::OpContext) -> Response {
        state
            .logs
            .record_log_timestamp(context.timestamp, context.log_index)
            .await
            .map_err(|e| Error::internal(e).into())
            .pipe(Response)
    }
}
