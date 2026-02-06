use super::RateLimiter;
use serde::{Deserialize, Serialize};

mod limit;
mod reset;

pub use limit::LimitOperation;
pub use reset::ResetOperation;

trait RateLimiterRequest: Into<RateLimiterOperation> + coyote_operations::OperationRequest
where
    Self: 'static,
{
    fn apply(self, state: &RateLimiter) -> Self::Response;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RateLimiterOperation {
    Limit(limit::LimitOperation),
    Reset(reset::ResetOperation),
}

impl coyote_operations::ModuleRequest for RateLimiterOperation {
    type Response = Response;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Limit(limit::LimitResponse),
    Reset(reset::ResetResponse),
}

impl coyote_operations::ModuleResponse for Response {}

impl RateLimiterOperation {
    pub fn apply(self, state: &RateLimiter) -> Response {
        match self {
            Self::Limit(req) => Response::Limit(req.apply(state)),
            Self::Reset(req) => Response::Reset(req.apply(state)),
        }
    }
}

impl From<limit::LimitOperation> for RateLimiterOperation {
    fn from(value: limit::LimitOperation) -> Self {
        Self::Limit(value)
    }
}

impl From<reset::ResetOperation> for RateLimiterOperation {
    fn from(value: reset::ResetOperation) -> Self {
        Self::Reset(value)
    }
}

impl TryFrom<Response> for limit::LimitResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Limit(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Response> for reset::ResetResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        match value {
            Response::Reset(value) => Ok(value),
            _ => Err(()),
        }
    }
}
