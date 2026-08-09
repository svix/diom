mod http;
#[cfg(feature = "kafka")]
mod kafka;
mod svix;

use std::{
    collections::HashMap,
    num::{NonZeroU16, NonZeroU32},
    time::{Duration, Instant},
};

use diom_core::{backoff::jitter, tokio_nursery::TaskNursery, types::DurationMs};
use diom_id::NamespaceId;
use diom_operations::{BackgroundResult, OperationWriter};
use fjall_utils::{FjallKey, TableRow};
use futures_util::StreamExt;
use tracing::instrument;

use http::CompiledHttpSink;
#[cfg(feature = "kafka")]
use kafka::CompiledKafkaSink;
use svix::CompiledSvixSink;

use crate::{
    State,
    entities::{
        ConsumerGroup, Offset, Partition, SeekPosition, SinkConfig, SinkSettings, TopicIn,
        TopicName, TopicPartition,
    },
    operations::{MsgsOperation, StreamCommitOperation, StreamReceiveMsg, StreamReceiveOperation},
    storage::{SinkKey, SinkRow},
};

/// Number of messages leased from a topic per receive.
const DEFAULT_SINK_BATCH_SIZE: NonZeroU16 = NonZeroU16::new(100).unwrap();

/// Per-request timeout for outbound deliveries.
const SINK_HTTP_TIMEOUT: Duration = Duration::from_secs(30);

/// How many times a failed delivery is retried within a single lease before the message is left
/// uncommitted for a later poll cycle to retry (effectively retrying forever across cycles).
const SINK_DELIVERY_RETRIES: u32 = 5;

/// Bounds on the short random pause between delivery retries.
const SINK_RETRY_MIN_INTERVAL: Duration = Duration::from_millis(50);
const SINK_RETRY_MAX_INTERVAL: Duration = Duration::from_millis(250);

/// How long a receive leases a partition. Needs to be long enough to cover a full retry loop.
const SINK_LEASE_DURATION: Duration = Duration::from_secs(200);

fn sink_lease_duration() -> DurationMs {
    DurationMs::try_from(SINK_LEASE_DURATION).expect("sink lease duration fits in DurationMs")
}

#[derive(Clone, Copy)]
pub struct SinkWorkerConfig {
    /// A "global" limit on concurrency. Not related to per-sink concurrency, other than
    /// the fact that it's a strict upper bound.
    pub max_concurrent: std::num::NonZeroUsize,
    /// Cap how long tasks take so we don't accidentally starve other sink dispatch loops.
    pub max_task_duration: Duration,
}

#[derive(Clone)]
pub struct LeaderWorker<F: OperationWriter<MsgsOperation>> {
    state: State,
    poll_interval: Duration,
    handle: F,
    config: SinkWorkerConfig,
    http: reqwest::Client,
}

impl<F> LeaderWorker<F>
where
    F: OperationWriter<MsgsOperation> + Send + Sync + 'static,
{
    pub fn new(state: State, poll_interval: Duration, handle: F, config: SinkWorkerConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(SINK_HTTP_TIMEOUT)
            .build()
            .expect("failed to build sink HTTP client");
        Self {
            state,
            poll_interval,
            handle,
            config,
            http,
        }
    }

    /// Iterates over every sink config and drains each one concurrently, bounded by
    /// [`SinkWorkerConfig::max_concurrent`].
    async fn poll_cycle(&self) -> BackgroundResult<()> {
        let batch_size = self.config.max_concurrent.get();
        let mut nursery = TaskNursery::new(self.config.max_concurrent);

        let mut iterator: Option<Vec<u8>> = None;
        let prefix = &[<SinkRow as TableRow>::ROW_TYPE];

        loop {
            let batch =
                SinkRow::list_range(&self.state.metadata_tables, prefix, iterator, batch_size)
                    .map_err(diom_operations::BackgroundError::Other)?;

            if batch.is_empty() {
                break;
            }

            iterator = batch.last().map(|(k, _)| k.to_vec());
            let batch_len = batch.len();

            for (key_bytes, row) in batch {
                let Ok(key) = SinkKey::from_fjall_key(key_bytes) else {
                    continue;
                };

                let handle = self.handle.clone();
                let http = self.http.clone();
                let deadline = self.config.max_task_duration;
                nursery
                    .spawn(drain_sink(
                        handle,
                        http,
                        key,
                        row.topic,
                        row.settings,
                        deadline,
                    ))
                    .await;
            }

            if batch_len < batch_size {
                break;
            }
        }

        nursery.join_all().await;

        Ok(())
    }
}

