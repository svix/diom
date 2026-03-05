use crate::State;

use serde::{Deserialize, Serialize};

mod abort;
mod complete;
mod create_idempotency;
mod try_start;

pub use abort::AbortOperation;
pub use complete::CompleteOperation;
pub use try_start::{TryStartOperation, TryStartResponseData};

pub use create_idempotency::{CreateIdempotencyOperation, CreateIdempotencyResponseData};

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
    },
    state = IdempotencyRaftState<'_>,
);

impl IdempotencyOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::TryStart(op) => &op.key,
            Self::Complete(op) => &op.key,
            Self::Abort(op) => &op.key,
            Self::CreateIdempotency(op) => &op.name,
        }
    }
}
