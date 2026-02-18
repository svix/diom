mod ack;
mod append_to_stream;
mod create_stream;
mod dlq;
mod fetch;
mod fetch_locking;
mod redrive;

pub use self::{ack::*, append_to_stream::*, create_stream::*, fetch::*, fetch_locking::*};
pub use dlq::*;
pub use redrive::*;

use crate::State;
use serde::{Deserialize, Serialize};

use coyote_operations::raft_module_operations;

pub struct StreamRaftState<'a> {
    pub stream: &'a State,
    pub configgroup: &'a coyote_configgroup::State,
}

raft_module_operations!(
    StreamRequest,
    StreamOperation {
        Append(AppendOperation) -> AppendResponseData,
        CreateStream(CreateStreamOperation) -> CreateStreamResponseData,
        Ack(AckOperation) -> AckResponseData,
        Fetch(FetchOperation) -> FetchResponseData,
        FetchLocking(FetchLockingOperation) -> FetchLockingResponseData,
        Dlq(DlqOperation) -> DlqResponseData,
        Redrive(RedriveOperation) -> RedriveResponseData,
    },
    state = StreamRaftState<'_>,
);
