use openraft::RaftTypeConfig;

pub(crate) mod app;
mod applier;
mod background;
mod discovery;
mod errors;
mod handle;
mod logs;
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
    logs::CoyoteLogs,
    raft::{Raft, TypeConfig, initialize_raft},
    state_machine::Stores,
};

pub type NodeId = <TypeConfig as RaftTypeConfig>::NodeId;
pub type Node = <TypeConfig as RaftTypeConfig>::Node;
pub type LogId = openraft::LogId<NodeId>;

pub use state_machine::ClusterId;
