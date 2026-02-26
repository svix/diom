mod create_namespace;
mod publish;

pub use self::{create_namespace::*, publish::*};

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
    },
    state = MsgsRaftState<'_>,
);
