use super::{InternalRequest, SetClusterUuidResponse as Response};
use crate::core::cluster::state_machine::{ClusterId, Store};
use diom_core::PersistableValue;
use diom_error::Error;
use serde::{Deserialize, Serialize};
use tap::Pipe;

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SetClusterUuidOperation(pub ClusterId);

impl InternalRequest for SetClusterUuidOperation {
    async fn apply(self, state: &mut Store, _ctx: &diom_operations::OpContext) -> Response {
        state
            .set_cluster_id(self.0)
            .await
            .map_err(|e| Error::internal(e).into())
            .pipe(Response)
    }
}
