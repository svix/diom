use super::{InternalRequest, TickResponse as Response};
use crate::core::cluster::{NodeId, state_machine::Store};
use openraft::LogId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickOperation {}

impl InternalRequest for TickOperation {
    async fn apply(
        self,
        _state: (&mut Store, LogId<NodeId>),
        _timestamp: jiff::Timestamp,
    ) -> Response {
        // This job does nothing except periodically write something to the log
        // (because openraft doesn't let us customize the existing Heartbeat event)
        Response(Ok(()))
    }
}
