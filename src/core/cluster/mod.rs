use openraft::RaftTypeConfig;

mod app;
mod applier;
mod discovery;
mod errors;
mod handle;
mod logs;
mod network;
mod operations;
pub mod proto;
mod raft;
mod serialized_state_machine;
mod state_machine;

pub use self::{
    app::router,
    handle::RaftState,
    logs::CoyoteLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Store,
};

pub type NodeId = <raft::TypeConfig as RaftTypeConfig>::NodeId;
type Node = <raft::TypeConfig as RaftTypeConfig>::Node;
