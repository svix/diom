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

use diom_operations::raft_module_operations;

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
            Self::TryStart(op) => &op.key,
            Self::Complete(op) => &op.key,
            Self::Abort(op) => &op.key,
        }
    }
}

raft_module_operations!(
    CreateIdempotencyRequest,
    CreateIdempotencyOp {
        CreateIdempotency(CreateIdempotencyOperation) -> CreateIdempotencyResponseData,
    },
    state = &diom_namespace::State,
    response = CreateIdempotencyOperationResponse,
);

impl CreateIdempotencyOp {
    pub fn key_name(&self) -> &str {
        match self {
            CreateIdempotencyOp::CreateIdempotency(op) => &op.name,
        }
    }
}
