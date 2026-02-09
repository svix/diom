use super::CacheStore;
use serde::{Deserialize, Serialize};

mod delete;
mod set;

pub use delete::DeleteOperation;
pub use set::SetOperation;

trait CacheRequest: Into<Operation> + coyote_operations::OperationRequest
where
    Self: 'static,
{
    fn apply(self, state: &mut CacheStore) -> Self::Response;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Operation {
    Set(set::SetOperation),
    Delete(delete::DeleteOperation),
}

impl coyote_operations::ModuleRequest for Operation {
    type Response = Response;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Response {
    Set(set::SetResponse),
    Delete(delete::DeleteResponse),
}

impl coyote_operations::ModuleResponse for Response {}

impl Operation {
    pub fn apply(self, state: &mut CacheStore) -> Response {
        match self {
            Self::Set(req) => Response::Set(req.apply(state)),
            Self::Delete(req) => Response::Delete(req.apply(state)),
        }
    }

    pub fn key_name(&self) -> &str {
        match self {
            Self::Set(req) => req.key.as_ref(),
            Self::Delete(req) => req.key.as_ref(),
        }
    }
}

impl From<set::SetOperation> for Operation {
    fn from(value: set::SetOperation) -> Self {
        Self::Set(value)
    }
}

impl From<delete::DeleteOperation> for Operation {
    fn from(value: delete::DeleteOperation) -> Self {
        Self::Delete(value)
    }
}

impl TryFrom<Response> for set::SetResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        if let Response::Set(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl TryFrom<Response> for delete::DeleteResponse {
    type Error = ();

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        if let Response::Delete(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}
