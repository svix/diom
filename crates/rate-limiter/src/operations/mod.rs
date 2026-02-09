use super::RateLimiter;
use serde::{Deserialize, Serialize};

mod limit;
mod reset;

pub use limit::{LimitOperation, LimitResponseData};
pub use reset::ResetOperation;

use coyote_operations::{raft_module_operations, raft_module_request_trait};

raft_module_request_trait!(RateLimiterRequest, RateLimiterOperation, &RateLimiter);

raft_module_operations!(
    RateLimiterOperation => [
        (Limit, LimitOperation, LimitResponseData, LimitResponse),
        (Reset, ResetOperation, (), ResetResponse)
    ]
);

impl RateLimiterOperation {
    pub fn apply(self, state: &RateLimiter) -> Response {
        match self {
            Self::Limit(req) => Response::Limit(req.apply(state)),
            Self::Reset(req) => Response::Reset(req.apply(state)),
        }
    }
}
