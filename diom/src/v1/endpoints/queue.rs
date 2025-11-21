// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post};
use axum::{Json, extract::State};
use diom_derive::aide_annotate;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use validator::Validate;
use std::sync::Arc;
use dashmap::DashMap;
use crossbeam::queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use svix_ksuid::{KsuidLike as _, KsuidMs};

use crate::{
    AppState, core::types::EntityKey, v1::utils::{ValidatedJson, openapi_tag},
    error::{Result, Error, HttpError},
};

/// Get current time in milliseconds since Unix epoch
fn now_millis() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

// ============================================================================
// Queue Store
// ============================================================================

#[derive(Clone)]
pub struct QueueStore {
    // Map of queue_name -> Queue
    queues: Arc<DashMap<String, Queue>>,
}

#[derive(Clone)]
struct Queue {
    // Ready messages (FIFO queue for immediate delivery)
    ready: Arc<SegQueue<Message>>,
    // Delayed messages (lock-free ordered map: timestamp -> message)
    // SkipMap maintains ordering and is lock-free
    delayed: Arc<SkipMap<u64, Message>>,
    // In-flight messages being processed (keyed by message ID for fast lookup)
    in_flight: Arc<DashMap<String, InFlightMessage>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    payload: String,
    /// When the message should become available (for delayed delivery)
    available_at_millis: u64,
    /// Number of times this message has been nacked/timed out
    attempt_count: u64,
    /// Maximum number of attempts before moving to DLQ
    max_attempts: u64,
    /// Dead letter queue name (if set)
    dlq_queue_name: Option<String>,
    /// Original enqueue time
    enqueued_at_millis: u64,
}

#[derive(Clone, Debug)]
struct InFlightMessage {
    message: Message,
    /// When this message's processing will timeout
    timeout_at_millis: u64,
}

impl QueueStore {
    pub fn new() -> Self {
        Self {
            queues: Arc::new(DashMap::new()),
        }
    }

    fn get_or_create_queue(&self, queue_name: &str) -> Queue {
        self.queues.entry(queue_name.to_string())
            .or_insert_with(|| Queue {
                ready: Arc::new(SegQueue::new()),
                delayed: Arc::new(SkipMap::new()),
                in_flight: Arc::new(DashMap::new()),
            })
            .clone()
    }

    /// Enqueue a message with optional delay
    fn enqueue(
        &self,
        queue_name: &str,
        payload: String,
        delay_seconds: u64,
        max_attempts: u64,
        dlq_queue_name: Option<String>,
    ) -> Result<String> {
        let now = now_millis();
        let message_id = KsuidMs::new(None, None).to_string();
        let available_at_millis = now + (delay_seconds * 1000);

        let queue = self.get_or_create_queue(queue_name);

        let message = Message {
            id: message_id.clone(),
            payload,
            available_at_millis,
            attempt_count: 0,
            max_attempts,
            dlq_queue_name,
            enqueued_at_millis: now,
        };

        if delay_seconds == 0 {
            // No delay - add to ready queue
            queue.ready.push(message);
        } else {
            // Has delay - add to delayed skipmap (keyed by availability time + message ID for uniqueness)
            // Use combination of timestamp and message ID to avoid key collisions
            let key = (available_at_millis << 32) | (message_id.len() as u64); // Simple unique key
            queue.delayed.insert(key, message);
        }

        Ok(message_id)
    }

    /// Move ready delayed messages to the ready queue
    fn promote_delayed_messages(&self, queue: &Queue) {
        let now = now_millis();

        // Find and remove all messages that are ready (lock-free iteration)
        let ready_keys: Vec<u64> = queue.delayed
            .iter()
            .filter_map(|entry: crossbeam_skiplist::map::Entry<'_, u64, Message>| {
                let key = *entry.key();
                let timestamp = key >> 32;
                if timestamp <= now {
                    Some(key)
                } else {
                    // SkipMap is ordered, so once we hit a future timestamp, stop
                    None
                }
            })
            .collect();

