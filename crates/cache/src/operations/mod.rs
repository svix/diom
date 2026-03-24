use crate::State;

mod create_namespace;
mod delete;
mod set;

pub use create_namespace::{CreateCacheOperation, CreateCacheResponseData};
pub use delete::{DeleteOperation, DeleteResponseData};
pub use set::SetOperation;

use diom_operations::raft_module_operations;

pub struct CacheRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    CacheRequest,
    CacheOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> DeleteResponseData,
        CreateCache(CreateCacheOperation) -> CreateCacheResponseData,
    },
    state = CacheRaftState<'_>,
);

impl CacheOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::CreateCache(op) => Some(&op.name),
        }
    }
}
