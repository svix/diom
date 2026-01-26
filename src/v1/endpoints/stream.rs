// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Json, extract::State};
use coyote_derive::aide_annotate;
use coyote_error::Result;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stream::{
    entities::{MsgId, StreamId},
    operations::{CreateStreamOutput, MsgIn},
};
use validator::Validate;

use crate::{
    AppState,
    v1::utils::{ValidatedJson, openapi_tag},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamIn {
    pub name: String,
    /// How long messages are retained in the stream before being permanently nuked.
    pub retention_period_seconds: Option<NonZeroU64>,
    /// How many bytes in total the stream will retain before dropping data.
    pub max_byte_size: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamOut {
    pub id: StreamId,
    pub name: String,
    pub retention_period_seconds: Option<NonZeroU64>,
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Upserts a new Stream with the given name.
#[aide_annotate(op_id = "v1.stream.create")]
async fn create_stream(
    State(AppState { stream_state, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<CreateStreamIn>,
) -> Result<Json<CreateStreamOut>> {
    /*
    FIXME(@svix-gabriel)

    This is missing a few important things
        1. We haven't setup thread-per-core, so this could go to any thread.
        2. We haven't setup quorum/raft stuff yet, so there's no consensus.

    I didn't want to let either of these things block developing stream,
    so in practice the structure of this handler will look different once those two pieces are in place.
    */

    let out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::CreateStream::new(
            &stream_state,
            data.name,
            data.retention_period_seconds,
            data.max_byte_size,
        )?;

        op.apply_operation(&stream_state)
    })
    .await??;

    let CreateStreamOutput {
        id,
        name,
        retention_period_seconds,
        max_byte_size,
        created_at,
        updated_at,
    } = out;

    Ok(Json(CreateStreamOut {
        id,
        name,
        retention_period_seconds,
        max_byte_size,
        created_at,
        updated_at,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppendToStreamIn {
    pub stream_id: StreamId,
    pub msgs: Vec<MsgIn>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppendToStreamOut {
    pub msg_ids: Vec<MsgId>,
}

/// Appends messages to the stream.
#[aide_annotate(op_id = "v1.stream.append")]
async fn append_to_stream(
    State(AppState { stream_state, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<AppendToStreamIn>,
) -> Result<Json<AppendToStreamOut>> {
    /*
    FIXME(@svix-gabriel)

    This is missing a few important things
        1. We haven't setup thread-per-core, so this could go to any thread.
        2. We haven't setup quorum/raft stuff yet, so there's no consensus.

    I didn't want to let either of these things block developing stream,
    so in practice the structure of this handler will look different once those two pieces are in place.
    */

    let out = tokio::task::spawn_blocking(move || {
        let op = stream::operations::AppendToStream::new(&stream_state, data.stream_id, data.msgs)?;
        op.apply_operation(&stream_state)
    })
    .await??;

    Ok(Json(AppendToStreamOut {
        msg_ids: out.msg_ids,
    }))
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
}
