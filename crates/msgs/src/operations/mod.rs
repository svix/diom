mod configure_namespace;
mod publish;
mod queue;
mod stream;
mod topic_configure;

pub use self::{configure_namespace::*, publish::*, queue::*, stream::*, topic_configure::*};

use crate::State;

use diom_operations::raft_module_operations;

pub struct MsgsRaftState<'a> {
    pub msgs: &'a State,
    pub namespace: &'a diom_namespace::State,
}

raft_module_operations!(
    MsgsRequest,
    MsgsOperation {
        ConfigureNamespace(ConfigureNamespaceOperation) -> ConfigureNamespaceResponseData,
        Publish(PublishOperation) -> PublishResponseData,
        QueueAck(QueueAckOperation) -> QueueAckResponseData,
        QueueConfigure(QueueConfigureOperation) -> QueueConfigureResponseData,
        QueueExtendLease(QueueExtendLeaseOperation) -> QueueExtendLeaseResponseData,
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
            MsgsOperation::ConfigureNamespace(op) => op.name.to_string(),
            MsgsOperation::Publish(op) => op.topic.to_string(),
            MsgsOperation::QueueAck(op) => op.topic.to_string(),
            MsgsOperation::QueueConfigure(op) => op.topic.to_string(),
            MsgsOperation::QueueExtendLease(op) => op.topic.to_string(),
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
    use diom_id::NamespaceId;
    use diom_namespace::entities::NamespaceName;

    use super::*;
    use crate::entities::{Retention, TopicIn, TopicName};

    #[test]
    fn test_msgs_operation_key_name() {
        let op = ConfigureNamespaceOperation::new(
            NamespaceName("my-namespace".to_owned()),
            Retention::default(),
        );
        assert_eq!(
            MsgsOperation::ConfigureNamespace(op).key_name(),
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
