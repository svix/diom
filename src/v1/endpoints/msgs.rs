use std::num::NonZeroU16;

use crate::{AppState, core::cluster::RaftState, v1::utils::openapi_tag};
use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_core::types::DurationMs;
use coyote_derive::aide_annotate;
use coyote_error::{Error, OptionExt, Result, ResultExt};
use coyote_msgs::{
    MsgsNamespace,
    entities::{
        ConsumerGroup, MsgId, Offset, QueueMsgOut, Retention, SeekPosition, StreamMsgOut, TopicIn,
        TopicName, TopicPartition, default_retention_bytes, default_retention_millis,
    },
    operations::{
        CreateNamespaceOperation, PublishOperation, QueueAckOperation, QueueConfigureOperation,
        QueueNackOperation, QueueReceiveOperation, QueueRedriveDlqOperation, SeekTarget,
        StreamCommitOperation, StreamReceiveOperation, StreamSeekOperation,
        TopicConfigureOperation,
    },
};
use coyote_namespace::entities::{NamespaceName, StorageType};
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["name"]))]
pub(crate) struct MsgNamespaceCreateIn {
    pub name: NamespaceName,
    #[serde(default)]
    pub retention: Retention,
    #[serde(default)]
    pub storage_type: StorageType,
}

impl From<MsgNamespaceCreateIn> for CreateNamespaceOperation {
    fn from(v: MsgNamespaceCreateIn) -> Self {
        CreateNamespaceOperation::new(v.name, v.retention, v.storage_type)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct MsgNamespaceCreateOut {
    pub name: NamespaceName,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Creates or updates a msgs namespace with the given name.
#[aide_annotate(op_id = "v1.msgs.namespace.create")]
async fn create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgNamespaceCreateIn>,
) -> Result<MsgPackOrJson<MsgNamespaceCreateOut>> {
    let operation = CreateNamespaceOperation::from(data);
    let response = repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgNamespaceCreateOut {
        name: response.name,
        retention: response.retention,
        storage_type: response.storage_type,
        created: response.created,
        updated: response.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["name"]))]
struct MsgNamespaceGetIn {
    pub name: NamespaceName,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct MsgNamespaceGetOut {
    pub name: NamespaceName,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
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

    let millis = u64::try_from(namespace.config.retention_period.as_millis())
        .ok()
        .and_then(|ms| ms.try_into().ok())
        .unwrap_or_else(default_retention_millis);
    let bytes = namespace
        .max_storage_bytes
        .unwrap_or_else(default_retention_bytes);

    Ok(MsgPackOrJson(MsgNamespaceGetOut {
        name: namespace.name,
        retention: Retention { millis, bytes },
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["topic"]))]
struct MsgPublishIn {
    #[serde(default)]
    pub namespace: Option<NamespaceName>,
    pub topic: TopicIn,
    pub msgs: Vec<coyote_msgs::entities::MsgIn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct MsgPublishOutTopic {
    pub topic: TopicPartition,
    pub start_offset: Offset,
    pub offset: Offset,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct MsgPublishOut {
    pub topics: Vec<MsgPublishOutTopic>,
}

/// Publishes messages to a topic within a namespace.
#[aide_annotate(op_id = "v1.msgs.publish")]
async fn publish(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<MsgPublishIn>,
) -> Result<MsgPackOrJson<MsgPublishOut>> {
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = PublishOperation::new(namespace.id, data.topic, data.msgs)?;
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

fn default_batch_size() -> NonZeroU16 {
    NonZeroU16::new(10).unwrap()
}

fn default_lease_duration_millis() -> DurationMs {
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
    #[serde(default = "default_lease_duration_millis")]
    pub lease_duration_millis: DurationMs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = StreamReceiveOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.batch_size,
        data.lease_duration_millis,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
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

fn default_queue_lease_duration_millis() -> DurationMs {
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
    #[serde(default = "default_queue_lease_duration_millis")]
    pub lease_duration_millis: DurationMs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = QueueReceiveOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.batch_size,
        data.lease_duration_millis,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation =
        QueueAckOperation::new(namespace.id, data.topic, data.consumer_group, data.msg_ids);
    repl.client_write(operation).await.or_internal_error()?.0?;

    Ok(MsgPackOrJson(MsgQueueAckOut {}))
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
    let namespace: MsgsNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation =
        QueueNackOperation::new(namespace.id, data.topic, data.consumer_group, data.msg_ids);
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
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
        .fetch_namespace(data.namespace.as_deref())?
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
            "/msgs/namespace/create",
            post_with(create_namespace, create_namespace_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/namespace/get",
            post_with(get_namespace, get_namespace_operation),
            &tag,
        )
        .api_route_with("/msgs/publish", post_with(publish, publish_operation), &tag)
        .api_route_with(
            "/msgs/stream/receive",
            post_with(stream_receive, stream_receive_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/stream/commit",
            post_with(stream_commit, stream_commit_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/stream/seek",
            post_with(stream_seek, stream_seek_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/queue/receive",
            post_with(queue_receive, queue_receive_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/queue/ack",
            post_with(queue_ack, queue_ack_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/queue/configure",
            post_with(queue_configure, queue_configure_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/queue/nack",
            post_with(queue_nack, queue_nack_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/queue/redrive-dlq",
            post_with(queue_redrive_dlq, queue_redrive_dlq_operation),
            &tag,
        )
        .api_route_with(
            "/msgs/topic/configure",
            post_with(topic_configure, topic_configure_operation),
            &tag,
        )
}
