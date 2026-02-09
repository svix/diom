use super::KvStore;
use serde::{Deserialize, Serialize};

mod delete;
mod set;

pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::{raft_module_operations, raft_module_request_trait};

raft_module_request_trait!(KvRequest, KvOperation, &mut KvStore);

raft_module_operations!(
    KvOperation => [
        (Set, SetOperation, (), SetResponse),
        (Delete, DeleteOperation, (), DeleteResponse)
    ]
);

impl KvOperation {
    pub fn apply(self, state: &mut KvStore) -> Response {
        match self {
            Self::Set(req) => Response::Set(req.apply(state)),
            Self::Delete(req) => Response::Delete(req.apply(state)),
        }
    }

    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}
