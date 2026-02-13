use crate::CacheStore;
use serde::{Deserialize, Serialize};

mod delete;
mod set;

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
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}
