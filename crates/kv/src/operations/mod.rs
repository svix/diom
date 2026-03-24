use crate::State;

mod create_namespace;
mod delete;
mod set;

pub use create_namespace::{CreateKvOperation, CreateKvResponseData};
pub use delete::{DeleteOperation, DeleteResponseData};
pub use set::{SetOperation, SetResponseData};

use coyote_operations::raft_module_operations;

pub struct KvRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    KvRequest,
    KvOperation {
        Set(SetOperation) -> SetResponseData,
        Delete(DeleteOperation) -> DeleteResponseData,
        CreateKv(CreateKvOperation) -> CreateKvResponseData,
    },
    state = KvRaftState<'_>,
);

impl KvOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::CreateKv(op) => Some(&op.name),
        }
    }
}
