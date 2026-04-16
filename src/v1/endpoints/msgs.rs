use std::num::NonZeroU16;

use crate::{AppState, core::cluster::RaftState, v1::utils::openapi_tag};
use aide::axum::{ApiRouter, routing::post_with};
use axum::{
    Extension,
    extract::State,
    http::{HeaderMap, header::AUTHORIZATION},
};
use diom_authorization::RequestedOperation;
use diom_core::types::{DurationMs, UnixTimestampMs};
use diom_derive::aide_annotate;
use diom_error::{Error, OptionExt, Result, ResultExt};
use diom_id::Module;
use diom_msgs::{
    MsgsNamespace,
    entities::{
        ConsumerGroup, MsgId, MsgsIdempotencyKey, Offset, QueueMsgOut, Retention, SeekPosition,
        StreamMsgOut, TopicIn, TopicName, TopicPartition,
    },
    operations::{
        ConfigureNamespaceOperation, PublishOperation, QueueAckOperation, QueueConfigureOperation,
        QueueExtendLeaseOperation, QueueNackOperation, QueueReceiveOperation,
        QueueRedriveDlqOperation, SeekTarget, StreamCommitOperation, StreamReceiveOperation,
        StreamSeekOperation, TopicConfigureOperation,
    },
};
use diom_namespace::entities::NamespaceName;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use fjall_utils::{ReadableDatabase, ReadonlyConnection, StorageType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

fn msgs_metadata<'a>(
    ns: Option<&'a NamespaceName>,
    tp: &'a TopicName,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::Msgs,
        namespace: ns.map(|n| n.as_str()),
        key: Some(tp),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                msgs_metadata(self.namespace.as_ref(), self.topic.name(), $action)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["name"]))]
pub(crate) struct MsgNamespaceConfigureIn {
    pub name: NamespaceName,
    #[serde(default)]
    pub retention: Retention,
}

namespace_request_input!(MsgNamespaceConfigureIn, "configure");

