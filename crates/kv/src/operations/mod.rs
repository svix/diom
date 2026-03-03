use super::KvStore;
use serde::{Deserialize, Serialize};

mod create_kv;
mod delete;
mod set;

pub use create_kv::{CreateKvOperation, CreateKvResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use coyote_operations::raft_module_operations;

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
            Self::Set(op) => &op.key,
            Self::Delete(op) => &op.key,
        }
    }
}

raft_module_operations!(
    CreateKvRequest,
    CreateKvOp {
        CreateKv(CreateKvOperation) -> CreateKvResponseData,
    },
    state = &coyote_namespace::State,
    response = CreateKvOperationResponse,
);

impl CreateKvOp {
    pub fn key_name(&self) -> &str {
        match self {
            CreateKvOp::CreateKv(op) => &op.name,
        }
    }
}
