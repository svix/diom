mod create_namespace;
mod publish;
mod stream_receive;

pub use self::{create_namespace::*, publish::*, stream_receive::*};

use crate::State;
use serde::{Deserialize, Serialize};

use diom_operations::raft_module_operations;

pub struct MsgsRaftState<'a> {
    pub msgs: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    MsgsRequest,
    MsgsOperation {
        CreateNamespace(CreateNamespaceOperation) -> CreateNamespaceResponseData,
        Publish(PublishOperation) -> PublishResponseData,
        StreamReceive(StreamReceiveOperation) -> StreamReceiveResponseData,
    },
    state = MsgsRaftState<'_>,
);