/// Repeatedly receives a batch from the topic, delivers each message, and commits the cursor up to
/// the last contiguously-delivered offset per partition, until the topic is drained or
/// `max_duration` elapses. Partitions are drained concurrently (bounded by
/// [`SinkSettings::max_in_flight`]). Within a partition, messages are delivered strictly in order.
/// A failed delivery is retried up to [`SINK_DELIVERY_RETRIES`] times within the lease. If it still
/// fails, the message is left uncommitted so a later poll cycle re-leases and retries it.
#[instrument(skip_all, fields(namespace = ?key.namespace_id, topic = ?key.topic_id, consumer_group = %key.consumer_group))]
async fn drain_sink<F>(
    handle: F,
    http: reqwest::Client,
    key: SinkKey,
    topic_name: TopicName,
    settings: SinkSettings,
    max_duration: Duration,
) where
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    let consumer_group = key.consumer_group.clone();
    let compiled = match &settings.config {
        SinkConfig::Http(http_config) => CompiledSink::Http(CompiledHttpSink::new(http_config)),
        SinkConfig::Svix(svix_config) => CompiledSink::Svix(CompiledSvixSink::new(svix_config)),
        #[cfg(feature = "kafka")]
        SinkConfig::Kafka(kafka_config) => match CompiledKafkaSink::new(kafka_config) {
            Ok(sink) => CompiledSink::Kafka(sink),
            Err(e) => {
                tracing::error!(error = %e, "failed to build kafka producer, skipping sink");
                return;
            }
        },
        #[cfg(not(feature = "kafka"))]
        SinkConfig::Kafka(_) => {
            tracing::error!("kafka sink support not compiled in, skipping sink");
            return;
        }
    };
    let start = settings.default_starting_position.clone();
    let deadline = Instant::now() + max_duration;

    while Instant::now() < deadline {
        let MsgBatch { msgs, leased_at } = match read_next_msg_batch(
            &handle,
            key.namespace_id,
            &topic_name,
            &consumer_group,
            &start,
        )
        .await
        {
            Ok(batch) => batch,
            Err(e) => {
                tracing::error!(error = %e, "failed to read next message batch");
                break;
            }
        };

        if msgs.is_empty() {
            break;
        }
        let batch_len = msgs.len();
        let lease_deadline = leased_at + SINK_LEASE_DURATION;

        let stopped = deliver_and_commit_batch(
            &handle,
            &http,
            &compiled,
            key.namespace_id,
            &topic_name,
            &consumer_group,
            &msgs,
            settings.max_in_flight,
            lease_deadline,
            deadline,
        )
        .await;

        // Stop if a partition broke out of delivery early (budget/lease spent or a message failed
        // all its retries), or if the batch wasn't full (topic is drained for now).
        if stopped || batch_len < DEFAULT_SINK_BATCH_SIZE.get() as usize {
            break;
        }
    }
}

struct MsgBatch {
    msgs: Vec<StreamReceiveMsg>,
    leased_at: Instant,
}

async fn read_next_msg_batch<F>(
    handle: &F,
    namespace_id: NamespaceId,
    topic_name: &TopicName,
    consumer_group: &ConsumerGroup,
    start: &SeekPosition,
) -> Result<MsgBatch, String>
where
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    let recv_op = StreamReceiveOperation::new(
        namespace_id,
        TopicIn::TopicName(topic_name.clone()),
        consumer_group.clone(),
        DEFAULT_SINK_BATCH_SIZE,
        sink_lease_duration(),
        start.clone(),
        None,
    )
    .map_err(|e| format!("{e:?}"))?;

    let leased_at = Instant::now();
    let response = handle
        .write_request(recv_op)
        .await
        .map_err(|e| format!("{e:?}"))?;
    let msgs = response.0.map_err(|e| format!("{e:?}"))?.msgs;

    Ok(MsgBatch { msgs, leased_at })
}

