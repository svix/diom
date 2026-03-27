use openraft::type_config::alias::{LogIdOf, NodeIdOf, NodeOf};

pub(crate) mod app;
mod applier;
mod background;
mod discovery;
mod handle;
mod logs;
pub(crate) mod network;
mod node;
mod operations;
pub mod proto;
pub(crate) mod raft;
mod serialized_state_machine;
mod state_machine;
mod streaming_snapshot;

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
pub(crate) type RaftError<C> = openraft::error::RaftError<TypeConfig, C>;
pub(crate) type ClientWriteError = openraft::error::ClientWriteError<TypeConfig>;

pub use state_machine::ClusterId;
