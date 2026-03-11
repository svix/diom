use crate::State;

use serde::{Deserialize, Serialize};

mod create_namespace;
mod limit;
mod reset;

pub use create_namespace::{CreateRateLimitOperation, CreateRateLimitResponseData};
pub use limit::{LimitOperation, LimitResponseData};
pub use reset::ResetOperation;

use coyote_operations::raft_module_operations;

pub struct RateLimiterRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    RateLimiterRequest,
    RateLimiterOperation {
        Limit(LimitOperation) -> LimitResponseData,
        Reset(ResetOperation) -> (),
        CreateRateLimit(CreateRateLimitOperation) -> CreateRateLimitResponseData,
    },
    state = RateLimiterRaftState<'_>,
);

impl RateLimiterOperation {
    pub fn key_name(&self) -> &str {
        match self {
            RateLimiterOperation::Limit(limit_operation) => &limit_operation.key,
            RateLimiterOperation::Reset(reset_operation) => &reset_operation.key,
            RateLimiterOperation::CreateRateLimit(op) => &op.name,
        }
    }
}