#[allow(clippy::too_many_arguments)]
async fn deliver_and_commit_batch<F>(
    handle: &F,
    http: &reqwest::Client,
    compiled: &CompiledSink<'_>,
    namespace_id: NamespaceId,
    topic_name: &TopicName,
    consumer_group: &ConsumerGroup,
    msgs: &[StreamReceiveMsg],
    max_in_flight: Option<NonZeroU32>,
    lease_deadline: Instant,
    deadline: Instant,
) -> bool
where
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    // Group by partition, preserving offset order (the receive returns messages ordered).
    let mut by_partition: HashMap<Partition, Vec<&StreamReceiveMsg>> = HashMap::new();
    for msg in msgs {
        by_partition
            .entry(msg.topic.partition)
            .or_default()
            .push(msg);
    }

    let limit = max_in_flight
        .map(|n| n.get() as usize)
        .unwrap_or_else(|| by_partition.len().max(1));

    let partition_futures: Vec<_> = by_partition
        .into_iter()
        .map(|(partition, partition_msgs)| {
            deliver_and_commit_partition(
                handle,
                http,
                compiled,
                namespace_id,
                topic_name,
                consumer_group,
                partition,
                partition_msgs,
                lease_deadline,
                deadline,
            )
        })
        .collect();

    let outcomes = futures_util::stream::iter(partition_futures)
        .buffer_unordered(limit)
        .collect::<Vec<_>>()
        .await;

    outcomes.into_iter().any(|stopped| stopped)
}

/// Delivers one partition's messages in offset order and commits the contiguously-delivered prefix.
/// Returns `true` if delivery stopped early. This can happen if a message failed all
/// its retries, or the commit failed.
#[allow(clippy::too_many_arguments)]
async fn deliver_and_commit_partition<F>(
    handle: &F,
    http: &reqwest::Client,
    compiled: &CompiledSink<'_>,
    namespace_id: NamespaceId,
    topic_name: &TopicName,
    consumer_group: &ConsumerGroup,
    partition: Partition,
    msgs: Vec<&StreamReceiveMsg>,
    lease_deadline: Instant,
    deadline: Instant,
) -> bool
where
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    let mut last_ok = None;
    let mut stopped = false;
    for msg in msgs {
        let now = Instant::now();
        if now + SINK_HTTP_TIMEOUT > lease_deadline || now >= deadline {
            stopped = true;
            break;
        }
        match deliver_with_retry(http, compiled, msg, lease_deadline, deadline).await {
            Delivery::Delivered => last_ok = Some(msg.offset),
            Delivery::Stopped => {
                stopped = true;
                break;
            }
            Delivery::Failed => {
                tracing::error!(
                    offset = msg.offset,
                    "sink delivery failed after all retries; leaving uncommitted for a later cycle"
                );
                stopped = true;
                break;
            }
        }
    }

    if let Some(offset) = last_ok
        && let Err(e) = commit_offset(
            handle,
            namespace_id,
            topic_name,
            consumer_group,
            partition,
            offset,
        )
        .await
    {
        tracing::error!(error = %e, "failed to commit sink offset");
        stopped = true;
    }

    stopped
}

async fn commit_offset<F>(
    handle: &F,
    namespace_id: NamespaceId,
    topic_name: &TopicName,
    consumer_group: &ConsumerGroup,
    partition: Partition,
    offset: Offset,
) -> Result<(), String>
where
    F: OperationWriter<MsgsOperation> + Send + Sync,
{
    let tp = TopicPartition::new(topic_name.clone(), partition);
    let commit = StreamCommitOperation::new(namespace_id, tp, consumer_group.clone(), offset);
    let response = handle
        .write_request(commit)
        .await
        .map_err(|e| format!("{e:?}"))?;
    response.0.map_err(|e| format!("{e:?}"))?;
    Ok(())
}

