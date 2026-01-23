mod app;
mod errors;
mod logs;
mod network;
mod raft;
mod serialized_state_machine;
mod state_machine;

pub use app::router;
pub use logs::CoyoteLogs;
pub use raft::{Raft, TypeConfig, initialize_raft};
pub use state_machine::Store;

use openraft::RaftTypeConfig;

pub type NodeId = <raft::TypeConfig as RaftTypeConfig>::NodeId;
type Node = <raft::TypeConfig as RaftTypeConfig>::Node;
