use crate::State;

mod configure_namespace;
mod create;
mod delete;
mod expire;
mod rotate;
mod update;

pub use configure_namespace::{
    ConfigureAuthTokenNamespaceOperation, ConfigureAuthTokenNamespaceResponseData,
};
pub use create::{CreateAuthTokenOperation, CreateResponseData};
pub use delete::{DeleteAuthTokenOperation, DeleteResponseData};
pub use expire::{ExpireAuthTokenOperation, ExpireResponseData};
pub use rotate::{RotateAuthTokenOperation, RotateResponseData};
pub use update::{UpdateAuthTokenOperation, UpdateResponseData};

use diom_operations::raft_module_operations;

pub struct AuthTokenRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    AuthTokenRequest,
    AuthTokenOperation {
        Create(CreateAuthTokenOperation) -> CreateResponseData,
        Expire(ExpireAuthTokenOperation) -> ExpireResponseData,
        Delete(DeleteAuthTokenOperation) -> DeleteResponseData,
        Update(UpdateAuthTokenOperation) -> UpdateResponseData,
        Rotate(RotateAuthTokenOperation) -> RotateResponseData,
        ConfigureNamespace(ConfigureAuthTokenNamespaceOperation) -> ConfigureAuthTokenNamespaceResponseData,
    },
    state = AuthTokenRaftState<'_>,
);
