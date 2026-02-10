use super::RateLimiter;
use serde::{Deserialize, Serialize};

mod limit;
mod reset;

pub use limit::{LimitOperation, LimitResponseData};
pub use reset::ResetOperation;

use coyote_operations::raft_module_operations;

raft_module_operations!(
    RateLimiterRequest,
    RateLimiterOperation {
        Limit(LimitOperation) -> LimitResponseData,
        Reset(ResetOperation) -> (),
    },
    state = &RateLimiter,
);
