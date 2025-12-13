// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use coyote_derive::aide_annotate;
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
pub use crate::v1::modules::queue::worker;
pub use crate::v1::modules::queue::{QueueConfiguration, QueueStore};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueSendIn {
    #[validate]
    pub name: EntityKey,

    // FIXME: needs to be bytes.
    /// Message payload
    pub payload: String,

    /// Delay before message becomes available (seconds, default: 0)
    #[serde(default)]
    pub delay_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueSendOut {
    /// Unique message ID
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueReceiveIn {
    #[validate]
    pub name: EntityKey,

    /// Visibility timeout in seconds (how long before message returns to queue if not ack'd)
    #[validate(range(min = 1))]
    pub visibility_timeout_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum QueueReceiveOut {
    Message { message_id: String, payload: String },
    Empty {},
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueAckIn {
    #[validate]
    pub name: EntityKey,

    /// Message ID to acknowledge
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueAckOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueNackIn {
    #[validate]
    pub name: EntityKey,

    /// Message ID to negative acknowledge
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueNackOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueuePurgeIn {
    #[validate]
    pub name: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueuePurgeOut {
    /// Number of messages purged
    pub purged_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueStatsIn {
    #[validate]
    pub name: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueStatsOut {
    /// Number of available messages
    pub available: u64,
    /// Number of in-flight messages
    pub in_flight: u64,
}

// ============================================================================
// API Endpoints
// ============================================================================

/// Send a message to the queue
#[aide_annotate(op_id = "v1.queue.send")]
async fn queue_send(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueSendIn>,
) -> Result<Json<QueueSendOut>> {
    let name = data.name.to_string();
    let message_id = queue_store.enqueue(&name, data.payload, data.delay_seconds)?;

    Ok(Json(QueueSendOut { message_id }))
}

/// Receive a message from the queue
#[aide_annotate(op_id = "v1.queue.receive")]
async fn queue_receive(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueReceiveIn>,
) -> Result<Json<QueueReceiveOut>> {
    let name = data.name.to_string();

    match queue_store.dequeue(&name, data.visibility_timeout_seconds)? {
        Some((message_id, payload)) => Ok(Json(QueueReceiveOut::Message {
            message_id,
            payload,
        })),
        None => Ok(Json(QueueReceiveOut::Empty {})),
    }
}

/// Acknowledge successful message processing
#[aide_annotate(op_id = "v1.queue.ack")]
async fn queue_ack(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueAckIn>,
) -> Result<Json<QueueAckOut>> {
    let name = data.name.to_string();

    queue_store.ack(&name, &data.message_id)?;

    Ok(Json(QueueAckOut {}))
}

/// Negative acknowledge - return message to queue or move to DLQ
#[aide_annotate(op_id = "v1.queue.nack")]
async fn queue_nack(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueNackIn>,
) -> Result<Json<QueueNackOut>> {
    let name = data.name.to_string();

    queue_store.nack(&name, &data.message_id)?;

    Ok(Json(QueueNackOut {}))
}

/// Purge all messages from a queue
#[aide_annotate(op_id = "v1.queue.purge")]
async fn queue_purge(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueuePurgeIn>,
) -> Result<Json<QueuePurgeOut>> {
    let name = data.name.to_string();

    let purged_count = queue_store.purge(&name)?;

    Ok(Json(QueuePurgeOut { purged_count }))
}

/// Get queue statistics
#[aide_annotate(op_id = "v1.queue.stats")]
async fn queue_stats(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueStatsIn>,
) -> Result<Json<QueueStatsOut>> {
    let name = data.name.to_string();

    let stats = queue_store.stats(&name)?;

    Ok(Json(QueueStatsOut {
        available: stats.available,
        in_flight: stats.in_flight,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Queue");

    ApiRouter::new()
        .api_route_with(
            "/queue/send",
            post_with(queue_send, queue_send_operation),
            &tag,
        )
        .api_route_with(
            "/queue/receive",
            post_with(queue_receive, queue_receive_operation),
            &tag,
        )
        .api_route_with(
            "/queue/ack",
            post_with(queue_ack, queue_ack_operation),
            &tag,
        )
        .api_route_with(
            "/queue/nack",
            post_with(queue_nack, queue_nack_operation),
            &tag,
        )
        .api_route_with(
            "/queue/purge",
            post_with(queue_purge, queue_purge_operation),
            &tag,
        )
        .api_route_with(
            "/queue/stats",
            post_with(queue_stats, queue_stats_operation),
            &tag,
        )
}
