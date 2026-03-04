use crate::CacheStore;
use serde::{Deserialize, Serialize};

mod create_cache;
mod delete;
mod set;

pub use create_cache::{CreateCacheOperation, CreateCacheResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use diom_operations::raft_module_operations;

raft_module_operations!(
    CacheRequest,
    CacheOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
    },
    state = &mut CacheStore,
);

impl CacheOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(op) => &op.key,
            Self::Delete(op) => &op.key,
        }
    }
}

raft_module_operations!(
    CreateCacheRequest,
    CreateCacheOp {
        CreateCache(CreateCacheOperation) -> CreateCacheResponseData,
    },
    state = &diom_namespace::State,
    response = CreateCacheOperationResponse,
);

impl CreateCacheOp {
    pub fn key_name(&self) -> &str {
        match self {
            CreateCacheOp::CreateCache(op) => &op.name,
        }
    }
}