enum Delivery {
    Delivered,
    /// Every attempt within the lease failed, so the message is left uncommitted for a later cycle.
    Failed,
    Stopped,
}

/// Delivers a single message, retrying on failure with a short random pause between attempts, up to
/// [`SINK_DELIVERY_RETRIES`] times. A retry whose pause would push the delivery past the lease/task
/// budget stops early, leaving the message uncommitted for a fresh attempt next cycle.
async fn deliver_with_retry(
    http: &reqwest::Client,
    compiled: &CompiledSink<'_>,
    msg: &StreamReceiveMsg,
    lease_deadline: Instant,
    deadline: Instant,
) -> Delivery {
    for attempt in 0..=SINK_DELIVERY_RETRIES {
        if attempt > 0 {
            let delay = jitter(SINK_RETRY_MIN_INTERVAL..SINK_RETRY_MAX_INTERVAL);
            let now = Instant::now();
            if now + delay + SINK_HTTP_TIMEOUT > lease_deadline || now + delay >= deadline {
                return Delivery::Stopped;
            }
            tokio::time::sleep(delay).await;
        }

        match compiled.deliver_one(http, msg).await {
            Ok(()) => return Delivery::Delivered,
            Err(e) => {
                tracing::warn!(error = %e, offset = msg.offset, attempt, "sink delivery failed");
            }
        }
    }

    Delivery::Failed
}

enum CompiledSink<'a> {
    Http(CompiledHttpSink<'a>),
    Svix(CompiledSvixSink<'a>),
    #[cfg(feature = "kafka")]
    Kafka(CompiledKafkaSink<'a>),
}

impl CompiledSink<'_> {
    async fn deliver_one(
        &self,
        http: &reqwest::Client,
        msg: &StreamReceiveMsg,
    ) -> Result<(), String> {
        match self {
            CompiledSink::Http(sink) => sink.deliver(http, msg).await,
            CompiledSink::Svix(sink) => sink.deliver(http, msg).await,
            #[cfg(feature = "kafka")]
            CompiledSink::Kafka(sink) => sink.deliver(msg).await,
        }
    }
}

/// Build the template variables for a message.
fn build_vars(msg: &StreamReceiveMsg) -> HashMap<String, String> {
    let mut vars = HashMap::new();

    for (key, value) in &msg.headers {
        vars.insert(format!("headers.{key}"), value.clone());
    }

    vars.insert(
        "value".to_owned(),
        String::from_utf8_lossy(&msg.value).into_owned(),
    );
    vars.insert("offset".to_owned(), msg.offset.to_string());
    vars.insert(
        "partition".to_owned(),
        msg.topic.partition.get().to_string(),
    );
    vars.insert("topic".to_owned(), msg.topic.topic.to_string());
    vars.insert(
        "timestamp".to_owned(),
        msg.timestamp.as_millisecond().to_string(),
    );

    vars
}

/// Sends a prepared request and maps the outcome to the sink delivery contract (any 2xx is success).
async fn send_request(request: reqwest::RequestBuilder) -> Result<(), String> {
    let response = request
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    let status = response.status();
    if status.is_success() {
        Ok(())
    } else {
        Err(format!("sink returned status {status}"))
    }
}

impl<F: OperationWriter<MsgsOperation>> diom_operations::workers::BackgroundWorker
    for LeaderWorker<F>
{
    const NAME: &'static str = "leader-worker:msg-sink";

    async fn run(self) -> BackgroundResult<()> {
        let mut timer = tokio::time::interval(self.poll_interval);
        let shutting_down = diom_core::shutdown::shutting_down_token();

        while shutting_down
            .run_until_cancelled(timer.tick())
            .await
            .is_some()
        {
            self.poll_cycle().await?;
        }

        Ok(())
    }
}
