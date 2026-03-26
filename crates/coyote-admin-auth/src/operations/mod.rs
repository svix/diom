use crate::State;

mod delete_policy;
mod delete_role;
mod upsert_policy;
mod upsert_role;

pub use delete_policy::{DeleteAccessPolicyOperation, DeleteAccessPolicyResponseData};
pub use delete_role::{DeleteRoleOperation, DeleteRoleResponseData};
pub use upsert_policy::{UpsertAccessPolicyOperation, UpsertAccessPolicyResponseData};
pub use upsert_role::{UpsertRoleOperation, UpsertRoleResponseData};

use coyote_operations::raft_module_operations;

pub struct AdminAuthRaftState<'a> {
    pub state: &'a State,
}

raft_module_operations!(
    AdminAuthRequest,
    AdminAuthOperation {
        UpsertRole(UpsertRoleOperation) -> UpsertRoleResponseData,
        DeleteRole(DeleteRoleOperation) -> DeleteRoleResponseData,
        UpsertAccessPolicy(UpsertAccessPolicyOperation) -> UpsertAccessPolicyResponseData,
        DeleteAccessPolicy(DeleteAccessPolicyOperation) -> DeleteAccessPolicyResponseData,
    },
    state = AdminAuthRaftState<'_>,
);
