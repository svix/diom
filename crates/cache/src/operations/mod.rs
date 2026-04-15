use crate::State;

mod configure_namespace;
mod delete;
mod set;

pub use configure_namespace::{ConfigureCacheOperation, ConfigureCacheResponseData};
pub use delete::{DeleteOperation, DeleteResponseData};
pub use set::SetOperation;

use diom_operations::raft_module_operations;

pub struct CacheRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    CacheRequest,
    CacheOperation {
        Set(SetOperation) -> (),
        Delete(DeleteOperation) -> DeleteResponseData,
        ConfigureCache(ConfigureCacheOperation) -> ConfigureCacheResponseData,
    },
    state = CacheRaftState<'_>,
);

impl CacheOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::Set(op) => Some(&op.key),
            Self::Delete(op) => Some(&op.key),
            Self::ConfigureCache(op) => Some(&op.name),
        }
    }
}
