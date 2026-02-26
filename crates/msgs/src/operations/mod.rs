mod create_namespace;

pub use self::create_namespace::*;

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
    },
    state = MsgsRaftState<'_>,
);
