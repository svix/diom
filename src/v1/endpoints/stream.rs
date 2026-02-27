#![allow(dead_code)]

use std::num::NonZeroU16;

use aide::axum::ApiRouter;
use axum::{Extension, extract::State};
use coyote_derive::aide_annotate;
use coyote_error::{Error, HttpError, Result, ResultExt};
use coyote_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stream_deprecated::entities::{ConsumerGroup, MsgId, MsgIn, MsgOut, StreamName};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AppendToStreamIn {
    pub name: StreamName,
    pub msgs: Vec<MsgIn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AppendToStreamOut {
    pub msg_ids: Vec<MsgId>,
}

/// Appends messages to the stream.
#[aide_annotate(op_id = "v1.stream.append")]
async fn append_to_stream(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AppendToStreamIn>,
) -> Result<MsgPackOrJson<AppendToStreamOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::AppendOperation::new(namespace.id, data.msgs);
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(AppendToStreamOut {
        msg_ids: response.msg_ids,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct FetchFromStreamIn {
    name: StreamName,
    consumer_group: ConsumerGroup,

    /// How many messages to read from the stream.
    batch_size: NonZeroU16,

    /// How long messages are locked by the consumer.
    visibility_timeout_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct FetchFromStreamOut {
    msgs: Vec<MsgOut>,
}

/// Fetches messages from the stream, locking over the consumer group.
///
/// This call prevents other consumers within the same consumer group from reading from the stream
/// until either the visibility timeout expires, or the last message in the batch is acknowledged.
#[aide_annotate(op_id = "v1.stream.fetch-locking")]
async fn locking_fetch_from_stream(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<FetchFromStreamIn>,
) -> Result<MsgPackOrJson<FetchFromStreamOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::FetchLockingOperation::new(
        namespace.id,
        data.consumer_group,
        data.batch_size,
        data.visibility_timeout_seconds,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(FetchFromStreamOut {
        msgs: response.msgs,
    }))
}

/// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
///
/// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
/// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
/// until the visibility timeout expires, or the messages are acked.
#[aide_annotate(op_id = "v1.stream.fetch")]
async fn fetch_from_stream(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<FetchFromStreamIn>,
) -> Result<MsgPackOrJson<FetchFromStreamOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::FetchOperation::new(
        namespace.id,
        data.consumer_group,
        data.batch_size,
        data.visibility_timeout_seconds,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(FetchFromStreamOut {
        msgs: response.msgs,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AckMsgRangeIn {
    name: StreamName,
    consumer_group: ConsumerGroup,
    min_msg_id: Option<MsgId>,
    max_msg_id: MsgId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AckMsgRangeOut {}

/// Acks the messages for the consumer group, allowing more messages to be consumed.
#[aide_annotate(op_id = "v1.stream.ack-range")]
async fn ack_range(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AckMsgRangeIn>,
) -> Result<MsgPackOrJson<AckMsgRangeOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::AckOperation::new(
        namespace.id,
        data.consumer_group,
        data.min_msg_id.unwrap_or(MsgId::MIN),
        data.max_msg_id,
    );
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(AckMsgRangeOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct Ack {
    name: StreamName,
    consumer_group: ConsumerGroup,
    msg_id: MsgId,
}

/// Acks a single message.
#[aide_annotate(op_id = "v1.stream.ack")]
async fn ack(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<Ack>,
) -> Result<MsgPackOrJson<AckOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::AckOperation::new(
        namespace.id,
        data.consumer_group,
        data.msg_id,
        data.msg_id,
    );
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(AckOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AckOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct DlqIn {
    name: StreamName,
    consumer_group: ConsumerGroup,
    msg_id: MsgId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct DlqOut {}

/// Moves a message to the dead letter queue.
#[aide_annotate(op_id = "v1.stream.dlq")]
async fn dlq(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<DlqIn>,
) -> Result<MsgPackOrJson<DlqOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation = stream_deprecated::operations::DlqOperation::new(
        namespace.id,
        data.consumer_group,
        data.msg_id,
    );
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(DlqOut {}))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RedriveIn {
    name: StreamName,
    consumer_group: ConsumerGroup,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct RedriveOut {}

/// Redrives messages from the dead letter queue back to the stream.
#[aide_annotate(op_id = "v1.stream.redrive")]
async fn redrive(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<RedriveIn>,
) -> Result<MsgPackOrJson<RedriveOut>> {
    let namespace = state
        .namespace_state
        .fetch_stream_namespace(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    let operation =
        stream_deprecated::operations::RedriveOperation::new(namespace.id, data.consumer_group);
    repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(RedriveOut {}))
}

pub fn router() -> ApiRouter<AppState> {
    // let tag = openapi_tag("Stream");

    ApiRouter::new()
    // .api_route_with(
    //     "/stream/append",
    //     post_with(append_to_stream, append_to_stream_operation),
    //     &tag,
    // )
    // .api_route_with(
    //     "/stream/fetch",
    //     post_with(fetch_from_stream, fetch_from_stream_operation),
    //     &tag,
    // )
    // .api_route_with(
    //     "/stream/fetch-locking",
    //     post_with(
    //         locking_fetch_from_stream,
    //         locking_fetch_from_stream_operation,
    //     ),
    //     &tag,
    // )
    // .api_route_with(
    //     "/stream/ack-range",
    //     post_with(ack_range, ack_range_operation),
    //     &tag,
    // )
    // .api_route_with("/stream/ack", post_with(ack, ack_operation), &tag)
    // .api_route_with("/stream/dlq", post_with(dlq, dlq_operation), &tag)
    // .api_route_with(
    //     "/stream/redrive-dlq",
    //     post_with(redrive, redrive_operation),
    //     &tag,
    // )
}
