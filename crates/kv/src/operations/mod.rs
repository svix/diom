use crate::State;

mod configure_namespace;
mod delete;
mod set;

pub use configure_namespace::{ConfigureKvOperation, ConfigureKvResponseData};
pub use delete::{DeleteOperation, DeleteResponseData};
pub use set::{SetOperation, SetResponseData};

use diom_operations::raft_module_operations;

pub struct KvRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    KvRequest,
    KvOperation {
        Set(SetOperation) -> SetResponseData,
        Delete(DeleteOperation) -> DeleteResponseData,
        ConfigureKv(ConfigureKvOperation) -> ConfigureKvResponseData,
    },
    state = KvRaftState<'_>,
);

impl KvOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::ConfigureKv(op) => Some(&op.name),
        }
    }
}
