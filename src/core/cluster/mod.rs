use openraft::type_config::alias::{LogIdOf, NodeIdOf, NodeOf};

pub(crate) mod app;
mod applier;
mod background;
mod discovery;
mod handle;
mod logs;
pub(crate) mod middleware;
pub(crate) mod network;
mod node;
mod operations;
pub mod proto;
pub(crate) mod raft;
mod serialized_state_machine;
mod state_machine;

pub use self::{
    app::router,
    handle::{RaftState, RequestWithContext},
    logs::DiomLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Stores,
};

pub(crate) type LogId = LogIdOf<TypeConfig>;
pub type NodeId = NodeIdOf<TypeConfig>;
pub(crate) type Node = NodeOf<TypeConfig>;
pub(crate) type RaftError<E = openraft::errors::Infallible> =
    openraft::error::RaftError<TypeConfig, E>;
pub(crate) type ClientWriteError = openraft::error::ClientWriteError<TypeConfig>;
pub(crate) type SnapshotSignature = openraft::type_config::alias::SnapshotSignatureOf<TypeConfig>;

pub use state_machine::ClusterId;
