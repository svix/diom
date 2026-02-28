use std::num::NonZeroU16;

use crate::{AppState, core::cluster::RaftState, v1::utils::openapi_tag};
use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_derive::aide_annotate;
use diom_error::{Error, HttpError, Result, ResultExt};
use diom_msgs::entities::MAX_PARTITION_COUNT;
use diom_namespace::entities::StorageType;
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stream_internals::entities::{Retention, default_retention_bytes, default_retention_millis};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateNamespaceIn {
    pub name: String,
    #[serde(default)]
    pub retention: Retention,
    #[serde(default)]
    pub storage_type: StorageType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateNamespaceOut {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Creates or updates a msgs namespace with the given name.
#[aide_annotate(op_id = "v1.msgs.namespace.create")]
async fn create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CreateNamespaceIn>,
) -> Result<MsgPackOrJson<CreateNamespaceOut>> {
    let operation = diom_msgs::operations::CreateNamespaceOperation::new(
        data.name,
        data.retention,
        data.storage_type,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(CreateNamespaceOut {
        name: response.name,
        retention: response.retention,
        storage_type: response.storage_type,
        created: response.created,
        updated: response.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct GetNamespaceIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct GetNamespaceOut {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Gets a msgs namespace by name.
#[aide_annotate(op_id = "v1.msgs.namespace.get")]
async fn get_namespace(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<GetNamespaceIn>,
) -> Result<MsgPackOrJson<GetNamespaceOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let millis = u64::try_from(namespace.config.retention_period.as_millis())
        .ok()
        .and_then(|ms| ms.try_into().ok())
        .unwrap_or_else(default_retention_millis);
    let bytes = namespace
        .max_storage_bytes
        .unwrap_or_else(default_retention_bytes);

    Ok(MsgPackOrJson(GetNamespaceOut {
        name: namespace.name,
        retention: Retention { millis, bytes },
        storage_type: namespace.storage_type,
        created: namespace.created_at,
        updated: namespace.updated_at,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct PublishIn {
    pub name: String,
    pub topic: String,
    pub msgs: Vec<diom_msgs::entities::MsgIn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct PublishOutMsg {
    pub partition: u16,
    pub offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct PublishOut {
    pub msgs: Vec<PublishOutMsg>,
}

/// Publishes messages to a topic within a namespace.
#[aide_annotate(op_id = "v1.msgs.publish")]
async fn publish(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<PublishIn>,
) -> Result<MsgPackOrJson<PublishOut>> {
    if data.topic.contains('~') {
        return Err(Error::http(HttpError::bad_request(
            Some("invalid_topic".to_owned()),
            Some("Topic name must not contain '~'. Use the partition key to route messages to a specific partition.".to_owned()),
        )));
    }

    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let keyless_partition = diom_msgs::entities::random_partition(MAX_PARTITION_COUNT);
    let operation = diom_msgs::operations::PublishOperation::new(
        namespace.id,
        data.topic,
        data.msgs,
        keyless_partition,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(PublishOut {
        msgs: response
            .msgs
            .into_iter()
            .map(|m| PublishOutMsg {
                partition: m.partition.get(),
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

fn default_lease_duration_millis() -> u64 {
    300_000 // 5 minutes
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct StreamReceiveIn {
    pub name: String,
    pub topic: String,
    pub consumer_group: diom_msgs::entities::ConsumerGroup,
    #[serde(default = "default_batch_size")]
    pub batch_size: NonZeroU16,
    #[serde(default = "default_lease_duration_millis")]
    pub lease_duration_millis: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct StreamReceiveOut {
    pub msgs: Vec<diom_msgs::entities::StreamMsgOut>,
}

/// Receives messages from a topic using a consumer group.
///
/// Each consumer in the group reads from all partitions. Messages are locked by leases for the
/// specified duration to prevent duplicate delivery within the same consumer group.
#[aide_annotate(op_id = "v1.msgs.stream.receive")]
async fn stream_receive(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<StreamReceiveIn>,
) -> Result<MsgPackOrJson<StreamReceiveOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = diom_msgs::operations::StreamReceiveOperation::new(
        namespace.id,
        data.topic,
        data.consumer_group,
        data.batch_size,
        data.lease_duration_millis,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(StreamReceiveOut {
        msgs: response
            .msgs
            .into_iter()
            .map(|m| diom_msgs::entities::StreamMsgOut {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct StreamCommitIn {
    pub name: String,
    pub topic: String,
    pub consumer_group: diom_msgs::entities::ConsumerGroup,
    pub offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct StreamCommitOut {}

/// Commits an offset for a consumer group on a specific partition.
///
/// The topic must be a partition-level topic (e.g. `my-topic~3`). The offset is the last
/// successfully processed offset; future receives will start after it.
#[aide_annotate(op_id = "v1.msgs.stream.commit")]
async fn stream_commit(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<StreamCommitIn>,
) -> Result<MsgPackOrJson<StreamCommitOut>> {
    let (_topic, partition) =
        diom_msgs::entities::parse_partition_topic(&data.topic).map_err(|msg| {
            Error::http(HttpError::bad_request(
                Some("invalid_topic".to_owned()),
                Some(msg.to_owned()),
            ))
        })?;

    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = diom_msgs::operations::StreamCommitOperation::new(
        namespace.id,
        partition,
        data.consumer_group,
        data.offset,
    );
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(StreamCommitOut {}))
}

// ---------------------------------------------------------------------------
// topic/configure
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct TopicConfigureIn {
    pub name: String,
    pub topic: String,
    pub partitions: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct TopicConfigureOut {
    pub partitions: u16,
}

/// Configures the number of partitions for a topic.
///
/// Partition count can only be increased, never decreased. The default for a new topic is 1.
#[aide_annotate(op_id = "v1.msgs.topic.configure")]
async fn topic_configure(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<TopicConfigureIn>,
) -> Result<MsgPackOrJson<TopicConfigureOut>> {
    if data.topic.contains('~') {
        return Err(Error::http(HttpError::bad_request(
            Some("invalid_topic".to_owned()),
            Some("Topic name must not contain '~'.".to_owned()),
        )));
    }

    if data.partitions == 0 || data.partitions > MAX_PARTITION_COUNT {
        return Err(Error::http(HttpError::bad_request(
            Some("invalid_partition_count".to_owned()),
            Some(format!(
                "Partition count must be between 1 and {MAX_PARTITION_COUNT}."
            )),
        )));
    }

    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = diom_msgs::operations::TopicConfigureOperation::new(
        namespace.id,
        data.topic,
        data.partitions,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(TopicConfigureOut {
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
            "/msgs/topic/configure",
            post_with(topic_configure, topic_configure_operation),
            &tag,
        )
}
