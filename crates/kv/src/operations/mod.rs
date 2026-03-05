use crate::State;

use serde::{Deserialize, Serialize};

mod create_namespace;
mod delete;
mod set;

pub use create_namespace::{CreateKvOperation, CreateKvResponseData};
pub use delete::DeleteOperation;
pub use set::SetOperation;

use diom_operations::raft_module_operations;

pub struct KvRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    KvRequest,
    KvOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> (),
        CreateKv(CreateKvOperation) -> CreateKvResponseData,
    },
    state = KvRaftState<'_>,
);

impl KvOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(op) => &op.key,
            Self::Delete(op) => &op.key,
            Self::CreateKv(op) => &op.name,
        }
    }
}