impl From<MsgNamespaceConfigureIn> for ConfigureNamespaceOperation {
    fn from(v: MsgNamespaceConfigureIn) -> Self {
        ConfigureNamespaceOperation::new(v.name, v.retention)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgNamespaceConfigureOut {
    pub name: NamespaceName,
    pub retention: Retention,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Configures a msgs namespace with the given name.
#[aide_annotate(op_id = "v1.msgs.namespace.configure")]
async fn configure_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgNamespaceConfigureIn>,
) -> Result<MsgPackOrJson<MsgNamespaceConfigureOut>> {
    let operation = ConfigureNamespaceOperation::from(data);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgNamespaceConfigureOut {
        name: response.name,
        retention: response.retention,
        created: response.created,
        updated: response.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["name"]))]
struct MsgNamespaceGetIn {
    pub name: NamespaceName,
}

namespace_request_input!(MsgNamespaceGetIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgNamespaceGetOut {
    pub name: NamespaceName,
    pub retention: Retention,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Gets a msgs namespace by name.
#[aide_annotate(op_id = "v1.msgs.namespace.get")]
async fn get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgNamespaceGetIn>,
) -> Result<MsgPackOrJson<MsgNamespaceGetOut>> {
    // Ensure we have the latest version of namespace
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(MsgNamespaceGetOut {
        name: namespace.name,
        retention: Retention {
            period: namespace.config.retention_period,
            size_bytes: namespace.config.retention_bytes,
        },
        created: namespace.created,
        updated: namespace.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic"]))]
struct MsgPublishIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicIn,
    pub msgs: Vec<diom_msgs::entities::MsgIn>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

request_input!(MsgPublishIn, "publish");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgPublishOutTopic {
    pub topic: TopicPartition,
    pub start_offset: Offset,
    pub offset: Offset,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgPublishOut {
    pub topics: Vec<MsgPublishOutTopic>,
}

/// Publishes messages to a topic within a namespace.
#[aide_annotate(op_id = "v1.msgs.publish")]
async fn publish(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    headers: HeaderMap,
    MsgPackOrJson(data): MsgPackOrJson<MsgPublishIn>,
) -> Result<MsgPackOrJson<MsgPublishOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let topic_name = data.topic.name();
    let authorization_header = headers.get(AUTHORIZATION).map(|c| c.as_bytes());
    let idempotency_key = data
        .idempotency_key
        .as_deref()
        .map(|key| MsgsIdempotencyKey::new(authorization_header, topic_name, key));
    let operation = PublishOperation::new(namespace.id, data.topic, data.msgs, idempotency_key)?;
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgPublishOut {
        topics: response
            .topics
            .into_iter()
            .map(|m| MsgPublishOutTopic {
                topic: m.topic,
                start_offset: m.start_offset,
                offset: m.offset,
            })
            .collect(),
    }))
}

// ---------------------------------------------------------------------------
// stream/receive
// ---------------------------------------------------------------------------

const fn default_batch_size() -> NonZeroU16 {
    NonZeroU16::new(10).unwrap()
}

const fn default_lease_duration_ms() -> DurationMs {
    DurationMs::from_mins(5)
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgStreamReceiveIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicIn,
    pub consumer_group: ConsumerGroup,
    #[serde(default = "default_batch_size")]
    pub batch_size: NonZeroU16,
    #[serde(rename = "lease_duration_ms", default = "default_lease_duration_ms")]
    pub lease_duration: DurationMs,
    #[serde(default)]
    pub default_starting_position: SeekPosition,
    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(rename = "batch_wait_ms", default)]
    pub batch_wait: Option<DurationMs>,
}

request_input!(MsgStreamReceiveIn, "stream.receive");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgStreamReceiveOut {
    pub msgs: Vec<StreamMsgOut>,
}

/// Receives messages from a topic using a consumer group.
///
/// Each consumer in the group reads from all partitions. Messages are locked by leases for the
/// specified duration to prevent duplicate delivery within the same consumer group.
#[aide_annotate(op_id = "v1.msgs.stream.receive")]
async fn stream_receive(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgStreamReceiveIn>,
) -> Result<MsgPackOrJson<MsgStreamReceiveOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    if let Some(max_wait) = data.batch_wait {
        let ro_db = state.ro_dbs.db_for(StorageType::Persistent);
        let metadata_ks = ro_db
            .keyspace(diom_msgs::METADATA_KEYSPACE)
            .or_internal_error()?;
        let msg_ks = ro_db
            .keyspace(diom_msgs::MSG_KEYSPACE)
            .or_internal_error()?;

        let ns_id = namespace.id;
        let topic_name = data.topic.name().clone();
        let cg = data.consumer_group.clone();
        let now = state.time.now_utm();
        let batch_size = data.batch_size;

        let estimate = diom_core::task::spawn_blocking_in_current_span(move || {
            diom_msgs::estimate_available_stream_messages(
                &metadata_ks,
                &msg_ks,
                ns_id,
                &topic_name,
                &cg,
                now,
            )
        })
        .await
        .or_internal_error()??;

        tracing::trace!(?estimate, "estimate of available messages");

        if (estimate.count as usize) < batch_size.get() as usize {
            let needed = batch_size.get() as u64 - estimate.count;
            let mut notified = state.topic_publish_notifier.register_notifier(
                namespace.id,
                data.topic.name().clone(),
                estimate.available_partitions,
            );
            tracing::trace!(needed, "waiting for more messages");
            let _ = tokio::time::timeout(max_wait.into(), notified.wait(needed)).await;
        }
    }

    let operation = StreamReceiveOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.batch_size,
        data.lease_duration,
        data.default_starting_position,
        namespace.config.retention_period,
    )?;
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgStreamReceiveOut {
        msgs: response
            .msgs
            .into_iter()
            .map(|m| StreamMsgOut {
                offset: m.offset,
                topic: m.topic,
                value: m.value,
                headers: m.headers,
                timestamp: m.timestamp,
                scheduled_at: m.scheduled_at,
            })
            .collect(),
    }))
}

