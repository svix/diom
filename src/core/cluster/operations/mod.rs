use crate::core::cluster::{NodeId, state_machine::Store};
use openraft::LogId;
use serde::{Deserialize, Serialize};

use diom_operations::async_raft_module_operations;

mod record_log_timestamp;
mod set_cluster_uuid;

pub(super) use record_log_timestamp::RecordLogTimestampOperation;
pub(super) use set_cluster_uuid::SetClusterUuidOperation;

async_raft_module_operations!(
    InternalRequest,
    InternalOperation {
        SetClusterUuid(SetClusterUuidOperation) -> (),
        RecordLogTimestamp(RecordLogTimestampOperation) -> (),
    },
    state = (&mut Store, LogId<NodeId>)
);
