use crate::core::cluster::state_machine::Store;

use diom_operations::raft_module_operations;

mod advance_feature_version;
mod record_log_timestamp;
mod set_cluster_uuid;
mod tick;

pub(super) use advance_feature_version::AdvanceFeatureVersionOperation;
pub(super) use record_log_timestamp::RecordLogTimestampOperation;
pub(super) use set_cluster_uuid::SetClusterUuidOperation;
pub(super) use tick::TickOperation;

raft_module_operations!(
    InternalRequest,
    InternalOperation {
        SetClusterUuid(SetClusterUuidOperation) -> (),
        RecordLogTimestamp(RecordLogTimestampOperation) -> (),
        Tick(TickOperation) -> (),
        AdvanceFeatureVersion(AdvanceFeatureVersionOperation) -> (),
    },
    state = &mut Store
);
