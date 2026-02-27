mod create_namespace;
mod publish;
mod stream_commit;
mod stream_receive;

pub use self::{create_namespace::*, publish::*, stream_commit::*, stream_receive::*};

use crate::State;
use serde::{Deserialize, Serialize};

use coyote_operations::raft_module_operations;

pub struct MsgsRaftState<'a> {
    pub msgs: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    MsgsRequest,
    MsgsOperation {
        CreateNamespace(CreateNamespaceOperation) -> CreateNamespaceResponseData,
        Publish(PublishOperation) -> PublishResponseData,
        StreamCommit(StreamCommitOperation) -> StreamCommitResponseData,
        StreamReceive(StreamReceiveOperation) -> StreamReceiveResponseData,
    },
    state = MsgsRaftState<'_>,
);