// ---------------------------------------------------------------------------
// stream/commit
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgStreamCommitIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicPartition,
    pub consumer_group: ConsumerGroup,
    pub offset: u64,
}

request_input!(MsgStreamCommitIn, "stream.commit");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgStreamCommitOut {}

/// Commits an offset for a consumer group on a specific partition.
///
/// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
/// successfully processed offset; future receives will start after it.
#[aide_annotate(op_id = "v1.msgs.stream.commit")]
async fn stream_commit(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgStreamCommitIn>,
) -> Result<MsgPackOrJson<MsgStreamCommitOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation =
        StreamCommitOperation::new(namespace.id, data.topic, data.consumer_group, data.offset);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgStreamCommitOut {}))
}

// ---------------------------------------------------------------------------
// stream/seek
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgStreamSeekIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicIn,
    pub consumer_group: ConsumerGroup,
    pub offset: Option<Offset>,
    pub position: Option<SeekPosition>,
}

request_input!(MsgStreamSeekIn, "stream.seek");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgStreamSeekOut {}

/// Repositions a consumer group's read cursor on a topic.
///
/// Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
/// partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
/// `"latest"` and may be used with or without a partition suffix.
#[aide_annotate(op_id = "v1.msgs.stream.seek")]
async fn stream_seek(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgStreamSeekIn>,
) -> Result<MsgPackOrJson<MsgStreamSeekOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let target = match (data.offset, data.position) {
        (Some(offset), None) => SeekTarget::Offset(offset),
        (None, Some(position)) => SeekTarget::Position(position),
        _ => {
            return Err(Error::invalid_user_input(
                "exactly one of 'offset' or 'position' must be provided",
            ));
        }
    };

    let operation =
        StreamSeekOperation::new(namespace.id, data.topic, data.consumer_group, target)?;
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgStreamSeekOut {}))
}

// ---------------------------------------------------------------------------
// queue/receive
// ---------------------------------------------------------------------------

const fn default_queue_lease_duration() -> DurationMs {
    DurationMs::from_secs(30)
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueReceiveIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicIn,
    pub consumer_group: ConsumerGroup,
    #[serde(default = "default_batch_size")]
    pub batch_size: NonZeroU16,
    #[serde(rename = "lease_duration_ms", default = "default_queue_lease_duration")]
    pub lease_duration: DurationMs,
    /// Maximum time (in milliseconds) to wait for messages before returning.
    #[serde(rename = "batch_wait_ms", default)]
    pub batch_wait: Option<DurationMs>,
}

request_input!(MsgQueueReceiveIn, "queue.receive");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueReceiveOut {
    pub msgs: Vec<QueueMsgOut>,
}

