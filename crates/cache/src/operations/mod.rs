use crate::State;

mod clear_expired;
mod create_namespace;
mod delete;
mod set;

pub use clear_expired::ClearExpiredOperation;
pub use create_namespace::{CreateCacheOperation, CreateCacheResponseData};
pub use delete::{DeleteOperation, DeleteResponseData};
pub use set::SetOperation;

use coyote_operations::async_raft_module_operations;

pub struct CacheRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

async_raft_module_operations!(
    CacheRequest,
    CacheOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> DeleteResponseData,
        CreateCache(CreateCacheOperation) -> CreateCacheResponseData,
        ClearExpired(ClearExpiredOperation) -> (),
    },
    state = CacheRaftState<'_>,
);

impl CacheOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::CreateCache(op) => Some(&op.name),
            Self::ClearExpired(_) => None,
        }
    }
}
