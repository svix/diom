use crate::State;

mod configure_policy;
mod configure_role;
mod delete_policy;
mod delete_role;

pub use configure_policy::{ConfigureAccessPolicyOperation, ConfigureAccessPolicyResponseData};
pub use configure_role::{ConfigureRoleOperation, ConfigureRoleResponseData};
pub use delete_policy::{DeleteAccessPolicyOperation, DeleteAccessPolicyResponseData};
pub use delete_role::{DeleteRoleOperation, DeleteRoleResponseData};

use diom_operations::raft_module_operations;

pub struct AdminAuthRaftState<'a> {
    pub state: &'a State,
}

raft_module_operations!(
    AdminAuthRequest,
    AdminAuthOperation {
        ConfigureRole(ConfigureRoleOperation) -> ConfigureRoleResponseData,
        DeleteRole(DeleteRoleOperation) -> DeleteRoleResponseData,
        ConfigureAccessPolicy(ConfigureAccessPolicyOperation) -> ConfigureAccessPolicyResponseData,
        DeleteAccessPolicy(DeleteAccessPolicyOperation) -> DeleteAccessPolicyResponseData,
    },
    state = AdminAuthRaftState<'_>,
);