/// Receives messages from a topic as competing consumers.
///
/// Messages are individually leased for the specified duration. Multiple consumers can receive
/// different messages from the same topic concurrently. Leased messages are skipped until they
/// are acked or their lease expires.
#[aide_annotate(op_id = "v1.msgs.queue.receive")]
async fn queue_receive(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueReceiveIn>,
) -> Result<MsgPackOrJson<MsgQueueReceiveOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    if let Some(max_wait) = data.batch_wait {
        let ro_db = state.ro_dbs.db_for(StorageType::Persistent);
        let metadata_ks = ro_db
            .keyspace(diom_msgs::METADATA_KEYSPACE)
            .or_internal_error()?;
        let msg_ks = ro_db
            .keyspace(diom_msgs::MSG_KEYSPACE)
            .or_internal_error()?;

        let ns_id = namespace.id;
        let topic_name = data.topic.name().clone();
        let cg = data.consumer_group.clone();
        let now = state.time.now_utm();
        let batch_size = data.batch_size;

        let estimated = diom_core::task::spawn_blocking_in_current_span(move || {
            diom_msgs::estimate_available_queue_messages(
                &metadata_ks,
                &msg_ks,
                ns_id,
                &topic_name,
                &cg,
                now,
            )
        })
        .await
        .or_internal_error()??;

        if (estimated as usize) < batch_size.get() as usize {
            let needed = batch_size.get() as u64 - estimated;
            let mut notified = state.topic_publish_notifier.register_notifier(
                namespace.id,
                data.topic.name().clone(),
                vec![],
            );
            tracing::trace!(needed, "waiting for more messages");
            let _ = tokio::time::timeout(max_wait.into(), notified.wait(needed)).await;
        }
    }

    let operation = QueueReceiveOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.batch_size,
        data.lease_duration,
        namespace.config.retention_period,
    )?;
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueReceiveOut {
        msgs: response
            .msgs
            .into_iter()
            .map(|m| QueueMsgOut {
                msg_id: m.msg_id,
                value: m.value,
                headers: m.headers,
                timestamp: m.timestamp,
                scheduled_at: m.scheduled_at,
            })
            .collect(),
    }))
}

// ---------------------------------------------------------------------------
// queue/ack
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueAckIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub msg_ids: Vec<MsgId>,
}

request_input!(MsgQueueAckIn, "queue.ack");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueAckOut {}

/// Acknowledges messages by their opaque msg_ids.
///
/// Acked messages are permanently removed from the queue and will never be re-delivered.
#[aide_annotate(op_id = "v1.msgs.queue.ack")]
async fn queue_ack(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueAckIn>,
) -> Result<MsgPackOrJson<MsgQueueAckOut>> {
    if data.msg_ids.is_empty() {
        return Ok(MsgPackOrJson(MsgQueueAckOut {}));
    }

    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation =
        QueueAckOperation::new(namespace.id, data.topic, data.consumer_group, data.msg_ids);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueAckOut {}))
}

// ---------------------------------------------------------------------------
// queue/extend-lease
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueExtendLeaseIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub msg_ids: Vec<MsgId>,
    #[serde(rename = "lease_duration_ms", default = "default_queue_lease_duration")]
    pub lease_duration: DurationMs,
}

request_input!(MsgQueueExtendLeaseIn, "extend-lease");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueExtendLeaseOut {}

/// Extends the lease on in-flight messages.
///
/// Consumers that need more processing time can call this before the lease expires to prevent the
/// message from being re-delivered to another consumer.
#[aide_annotate(op_id = "v1.msgs.queue.extend-lease")]
async fn queue_extend_lease(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueExtendLeaseIn>,
) -> Result<MsgPackOrJson<MsgQueueExtendLeaseOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = QueueExtendLeaseOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.msg_ids,
        data.lease_duration,
    );
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueExtendLeaseOut {}))
}

// ---------------------------------------------------------------------------
// queue/configure
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueConfigureIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    #[serde(default)]
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

request_input!(MsgQueueConfigureIn, "queue.configure");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueConfigureOut {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

/// Configures retry and DLQ behavior for a consumer group on a topic.
///
/// `retry_schedule` is a list of delays (in millis) between retries after a nack. Once exhausted,
/// the message is moved to the DLQ (or forwarded to `dlq_topic` if set).
#[aide_annotate(op_id = "v1.msgs.queue.configure")]
async fn queue_configure(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueConfigureIn>,
) -> Result<MsgPackOrJson<MsgQueueConfigureOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = QueueConfigureOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.retry_schedule,
        data.dlq_topic,
    );
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueConfigureOut {
        retry_schedule: response.retry_schedule,
        dlq_topic: response.dlq_topic,
    }))
}

// ---------------------------------------------------------------------------
// queue/nack
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueNackIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
    pub msg_ids: Vec<MsgId>,
}

