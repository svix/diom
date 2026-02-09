use super::IdempotencyStore;
use serde::{Deserialize, Serialize};

mod abandon;
mod complete;
mod start;

pub use abandon::AbandonOperation;
pub use complete::CompleteOperation;
pub use start::StartOperation;

trait IdempotencyRequest: Into<Operation> + coyote_operations::OperationRequest
where
    Self: 'static,
{
    fn apply(self, state: &mut IdempotencyStore) -> Self::Response;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Start(start::StartOperation),
    Complete(complete::CompleteOperation),
    Abandon(abandon::AbandonOperation),
}

impl coyote_operations::ModuleRequest for Operation {
    type Response = Response;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Start(start::StartResponse),
    Complete(complete::CompleteResponse),
    Abandon(abandon::AbandonResponse),
}

impl coyote_operations::ModuleResponse for Response {}

impl Operation {
    pub fn apply(self, state: &mut IdempotencyStore) -> Response {
        match self {
            Self::Start(req) => Response::Start(req.apply(state)),
            Self::Complete(req) => Response::Complete(req.apply(state)),
            Self::Abandon(req) => Response::Abandon(req.apply(state)),
        }
    }

    pub fn key_name(&self) -> &str {
        match self {
            Self::Start(req) => req.key.as_ref(),
            Self::Complete(req) => req.key.as_ref(),
            Self::Abandon(req) => req.key.as_ref(),
        }
    }
}

impl From<start::StartOperation> for Operation {
    fn from(value: start::StartOperation) -> Self {
        Self::Start(value)
    }
}

impl From<complete::CompleteOperation> for Operation {
    fn from(value: complete::CompleteOperation) -> Self {
        Self::Complete(value)
    }
}

impl From<abandon::AbandonOperation> for Operation {
    fn from(value: abandon::AbandonOperation) -> Self {
        Self::Abandon(value)
    }
}

impl TryFrom<Response> for start::StartResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        if let Response::Start(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Response> for complete::CompleteResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        if let Response::Complete(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Response> for abandon::AbandonResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        if let Response::Abandon(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
