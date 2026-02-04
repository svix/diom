use openraft::RaftTypeConfig;

mod app;
mod discovery;
mod errors;
mod logs;
mod network;
pub mod proto;
mod raft;
mod serialized_state_machine;
mod state_machine;

pub use self::{
    app::router,
    logs::CoyoteLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Store,
};

pub type NodeId = <raft::TypeConfig as RaftTypeConfig>::NodeId;
type Node = <raft::TypeConfig as RaftTypeConfig>::Node;
