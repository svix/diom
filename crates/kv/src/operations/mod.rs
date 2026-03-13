use crate::State;

use serde::{Deserialize, Serialize};

mod clear_expired;
mod create_namespace;
mod delete;
mod set;

pub use clear_expired::ClearExpiredOperation;
pub use create_namespace::{CreateKvOperation, CreateKvResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::raft_module_operations;

pub struct KvRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    KvRequest,
    KvOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
        CreateKv(CreateKvOperation) -> CreateKvResponseData,
        ClearExpired(ClearExpiredOperation) -> (),
    },
    state = KvRaftState<'_>,
);

impl KvOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::CreateKv(op) => Some(&op.name),
            Self::ClearExpired(_) => None,
        }
    }
}
