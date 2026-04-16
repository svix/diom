use crate::State;

mod clear_expired;
mod configure_namespace;
mod delete;
mod set;

pub use clear_expired::ClearExpiredOperation;
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
        ClearExpired(ClearExpiredOperation) -> (),
        ConfigureKv(ConfigureKvOperation) -> ConfigureKvResponseData,
        Delete(DeleteOperation) -> DeleteResponseData,
        Set(SetOperation) -> SetResponseData,
    },
    state = KvRaftState<'_>,
);

impl KvOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::ClearExpired(_) => None,
            Self::ConfigureKv(op) => Some(&op.name),
            Self::Delete(op) => Some(&op.key),
            Self::Set(op) => Some(&op.key),
        }
    }
}
