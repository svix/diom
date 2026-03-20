use crate::State;

mod create;
mod create_namespace;
mod delete;
mod expire;
mod rotate;
mod update;

pub use create::{CreateAuthTokenOperation, CreateResponseData};
pub use create_namespace::{
    CreateAuthTokenNamespaceOperation, CreateAuthTokenNamespaceResponseData,
};
pub use delete::{DeleteAuthTokenOperation, DeleteResponseData};
pub use expire::{ExpireAuthTokenOperation, ExpireResponseData};
pub use rotate::{RotateAuthTokenOperation, RotateResponseData};
pub use update::{UpdateAuthTokenOperation, UpdateResponseData};

use coyote_operations::async_raft_module_operations;

pub struct AuthTokenRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

async_raft_module_operations!(
    AuthTokenRequest,
    AuthTokenOperation {
        Create(CreateAuthTokenOperation) -> CreateResponseData,
        Expire(ExpireAuthTokenOperation) -> ExpireResponseData,
        Delete(DeleteAuthTokenOperation) -> DeleteResponseData,
        Update(UpdateAuthTokenOperation) -> UpdateResponseData,
        Rotate(RotateAuthTokenOperation) -> RotateResponseData,
        CreateNamespace(CreateAuthTokenNamespaceOperation) -> CreateAuthTokenNamespaceResponseData,
    },
    state = AuthTokenRaftState<'_>,
);
