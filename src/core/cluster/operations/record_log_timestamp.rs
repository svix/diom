use super::{InternalRequest, RecordLogTimestampResponse as Response};
use crate::core::cluster::state_machine::Store;
use diom_core::PersistableValue;
use diom_error::Error;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct RecordLogTimestampOperation {}

impl InternalRequest for RecordLogTimestampOperation {
    async fn apply(self, state: &mut Store, context: &diom_operations::OpContext) -> Response {
        state
            .logs
            .record_log_timestamp(context.timestamp, context.log_index)
            .await
            .map_err(|e| Error::internal(e).into())
            .pipe(Response)
    }
}