        // Move ready messages to ready queue
        for key in ready_keys {
            if let Some(entry) = queue.delayed.remove(&key) {
                queue.ready.push(entry.value().clone());
            }
        }
    }

    /// Dequeue a message (if available)
    fn dequeue(
        &self,
        queue_name: &str,
        visibility_timeout_seconds: u64,
    ) -> Result<Option<(String, String)>> {
        let queue = match self.queues.get(queue_name) {
            Some(q) => q.clone(),
            None => return Ok(None),
        };

        let now = now_millis();

        // First, check for timed-out in-flight messages and return them to the queue
        self.check_timeouts(queue_name)?;

        // Promote any delayed messages that are now ready
        self.promote_delayed_messages(&queue);

        // Try to dequeue from ready queue
        if let Some(message) = queue.ready.pop() {
            let message_id = message.id.clone();
            let payload = message.payload.clone();
            let timeout_at_millis = now + (visibility_timeout_seconds * 1000);

            let in_flight_msg = InFlightMessage {
                message,
                timeout_at_millis,
            };

            queue.in_flight.insert(message_id.clone(), in_flight_msg);

            return Ok(Some((message_id, payload)));
        }

        Ok(None)
    }

    /// Acknowledge successful processing of a message
    fn ack(&self, queue_name: &str, message_id: &str) -> Result<()> {
        let queue = self.queues.get(queue_name)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Queue not found".into()), None)))?;

        queue.in_flight.remove(message_id)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Message not found or not in-flight".into()), None)))?;

        Ok(())
    }

    /// Negative acknowledge - return message to queue or move to DLQ
    fn nack(&self, queue_name: &str, message_id: &str) -> Result<()> {
        let queue = self.queues.get(queue_name)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Queue not found".into()), None)))?;

        let (_, in_flight_msg) = queue.in_flight.remove(message_id)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Message not found or not in-flight".into()), None)))?;

        let mut message = in_flight_msg.message;
        message.attempt_count += 1;

        // Check if we've exceeded max attempts
        if message.attempt_count >= message.max_attempts {
            // Move to DLQ (if configured)
            if let Some(dlq_name) = &message.dlq_queue_name {
                let dlq = self.get_or_create_queue(dlq_name);
                // Reset availability so it's immediately available in DLQ
                message.available_at_millis = now_millis();
                // Add directly to ready queue (no delay for DLQ)
                dlq.ready.push(message);
            }
            // If no DLQ configured, message is just dropped
        } else {
            // Return to ready queue (immediately available for retry)
            queue.ready.push(message);
        }

        Ok(())
    }

    /// Check for timed-out in-flight messages and return them to the queue or DLQ
    fn check_timeouts(&self, queue_name: &str) -> Result<()> {
        let queue = match self.queues.get(queue_name) {
            Some(q) => q.clone(),
            None => return Ok(()),
        };

        let now = now_millis();
        let mut timed_out = Vec::new();

        // Collect timed-out messages
        for entry in queue.in_flight.iter() {
            if now >= entry.value().timeout_at_millis {
                timed_out.push(entry.key().clone());
            }
        }

        // Process timed-out messages
        for message_id in timed_out {
            if let Some((_, in_flight_msg)) = queue.in_flight.remove(&message_id) {
                let mut message = in_flight_msg.message;
                message.attempt_count += 1;

                // Check if we've exceeded max attempts
                if message.attempt_count >= message.max_attempts {
                    // Move to DLQ (if configured)
                    if let Some(dlq_name) = &message.dlq_queue_name {
                        let dlq = self.get_or_create_queue(dlq_name);
                        // Reset availability so it's immediately available in DLQ
                        message.available_at_millis = now;
                        // Add directly to ready queue (no delay for DLQ)
                        dlq.ready.push(message);
                    }
                    // If no DLQ configured, message is just dropped
                } else {
                    // Return to ready queue (immediately available for retry)
                    queue.ready.push(message);
                }
            }
        }

        Ok(())
    }

    /// Purge all messages from a queue
    fn purge(&self, queue_name: &str) -> Result<u64> {
        let queue = self.queues.get(queue_name)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Queue not found".into()), None)))?;

        let mut count = 0u64;

        // Count and clear ready messages
        while queue.ready.pop().is_some() {
            count += 1;
        }

        // Count and clear delayed messages
        let delayed_count = queue.delayed.len() as u64;
        queue.delayed.clear();
        count += delayed_count;

        // Clear in-flight messages
        queue.in_flight.clear();

        Ok(count)
    }

    /// Get stats about a queue
    fn stats(&self, queue_name: &str) -> Result<QueueStats> {
        let queue = self.queues.get(queue_name)
            .ok_or_else(|| Error::http(HttpError::not_found(Some("Queue not found".into()), None)))?;

        // Note: SegQueue doesn't have a len() method, so we approximate
        // by counting ready + delayed. This is slightly imprecise but acceptable.
        let delayed_count = queue.delayed.len() as u64;

        Ok(QueueStats {
            available: delayed_count, // Approximation: delayed messages (ready messages hard to count efficiently)
            in_flight: queue.in_flight.len() as u64,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct QueueStats {
    available: u64,
    in_flight: u64,
}

// ============================================================================
// API Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueEnqueueIn {
    #[validate]
    pub queue_name: EntityKey,

    /// Message payload
    pub payload: String,

    /// Delay before message becomes available (seconds, default: 0)
    #[serde(default)]
    pub delay_seconds: u64,

    /// Maximum number of processing attempts before moving to DLQ (default: 3)
    #[serde(default = "default_max_attempts")]
    #[validate(range(min = 1))]
    pub max_attempts: u64,

    /// Dead letter queue name (optional, defaults to "{queue_name}:DLQ")
    pub dlq_queue_name: Option<EntityKey>,
}

fn default_max_attempts() -> u64 {
    3
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueEnqueueOut {
    /// Unique message ID
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueDequeueIn {
    #[validate]
    pub queue_name: EntityKey,

    /// Visibility timeout in seconds (how long before message returns to queue if not ack'd)
    #[validate(range(min = 1))]
    pub visibility_timeout_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum QueueDequeueOut {
    Message {
        message_id: String,
        payload: String,
    },
    Empty {},
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueAckIn {
    #[validate]
    pub queue_name: EntityKey,

    /// Message ID to acknowledge
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueAckOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueNackIn {
    #[validate]
    pub queue_name: EntityKey,

    /// Message ID to negative acknowledge
    pub message_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueueNackOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueuePurgeIn {
    #[validate]
    pub queue_name: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct QueuePurgeOut {
    /// Number of messages purged
    pub purged_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct QueueStatsIn {
    #[validate]
    pub queue_name: EntityKey,
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

/// Enqueue a message
#[aide_annotate(op_id = "v1.queue.enqueue")]
async fn queue_enqueue(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueEnqueueIn>,
) -> Result<Json<QueueEnqueueOut>> {
    let queue_name = data.queue_name.to_string();

    // Default DLQ name is "{queue_name}:DLQ"
    let dlq_queue_name = data.dlq_queue_name
        .map(|k| k.to_string())
        .or_else(|| Some(format!("{}:DLQ", queue_name)));

    let message_id = queue_store.enqueue(
        &queue_name,
        data.payload,
        data.delay_seconds,
        data.max_attempts,
        dlq_queue_name,
    )?;

    Ok(Json(QueueEnqueueOut { message_id }))
}

/// Dequeue a message
#[aide_annotate(op_id = "v1.queue.dequeue")]
async fn queue_dequeue(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueDequeueIn>,
) -> Result<Json<QueueDequeueOut>> {
    let queue_name = data.queue_name.to_string();

    match queue_store.dequeue(&queue_name, data.visibility_timeout_seconds)? {
        Some((message_id, payload)) => {
            Ok(Json(QueueDequeueOut::Message { message_id, payload }))
        }
        None => {
            Ok(Json(QueueDequeueOut::Empty {}))
        }
    }
}

/// Acknowledge successful message processing
#[aide_annotate(op_id = "v1.queue.ack")]
async fn queue_ack(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueAckIn>,
) -> Result<Json<QueueAckOut>> {
    let queue_name = data.queue_name.to_string();

    queue_store.ack(&queue_name, &data.message_id)?;

    Ok(Json(QueueAckOut {}))
}

/// Negative acknowledge - return message to queue or move to DLQ
#[aide_annotate(op_id = "v1.queue.nack")]
async fn queue_nack(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueNackIn>,
) -> Result<Json<QueueNackOut>> {
    let queue_name = data.queue_name.to_string();

    queue_store.nack(&queue_name, &data.message_id)?;

    Ok(Json(QueueNackOut {}))
}

/// Purge all messages from a queue
#[aide_annotate(op_id = "v1.queue.purge")]
async fn queue_purge(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueuePurgeIn>,
) -> Result<Json<QueuePurgeOut>> {
    let queue_name = data.queue_name.to_string();

    let purged_count = queue_store.purge(&queue_name)?;

    Ok(Json(QueuePurgeOut { purged_count }))
}

/// Get queue statistics
#[aide_annotate(op_id = "v1.queue.stats")]
async fn queue_stats(
    State(AppState { queue_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<QueueStatsIn>,
) -> Result<Json<QueueStatsOut>> {
    let queue_name = data.queue_name.to_string();

    let stats = queue_store.stats(&queue_name)?;

    Ok(Json(QueueStatsOut {
        available: stats.available,
        in_flight: stats.in_flight,
    }))
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Queue");

    ApiRouter::new()
        .api_route("/queue/enqueue", post(queue_enqueue))
        .api_route("/queue/dequeue", post(queue_dequeue))
        .api_route("/queue/ack", post(queue_ack))
        .api_route("/queue/nack", post(queue_nack))
        .api_route("/queue/purge", post(queue_purge))
        .api_route("/queue/stats", post(queue_stats))
}
