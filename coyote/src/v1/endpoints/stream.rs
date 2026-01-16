// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use coyote_derive::aide_annotate;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::Result,
    v1::utils::{openapi_tag, ValidatedJson},
    AppState,
};

// Re-export types that are used in AppState
pub use crate::v1::modules::stream::worker;
pub use crate::v1::modules::stream::StreamStore;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamPublishIn {
    #[validate(nested)]
    pub name: EntityKey,

    /// Array of message payloads to publish
    pub messages: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamPublishOut {
    /// Number of messages published
    pub count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamReadIn {
    #[validate(nested)]
    pub name: EntityKey,

    /// Offset to start reading from (0-indexed)
    pub start_offset: u64,

    /// Maximum number of messages to read (default: 10, max: 1000)
    #[serde(default = "default_read_limit")]
    #[validate(range(min = 1, max = 1000))]
    pub limit: u32,
}

fn default_read_limit() -> u32 {
    10
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamMessage {
    pub id: String,
    pub offset: u64,
    pub payload: String,
    pub published_at: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamReadOut {
    /// Array of messages
    pub messages: Vec<StreamMessage>,
    /// Whether there are more messages available after these
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamTopicInfoIn {
    #[validate(nested)]
    pub name: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamTopicInfoOut {
    /// Total number of messages in the topic
    pub message_count: u64,
    /// Earliest available offset (usually 0)
    pub earliest_offset: u64,
    /// Latest offset (highest offset in the topic)
    pub latest_offset: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct StreamPurgeIn {
    #[validate(nested)]
    pub name: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StreamPurgeOut {
    /// Number of messages purged
    pub purged_count: u64,
}

// ============================================================================
// API Endpoints
// ============================================================================

/// Publish messages to a stream topic
#[aide_annotate(op_id = "v1.stream.publish")]
async fn stream_publish(
    State(AppState { stream_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<StreamPublishIn>,
) -> Result<Json<StreamPublishOut>> {
    let name = data.name.to_string();
    let count = data.messages.len() as u32;

    stream_store.publish(&name, data.messages)?;

    Ok(Json(StreamPublishOut { count }))
}

/// Read messages from a stream topic
#[aide_annotate(op_id = "v1.stream.read")]
async fn stream_read(
    State(AppState { stream_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<StreamReadIn>,
) -> Result<Json<StreamReadOut>> {
    let name = data.name.to_string();

    let (messages, has_more) = stream_store.read(&name, data.start_offset, data.limit)?;

    let messages = messages
        .into_iter()
        .map(|msg| StreamMessage {
            id: msg.id,
            offset: msg.offset,
            payload: msg.payload,
            published_at: msg.published_at,
        })
        .collect();

    Ok(Json(StreamReadOut { messages, has_more }))
}

/// Get information about a stream topic
#[aide_annotate(op_id = "v1.stream.topic_info")]
async fn stream_topic_info(
    State(AppState { stream_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<StreamTopicInfoIn>,
) -> Result<Json<StreamTopicInfoOut>> {
    let name = data.name.to_string();

    let info = stream_store.topic_info(&name)?;

    Ok(Json(StreamTopicInfoOut {
        message_count: info.message_count,
        earliest_offset: info.earliest_offset,
        latest_offset: info.latest_offset,
    }))
}

/// Purge all messages from a stream topic
#[aide_annotate(op_id = "v1.stream.purge")]
async fn stream_purge(
    State(AppState { stream_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<StreamPurgeIn>,
) -> Result<Json<StreamPurgeOut>> {
    let name = data.name.to_string();

    let purged_count = stream_store.purge(&name)?;

    Ok(Json(StreamPurgeOut { purged_count }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Stream");

    ApiRouter::new()
        .api_route_with(
            "/stream/publish",
            post_with(stream_publish, stream_publish_operation),
            &tag,
        )
        .api_route_with(
            "/stream/read",
            post_with(stream_read, stream_read_operation),
            &tag,
        )
        .api_route_with(
            "/stream/topic-info",
            post_with(stream_topic_info, stream_topic_info_operation),
            &tag,
        )
        .api_route_with(
            "/stream/purge",
            post_with(stream_purge, stream_purge_operation),
            &tag,
        )
}
