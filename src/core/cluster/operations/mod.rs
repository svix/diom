use crate::core::cluster::{NodeId, state_machine::Store};
use openraft::LogId;
use serde::{Deserialize, Serialize};

use coyote_operations::async_raft_module_operations;

mod record_log_timestamp;
mod set_cluster_uuid;
mod tick;

pub(super) use record_log_timestamp::RecordLogTimestampOperation;
pub(super) use set_cluster_uuid::SetClusterUuidOperation;
pub(super) use tick::TickOperation;

async_raft_module_operations!(
    InternalRequest,
    InternalOperation {
        SetClusterUuid(SetClusterUuidOperation) -> (),
        RecordLogTimestamp(RecordLogTimestampOperation) -> (),
        Tick(TickOperation) -> (),
    },
    state = (&mut Store, LogId<NodeId>)
);
