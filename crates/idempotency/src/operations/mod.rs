use crate::State;

mod abort;
mod clear_expired;
mod complete;
mod create_namespace;
mod try_start;

pub use abort::AbortOperation;
pub use clear_expired::ClearExpiredOperation;
pub use complete::CompleteOperation;
pub use try_start::{TryStartOperation, TryStartResponseData};

pub use create_namespace::{CreateIdempotencyOperation, CreateIdempotencyResponseData};

use coyote_operations::raft_module_operations;

pub struct IdempotencyRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    IdempotencyRequest,
    IdempotencyOperation {
        TryStart(TryStartOperation) -> TryStartResponseData,
        Complete(CompleteOperation) -> (),
        Abort(AbortOperation) -> (),
        CreateIdempotency(CreateIdempotencyOperation) -> CreateIdempotencyResponseData,
        ClearExpired(ClearExpiredOperation) -> (),
    },
    state = IdempotencyRaftState<'_>,
);

impl IdempotencyOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::TryStart(op) => Some(&op.key),
            Self::Complete(op) => Some(&op.key),
            Self::Abort(op) => Some(&op.key),
            Self::CreateIdempotency(op) => Some(&op.name),
            Self::ClearExpired(_) => None,
        }
    }
}
