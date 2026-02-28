mod create_namespace;
mod publish;
mod stream_commit;
mod stream_receive;
mod topic_configure;

pub use self::{
    create_namespace::*, publish::*, stream_commit::*, stream_receive::*, topic_configure::*,
};

use crate::State;
use serde::{Deserialize, Serialize};

use diom_operations::raft_module_operations;

pub struct MsgsRaftState<'a> {
    pub msgs: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    MsgsRequest,
    MsgsOperation {
        CreateNamespace(CreateNamespaceOperation) -> CreateNamespaceResponseData,
        Publish(PublishOperation) -> PublishResponseData,
        StreamCommit(StreamCommitOperation) -> StreamCommitResponseData,
        StreamReceive(StreamReceiveOperation) -> StreamReceiveResponseData,
        TopicConfigure(TopicConfigureOperation) -> TopicConfigureResponseData,
    },
    state = MsgsRaftState<'_>,
);
