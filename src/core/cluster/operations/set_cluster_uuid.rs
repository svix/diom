use super::{InternalRequest, SetClusterUuidResponse as Response};
use crate::core::cluster::{
    NodeId,
    state_machine::{ClusterId, Store},
};
use diom_error::Error;
use openraft::LogId;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetClusterUuidOperation(pub ClusterId);

impl InternalRequest for SetClusterUuidOperation {
    async fn apply(self, (state, _): (&mut Store, LogId<NodeId>)) -> Response {
        state
            .set_cluster_id(self.0)
            .await
            .map_err(|e| Error::generic(e).into())
            .pipe(Response)
    }
}
