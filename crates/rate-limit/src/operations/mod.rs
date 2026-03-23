use crate::State;

mod create_namespace;
mod limit;
mod reset;

pub use create_namespace::{CreateRateLimitOperation, CreateRateLimitResponseData};
pub use limit::{LimitOperation, LimitResponseData};
pub use reset::ResetOperation;

use diom_operations::raft_module_operations;

pub struct RateLimitRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    RateLimitRequest,
    RateLimitOperation {
        Limit(LimitOperation) -> LimitResponseData,
        Reset(ResetOperation) -> (),
        CreateRateLimit(CreateRateLimitOperation) -> CreateRateLimitResponseData,
    },
    state = RateLimitRaftState<'_>,
);

impl RateLimitOperation {
    pub fn key_name(&self) -> &str {
        match self {
            RateLimitOperation::Limit(limit_operation) => &limit_operation.key,
            RateLimitOperation::Reset(reset_operation) => &reset_operation.key,
            RateLimitOperation::CreateRateLimit(op) => &op.name,
        }
    }
}
