use openraft::RaftTypeConfig;

mod app;
mod errors;
mod logs;
mod network;
mod raft;
mod serialized_state_machine;
mod state_machine;

pub use self::{
    app::router,
    logs::DiomLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Store,
};

pub type NodeId = <raft::TypeConfig as RaftTypeConfig>::NodeId;
type Node = <raft::TypeConfig as RaftTypeConfig>::Node;
