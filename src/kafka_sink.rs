use std::time::Duration;

use anyhow::anyhow;
use diom_core::types::DurationMs;
use diom_msgs::{
    MsgsNamespace,
    entities::{ConsumerGroup, SeekPosition, TopicIn, TopicName, TopicPartition},
    operations::{StreamCommitOperation, StreamReceiveOperation},
};
use diom_namespace::State as NamespaceState;
use diom_operations::{BackgroundResult, workers::BackgroundWorker};
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use tracing::instrument;

use crate::{cfg::KafkaSinkConfig, core::cluster::RaftState};

#[derive(Clone)]
pub(crate) struct KafkaSinkWorker {
    raft: RaftState,
    namespace_state: NamespaceState,
    config: KafkaSinkConfig,
    producer: FutureProducer,
}

impl BackgroundWorker for KafkaSinkWorker {
    const NAME: &str = "bg-worker:kafka-sink";

    async fn run(self) -> BackgroundResult<()> {
        let shutting_down = diom_core::shutdown::shutting_down_token();
        let mut interval = tokio::time::interval(Duration::from_millis(500));

        while shutting_down
            .run_until_cancelled(interval.tick())
            .await
            .is_some()
        {
            match self.raft.state().await {
                Ok(openraft::ServerState::Leader) => {}
                _ => continue,
            }

            if let Err(err) = self.poll_and_forward().await {
                tracing::warn!(?err, "kafka sink poll error");
            }
        }

        Ok(())
    }
}

impl KafkaSinkWorker {
    pub(crate) fn new(
        raft: RaftState,
        namespace_state: NamespaceState,
        config: KafkaSinkConfig,
    ) -> anyhow::Result<Self> {
        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &config.brokers);
        if let Some(protocol) = &config.security_protocol {
            client_config.set("security.protocol", protocol);
        }
        if let Some(mechanism) = &config.sasl_mechanism {
            client_config.set("sasl.mechanism", mechanism);
        }
        if let Some(username) = &config.sasl_username {
            client_config.set("sasl.username", username);
        }
        if let Some(password) = &config.sasl_password {
            client_config.set("sasl.password", password);
        }
        let producer = client_config.create::<FutureProducer>()?;
        Ok(Self {
            raft,
            namespace_state,
            config,
            producer,
        })
    }

    #[instrument(skip_all)]
    async fn poll_and_forward(&self) -> anyhow::Result<()> {
        let namespace: MsgsNamespace = self
            .namespace_state
            .fetch_namespace(self.config.source_namespace.as_deref())?
            .ok_or_else(|| anyhow!("msgs namespace not found"))?;

        let topic = TopicName::new(self.config.source_topic.clone())
            .map(TopicIn::TopicName)
            .map_err(|e| anyhow!("invalid source_topic: {e}"))?;

        let consumer_group = ConsumerGroup::try_from(self.config.consumer_group.clone())
            .map_err(|e| anyhow!("invalid consumer_group: {e}"))?;

        let operation = StreamReceiveOperation::new(
            namespace.id,
            topic,
            consumer_group.clone(),
            self.config.batch_size,
            DurationMs::from_secs(30),
            SeekPosition::Earliest,
        )?;

        let response = self
            .raft
            .client_write(operation)
            .await
            .map_err(|e| anyhow!("stream receive raft error: {e}"))?
            .0
            .map_err(|e| anyhow!("stream receive error: {e:?}"))?;

        if response.msgs.is_empty() {
            return Ok(());
        }

        for msg in &response.msgs {
            let record = FutureRecord::<(), _>::to(&self.config.kafka_topic).payload(&msg.value[..]);
            self.producer
                .send(record, Duration::from_secs(5))
                .await
                .map_err(|(err, _)| anyhow!("kafka produce failed: {err}"))?;
        }

        // Commit the max offset seen per partition so the stream cursor advances
        let mut to_commit: Vec<(TopicPartition, u64)> = Vec::new();
        for msg in &response.msgs {
            if let Some(entry) = to_commit.iter_mut().find(|(tp, _)| tp == &msg.topic) {
                entry.1 = entry.1.max(msg.offset);
            } else {
                to_commit.push((msg.topic.clone(), msg.offset));
            }
        }

        for (topic_partition, offset) in to_commit {
            let commit_op = StreamCommitOperation::new(
                namespace.id,
                topic_partition,
                consumer_group.clone(),
                offset,
            );
            self.raft
                .client_write(commit_op)
                .await
                .map_err(|e| anyhow!("stream commit raft error: {e}"))?
                .0
                .map_err(|e| anyhow!("stream commit error: {e:?}"))?;
        }

        Ok(())
    }
}
