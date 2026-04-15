use crate::State;

mod abort;
mod complete;
mod configure_namespace;
mod try_start;

pub use abort::AbortOperation;
pub use complete::CompleteOperation;
pub use try_start::{TryStartOperation, TryStartResponseData};

pub use configure_namespace::{ConfigureIdempotencyOperation, ConfigureIdempotencyResponseData};

use diom_operations::raft_module_operations;

pub struct IdempotencyRaftState<'a> {
    pub state: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    IdempotencyRequest,
    IdempotencyOperation {
        TryStart(TryStartOperation) -> TryStartResponseData,
        Complete(CompleteOperation) -> (),
        Abort(AbortOperation) -> (),
        ConfigureIdempotency(ConfigureIdempotencyOperation) -> ConfigureIdempotencyResponseData,
    },
    state = IdempotencyRaftState<'_>,
);

impl IdempotencyOperation {
    pub fn key_name(&self) -> Option<&str> {
        match self {
            Self::TryStart(op) => Some(&op.key),
            Self::Complete(op) => Some(&op.key),
            Self::Abort(op) => Some(&op.key),
            Self::ConfigureIdempotency(op) => Some(&op.name),
        }
    }
}
