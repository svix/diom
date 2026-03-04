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

impl MsgsOperation {
    pub fn key_name(&self) -> String {
        match self {
            MsgsOperation::CreateNamespace(op) => op.name.to_string(),
            MsgsOperation::Publish(op) => op.topic.to_string(),
            MsgsOperation::StreamCommit(op) => op.topic.to_string(),
            MsgsOperation::StreamReceive(op) => op.topic.to_string(),
            MsgsOperation::TopicConfigure(op) => op.topic.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use diom_namespace::entities::StorageType;
    use stream_internals::entities::Retention;
    use uuid::Uuid;

    use super::*;
    use crate::entities::{TopicIn, TopicName};

    #[test]
    fn test_msgs_operation_key_name() {
        let op = CreateNamespaceOperation::new(
            "my-namespace".to_string(),
            Retention::default(),
            StorageType::Persistent,
        );
        assert_eq!(
            MsgsOperation::CreateNamespace(op).key_name(),
            "my-namespace"
        );

        let op = PublishOperation::new(
            Uuid::nil(),
            TopicIn::TopicName(
                TopicName::new("default".to_string(), "my-topic".to_string()).unwrap(),
            ),
            vec![],
        )
        .unwrap();
        assert_eq!(MsgsOperation::Publish(op).key_name(), "my-topic");

        let op = PublishOperation::new(
            Uuid::nil(),
            TopicIn::TopicName(
                TopicName::new("my-ns".to_string(), "my-topic".to_string()).unwrap(),
            ),
            vec![],
        )
        .unwrap();
        assert_eq!(MsgsOperation::Publish(op).key_name(), "my-ns:my-topic");
    }
}
