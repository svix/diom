// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use diom_derive::aide_annotate;
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
#[validate(schema(function = "validate_delay_options"))]
pub struct QueueSendIn {
    #[validate]
    pub name: EntityKey,

    // FIXME: needs to be bytes.
    /// Array of message payloads to send
    pub messages: Vec<String>,

    // FIXME: maybe make it millis?
    /// Delay before messages become available (seconds). Mutually exclusive with scheduled_at.
    pub delay_seconds: Option<u64>,

    /// Specific time when messages should become available. Mutually exclusive with delay_seconds.
    pub scheduled_at: Option<DateTime<Utc>>,
}

fn validate_delay_options(data: &QueueSendIn) -> Result<(), validator::ValidationError> {
    // Ensure only one delay option is specified
    if data.delay_seconds.is_some() && data.scheduled_at.is_some() {
        return Err(validator::ValidationError::new(
            "Cannot specify both delay_seconds and scheduled_at",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueSendOut {
    /// Array of unique message IDs for the enqueued messages
    pub message_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueReceiveIn {
    #[validate]
    pub name: EntityKey,

    /// Visibility timeout in seconds (how long before message returns to queue if not ack'd)
    #[validate(range(min = 1))]
    pub visibility_timeout_seconds: u64,

    /// Maximum number of messages to receive (default: 1, max: 50)
    #[serde(default = "default_batch_size")]
    #[validate(range(min = 1, max = 50))]
    pub batch_size: u32,
}

fn default_batch_size() -> u32 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueMessage {
    pub message_id: String,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueReceiveOut {
    /// Array of received messages (empty if no messages available)
    pub messages: Vec<QueueMessage>,
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

/// Send messages to the queue
#[aide_annotate(op_id = "v1.queue.send")]
async fn queue_send(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueSendIn>,
) -> Result<Json<QueueSendOut>> {
    let name = data.name.to_string();

    // Calculate available_at once for all messages
    let available_at = match (data.delay_seconds, data.scheduled_at) {
        (Some(delay), None) => Utc::now() + chrono::Duration::seconds(delay as i64),
        (None, Some(scheduled)) => scheduled,
        (None, None) => Utc::now(), // Default: immediately available
        (Some(_), Some(_)) => unreachable!("validation should prevent this"),
    };

    let mut message_ids = Vec::new();

    // Enqueue each message with the same availability time
    for payload in data.messages {
        let message_id = queue_store.enqueue(&name, payload, available_at)?;
        message_ids.push(message_id);
    }

    Ok(Json(QueueSendOut { message_ids }))
}

/// Receive messages from the queue
#[aide_annotate(op_id = "v1.queue.receive")]
async fn queue_receive(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueReceiveIn>,
) -> Result<Json<QueueReceiveOut>> {
    let name = data.name.to_string();
    let mut messages = Vec::new();

    // Dequeue up to batch_size messages
    for _ in 0..data.batch_size {
        match queue_store.dequeue(&name, data.visibility_timeout_seconds)? {
            Some((message_id, payload)) => {
                messages.push(QueueMessage {
                    message_id,
                    payload,
                });
            }
            None => break, // No more messages available
        }
    }

    Ok(Json(QueueReceiveOut { messages }))
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