request_input!(MsgQueueNackIn, "queue.nack");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueNackOut {}

/// Rejects messages, sending them to the dead-letter queue.
///
/// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
/// move them back to the queue for reprocessing.
#[aide_annotate(op_id = "v1.msgs.queue.nack")]
async fn queue_nack(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueNackIn>,
) -> Result<MsgPackOrJson<MsgQueueNackOut>> {
    if data.msg_ids.is_empty() {
        return Ok(MsgPackOrJson(MsgQueueNackOut {}));
    }

    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = QueueNackOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.msg_ids,
        namespace.config.retention_period,
    );
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueNackOut {}))
}

// ---------------------------------------------------------------------------
// queue/redrive-dlq
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic", "consumer_group"]))]
struct MsgQueueRedriveDlqIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub consumer_group: ConsumerGroup,
}

request_input!(MsgQueueRedriveDlqIn, "queue.redrive-dlq");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgQueueRedriveDlqOut {}

/// Moves all dead-letter queue messages back to the main queue for reprocessing.
#[aide_annotate(op_id = "v1.msgs.queue.redrive-dlq")]
async fn queue_redrive_dlq(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgQueueRedriveDlqIn>,
) -> Result<MsgPackOrJson<MsgQueueRedriveDlqOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = QueueRedriveDlqOperation::new(namespace.id, data.topic, data.consumer_group);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueRedriveDlqOut {}))
}

// ---------------------------------------------------------------------------
// topic/configure
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic"]))]
struct MsgTopicConfigureIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicName,
    pub partitions: u16,
}

request_input!(MsgTopicConfigureIn, "topic.configure");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct MsgTopicConfigureOut {
    pub partitions: u16,
}

/// Configures the number of partitions for a topic.
///
/// Partition count can only be increased, never decreased. The default for a new topic is 1.
#[aide_annotate(op_id = "v1.msgs.topic.configure")]
async fn topic_configure(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgTopicConfigureIn>,
) -> Result<MsgPackOrJson<MsgTopicConfigureOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = TopicConfigureOperation::new(namespace.id, data.topic, data.partitions)?;
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgTopicConfigureOut {
        partitions: response.partitions,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Msgs");

    ApiRouter::new()
        .api_route_with(
            configure_namespace_path,
            post_with(configure_namespace, configure_namespace_operation),
            &tag,
        )
        .api_route_with(
            get_namespace_path,
            post_with(get_namespace, get_namespace_operation),
            &tag,
        )
        .api_route_with(publish_path, post_with(publish, publish_operation), &tag)
        .api_route_with(
            stream_receive_path,
            post_with(stream_receive, stream_receive_operation),
            &tag,
        )
        .api_route_with(
            stream_commit_path,
            post_with(stream_commit, stream_commit_operation),
            &tag,
        )
        .api_route_with(
            stream_seek_path,
            post_with(stream_seek, stream_seek_operation),
            &tag,
        )
        .api_route_with(
            queue_receive_path,
            post_with(queue_receive, queue_receive_operation),
            &tag,
        )
        .api_route_with(
            queue_ack_path,
            post_with(queue_ack, queue_ack_operation),
            &tag,
        )
        .api_route_with(
            queue_extend_lease_path,
            post_with(queue_extend_lease, queue_extend_lease_operation),
            &tag,
        )
        .api_route_with(
            queue_configure_path,
            post_with(queue_configure, queue_configure_operation),
            &tag,
        )
        .api_route_with(
            queue_nack_path,
            post_with(queue_nack, queue_nack_operation),
            &tag,
        )
        .api_route_with(
            queue_redrive_dlq_path,
            post_with(queue_redrive_dlq, queue_redrive_dlq_operation),
            &tag,
        )
        .api_route_with(
            topic_configure_path,
            post_with(topic_configure, topic_configure_operation),
            &tag,
        )
}
