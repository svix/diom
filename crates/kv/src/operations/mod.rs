use super::KvStore;
use serde::{Deserialize, Serialize};

mod create_kv;
mod delete;
mod set;

pub use create_kv::{CreateKvOperation, CreateKvResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use diom_operations::raft_module_operations;

raft_module_operations!(
    KvRequest,
    KvOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
    },
    state = &mut KvStore,
);

impl KvOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}

raft_module_operations!(
    CreateKvRequest,
    CreateKvOp {
        CreateKv(CreateKvOperation) -> CreateKvResponseData,
    },
    state = &diom_configgroup::State,
    response = CreateKvOperationResponse,
);
