mod create_namespace;
mod publish;
mod queue;
mod stream;
mod topic_configure;

pub use self::{create_namespace::*, publish::*, queue::*, stream::*, topic_configure::*};

use crate::State;

use coyote_operations::raft_module_operations;

pub struct MsgsRaftState<'a> {
    pub msgs: &'a State,
    pub namespace: &'a coyote_namespace::State,
}

raft_module_operations!(
    MsgsRequest,
    MsgsOperation {
        CreateNamespace(CreateNamespaceOperation) -> CreateNamespaceResponseData,
        Publish(PublishOperation) -> PublishResponseData,
        QueueAck(QueueAckOperation) -> QueueAckResponseData,
        QueueConfigure(QueueConfigureOperation) -> QueueConfigureResponseData,
        QueueNack(QueueNackOperation) -> QueueNackResponseData,
        QueueReceive(QueueReceiveOperation) -> QueueReceiveResponseData,
        QueueRedriveDlq(QueueRedriveDlqOperation) -> QueueRedriveDlqResponseData,
        StreamCommit(StreamCommitOperation) -> StreamCommitResponseData,
        StreamReceive(StreamReceiveOperation) -> StreamReceiveResponseData,
        StreamSeek(StreamSeekOperation) -> StreamSeekResponseData,
        TopicConfigure(TopicConfigureOperation) -> TopicConfigureResponseData,
    },
    state = MsgsRaftState<'_>,
);

impl MsgsOperation {
    pub fn key_name(&self) -> String {
        match self {
            MsgsOperation::CreateNamespace(op) => op.name.to_string(),
            MsgsOperation::Publish(op) => op.topic.to_string(),
            MsgsOperation::QueueAck(op) => op.topic.to_string(),
            MsgsOperation::QueueConfigure(op) => op.topic.to_string(),
            MsgsOperation::QueueNack(op) => op.topic.to_string(),
            MsgsOperation::QueueReceive(op) => op.topic.to_string(),
            MsgsOperation::QueueRedriveDlq(op) => op.topic.to_string(),
            MsgsOperation::StreamCommit(op) => op.topic.to_string(),
            MsgsOperation::StreamReceive(op) => op.topic.to_string(),
            MsgsOperation::StreamSeek(op) => op.topic.to_string(),
            MsgsOperation::TopicConfigure(op) => op.topic.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use coyote_id::NamespaceId;

    use super::*;
    use crate::entities::{Retention, TopicIn, TopicName};

    #[test]
    fn test_msgs_operation_key_name() {
        let op = CreateNamespaceOperation::new("my-namespace".to_string(), Retention::default());
        assert_eq!(
            MsgsOperation::CreateNamespace(op).key_name(),
            "my-namespace"
        );

        let op = PublishOperation::new(
            NamespaceId::nil(),
            TopicIn::TopicName(TopicName::new("my-topic".to_string()).unwrap()),
            vec![],
            None,
        )
        .unwrap();
        assert_eq!(MsgsOperation::Publish(op).key_name(), "my-topic");
    }
}
