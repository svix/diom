use crate::IdempotencyStore;
use serde::{Deserialize, Serialize};

mod abandon;
mod complete;
mod try_start;

pub use abandon::AbandonOperation;
pub use complete::CompleteOperation;
pub use try_start::{TryStartOperation, TryStartResponseData};

use coyote_operations::raft_module_operations;

raft_module_operations!(
    IdempotencyRequest,
    IdempotencyOperation {
        TryStart(TryStartOperation) -> TryStartResponseData,
        Complete(CompleteOperation) -> (),
        Abandon(AbandonOperation) -> (),
    },
    state = &mut IdempotencyStore,
);

impl IdempotencyOperation {
    pub fn key_name(&self) -> &str {
        match self {
            Self::TryStart(req) => req.key.as_ref(),
            Self::Complete(req) => req.key.as_ref(),
            Self::Abandon(req) => req.key.as_ref(),
        }
    }
}
