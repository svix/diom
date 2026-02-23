use crate::CacheStore;
use serde::{Deserialize, Serialize};

mod create_cache;
mod delete;
mod set;

pub use create_cache::{CreateCacheOperation, CreateCacheResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::raft_module_operations;

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
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}

raft_module_operations!(
    CreateCacheRequest,
    CreateCacheOp {
        CreateCache(CreateCacheOperation) -> CreateCacheResponseData,
    },
    state = &coyote_configgroup::State,
    response = CreateCacheOperationResponse,
);
