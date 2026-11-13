use super::{AdvanceFeatureVersionResponse as Response, InternalRequest};
use crate::core::cluster::state_machine::Store;
use diom_core::PersistableValue;
use diom_error::Error;
use diom_operations::FeatureVersion;
use serde::{Deserialize, Serialize};

/// Raises the cluster feature version to the carried value. The setter is monotonic, so applying this
/// on every node deterministically advances the version and never lowers it. The leader only emits it
/// once every voter advertises support for the target version.
#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct AdvanceFeatureVersionOperation(pub FeatureVersion);

impl InternalRequest for AdvanceFeatureVersionOperation {
    async fn apply(self, state: &mut Store, _ctx: &diom_operations::OpContext) -> Response {
        if let Err(e) = state.set_feature_version(self.0).await {
            return Response(Err(Error::internal(e).into()));
        }
        // Refuse to apply anything further so the apply loop halts here, before it reaches an
        // operation gated on this version. The shutdown started above still tears the node down.
        if !state.supports_feature_version(self.0) {
            return Response(Err(Error::internal(format!(
                "refusing to apply past feature version {} which this build does not support",
                self.0
            ))
            .into()));
        }
        Response(Ok(()))
    }
}
