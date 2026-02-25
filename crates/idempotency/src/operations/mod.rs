use crate::IdempotencyStore;
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

raft_module_operations!(
    IdempotencyRequest,
    IdempotencyOperation {
        TryStart(TryStartOperation) -> TryStartResponseData,
        Complete(CompleteOperation) -> (),
        Abort(AbortOperation) -> (),
    },
    state = &mut IdempotencyStore,
);

impl IdempotencyOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::TryStart(req) => req.key.as_ref(),
            Self::Complete(req) => req.key.as_ref(),
            Self::Abort(req) => req.key.as_ref(),
        }
    }
}

raft_module_operations!(
    CreateIdempotencyRequest,
    CreateIdempotencyOp {
        CreateIdempotency(CreateIdempotencyOperation) -> CreateIdempotencyResponseData,
    },
    state = &coyote_namespace::State,
    response = CreateIdempotencyOperationResponse,
);
