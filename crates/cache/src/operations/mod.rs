use crate::State;

use serde::{Deserialize, Serialize};

mod create_cache;
mod delete;
mod set;

pub use create_cache::{CreateCacheOperation, CreateCacheResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::raft_module_operations;

pub struct CacheRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    CacheRequest,
    CacheOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
        CreateCache(CreateCacheOperation) -> CreateCacheResponseData,
    },
    state = CacheRaftState<'_>,
);

impl CacheOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(op) => &op.key,
            Self::Delete(op) => &op.key,
            Self::CreateCache(op) => &op.name,
        }
    }
}
