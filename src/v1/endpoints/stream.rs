// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    num::{NonZeroU16, NonZeroU64},
    time::Duration,
};

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_derive::aide_annotate;
use diom_error::{Result, ResultExt};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stream::entities::{ConsumerGroup, MsgId, MsgIn, MsgOut, StreamName};
use validator::Validate;

use crate::{AppState, core::cluster::RaftState, v1::utils::openapi_tag};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateStreamIn {
    pub name: StreamName,
    /// How long messages are retained in the stream before being permanently nuked.
    pub retention_period_seconds: Option<NonZeroU64>,
    /// How many bytes in total the stream will retain before dropping data.
    pub max_byte_size: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CreateStreamOut {
    pub name: StreamName,
    pub retention_period_seconds: Option<NonZeroU64>,
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Upserts a new Stream with the given name.
#[aide_annotate(op_id = "v1.stream.create")]
async fn create_stream(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CreateStreamIn>,
) -> Result<MsgPackOrJson<CreateStreamOut>> {
    let operation = stream::operations::CreateStreamOperation::new(
        data.name,
        data.retention_period_seconds,
        data.max_byte_size,
    );
    let response = repl.client_write(operation).await.map_err_generic()?.0?;

    Ok(MsgPackOrJson(CreateStreamOut {
        name: response.name,
        retention_period_seconds: response.retention_period_seconds,
        max_byte_size: response.max_byte_size,
        created_at: response.created_at,
        updated_at: response.updated_at,
    }))
}

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
    let group = state.get_stream(&data.name)?;
    let operation = stream::operations::AppendOperation::new(group.id, data.msgs);
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
    MsgPackOrJson(data): MsgPackOrJson<FetchFromStreamIn>,
) -> Result<MsgPackOrJson<FetchFromStreamOut>> {
    /*
    FIXME(@svix-gabriel)

    This is missing a few important things
        1. We haven't setup thread-per-core, so this could go to any thread.
        2. We haven't setup quorum/raft stuff yet, so there's no consensus..

    I didn't want to let either of these things block developing stream,
    so in practice the structure of this handler will look different once those two pieces are in place.
    */

    let group = state.get_stream(&data.name)?;
    let stream_state = state.stream_state;
    let out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::FetchLocking::new(
            &stream_state,
            group.id,
            data.consumer_group,
            data.batch_size,
            Duration::from_secs(data.visibility_timeout_seconds),
        )?;
        op.apply_operation(&stream_state)
    })
    .await??;

    Ok(MsgPackOrJson(FetchFromStreamOut { msgs: out.msgs }))
}

/// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
///
/// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
/// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
/// until the visibility timeout expires, or the messages are acked.
#[aide_annotate(op_id = "v1.stream.fetch")]
async fn fetch_from_stream(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<FetchFromStreamIn>,
) -> Result<MsgPackOrJson<FetchFromStreamOut>> {
    /*
    FIXME(@svix-gabriel)

    This is missing a few important things
        1. We haven't setup thread-per-core, so this could go to any thread.
        2. We haven't setup quorum/raft stuff yet, so there's no consensus..

    I didn't want to let either of these things block developing stream,
    so in practice the structure of this handler will look different once those two pieces are in place.
    */

    let group = state.get_stream(&data.name)?;
    let stream_state = state.stream_state;
    let out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::Fetch::new(
            &stream_state,
            group.id,
            data.consumer_group,
            data.batch_size,
            Duration::from_secs(data.visibility_timeout_seconds),
        )?;
        op.apply_operation(&stream_state)
    })
    .await??;

    Ok(MsgPackOrJson(FetchFromStreamOut { msgs: out.msgs }))
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
    let group = state.get_stream(&data.name)?;
    let operation = stream::operations::AckOperation::new(
        group.id,
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
    let group = state.get_stream(&data.name)?;
    let operation = stream::operations::AckOperation::new(
        group.id,
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
    MsgPackOrJson(data): MsgPackOrJson<DlqIn>,
) -> Result<MsgPackOrJson<DlqOut>> {
    let group = state.get_stream(&data.name)?;
    let stream_state = state.stream_state;
    let _out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::Dlq::new(
            &stream_state,
            group.id,
            data.consumer_group,
            data.msg_id,
        )?;
        op.apply_operation(&stream_state)
    })
    .await??;

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
    MsgPackOrJson(data): MsgPackOrJson<RedriveIn>,
) -> Result<MsgPackOrJson<RedriveOut>> {
    let group = state.get_stream(&data.name)?;
    let stream_state = state.stream_state;
    let _out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::Redrive::new(&stream_state, group.id, data.consumer_group)?;
        op.apply_operation(&stream_state)
    })
    .await??;

    Ok(MsgPackOrJson(RedriveOut {}))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Stream");

    ApiRouter::new()
        .api_route_with(
            "/stream/create",
            post_with(create_stream, create_stream_operation),
            &tag,
        )
        .api_route_with(
            "/stream/append",
            post_with(append_to_stream, append_to_stream_operation),
            &tag,
        )
        .api_route_with(
            "/stream/fetch",
            post_with(fetch_from_stream, fetch_from_stream_operation),
            &tag,
        )
        .api_route_with(
            "/stream/fetch-locking",
            post_with(
                locking_fetch_from_stream,
                locking_fetch_from_stream_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/stream/ack-range",
            post_with(ack_range, ack_range_operation),
            &tag,
        )
        .api_route_with("/stream/ack", post_with(ack, ack_operation), &tag)
        .api_route_with("/stream/dlq", post_with(dlq, dlq_operation), &tag)
        .api_route_with(
            "/stream/redrive-dlq",
            post_with(redrive, redrive_operation),
            &tag,
        )
}
