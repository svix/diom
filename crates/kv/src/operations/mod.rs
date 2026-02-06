use super::KvStore;
use serde::{Deserialize, Serialize};

mod delete;
mod set;

pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::raft_module_operations;

raft_module_operations!(
    state = &mut KvStore,
    KvRequest,
    KvOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
    }
);

impl KvOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}
