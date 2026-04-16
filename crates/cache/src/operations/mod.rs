use crate::State;

mod clear_expired;
mod configure_namespace;
mod delete;
mod set;

pub use clear_expired::ClearExpiredOperation;
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
        ClearExpired(ClearExpiredOperation) -> (),
        ConfigureCache(ConfigureCacheOperation) -> ConfigureCacheResponseData,
        Delete(DeleteOperation) -> DeleteResponseData,
        Set(SetOperation) -> (),
    },
    state = CacheRaftState<'_>,
);

impl CacheOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::ClearExpired(_) => None,
            Self::ConfigureCache(op) => Some(&op.name),
            Self::Delete(op) => Some(&op.key),
            Self::Set(op) => Some(&op.key),
        }
    }
}
