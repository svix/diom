mod ack;
mod append_to_stream;
mod dlq;
mod fetch;
mod fetch_locking;
mod redrive;

pub use self::{ack::*, append_to_stream::*, fetch::*, fetch_locking::*};
pub use dlq::*;
pub use redrive::*;

use crate::State;
use serde::{Deserialize, Serialize};

use diom_operations::raft_module_operations;

raft_module_operations!(
    StreamRequest,
    StreamOperation {
        Append(AppendOperation) -> AppendResponseData,
    },
    state = &State,
);
