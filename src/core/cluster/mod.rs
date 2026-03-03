use openraft::RaftTypeConfig;

mod app;
mod applier;
mod background;
mod discovery;
mod errors;
mod handle;
mod logs;
mod network;
mod node;
mod operations;
pub mod proto;
mod raft;
mod serialized_state_machine;
mod state_machine;

pub use self::{
    app::router,
    handle::{RaftState, RequestWithContext},
    logs::CoyoteLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Stores,
};

pub type NodeId = <TypeConfig as RaftTypeConfig>::NodeId;
type Node = <TypeConfig as RaftTypeConfig>::Node;
