use super::{InternalRequest, TickResponse as Response};
use crate::core::cluster::state_machine::Store;
use diom_core::PersistableValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct TickOperation {}

impl InternalRequest for TickOperation {
    async fn apply(self, _state: &mut Store, _context: &diom_operations::OpContext) -> Response {
        // This job does nothing except periodically write something to the log
        // (because openraft doesn't let us customize the existing Heartbeat event)
        Response(Ok(()))
    }
}
