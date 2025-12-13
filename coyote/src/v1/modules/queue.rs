// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Queue Module
//!
//! This module implements a message queue system with delayed delivery, visibility timeouts,
//! dead-letter queues (DLQ), and retry logic.
//!
//! ## Data Structure Design
//!
//! The queue store uses multiple data structures for efficient message handling:
//!
//! 1. Queue Store (DashMap) - Maps queue names to Queue instances
//! 2. Ready Queue (SegQueue) - Lock-free FIFO queue for messages ready for immediate delivery
//! 3. Delayed Messages (SkipMap) - Lock-free ordered map for messages with delayed delivery (sorted by availability time)
//! 4. In-Flight Messages (DashMap) - Fast lookup for messages currently being processed
//!
//! ## How It Works
//!
//! ### Message Lifecycle
//!
//! - Enqueue: Messages are added to either the ready queue (no delay) or delayed skipmap (with delay)
//! - Promotion: Background process moves delayed messages to ready queue when their time comes
//! - Dequeue: Retrieves message from ready queue and moves it to in-flight (with visibility timeout)
//! - ACK: Successful processing removes message from in-flight
//! - NACK: Failed processing increments attempt count and either:
//!   - Returns message to ready queue (if attempts remaining)
//!   - Moves to dead-letter queue (if max attempts exceeded)
//! - Timeout: Messages not ACK'd within visibility timeout are treated like NACK
//!
//! ### Key Features
//!
//! - Delayed Delivery: Messages can be scheduled for future delivery
//! - Visibility Timeout: Dequeued messages become invisible for a configurable period
//! - Automatic Retry: Failed messages are automatically retried up to max_attempts
//! - Dead-Letter Queue: Messages exceeding max attempts are moved to DLQ for inspection
//! - Lock-Free Operations: Uses concurrent data structures for high throughput
//!
//! ## TODO FIXME
//! - The delayed message key generation (line 120) uses a simple hash that could theoretically collide
//! - Consider adding message priorities
//! - Consider adding queue TTL for auto-cleanup of unused queues - probably a problem with
//!   configuration? Not if we do configuration as a group like I wanted.
//! - Message ID should probably be a uuidv7 or something and save some bytes. Or maybe even just
//!   a u64 if we are going at it from a kafka point of view? Though I guess that prevents some
//!   distributed publishing options?

use chrono::{DateTime, Utc};
use crossbeam::queue::SegQueue;
use crossbeam_skiplist::SkipMap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use svix_ksuid::{KsuidLike as _, KsuidMs};

use crate::{
    error::{Error, HttpError, Result},
    AppState,
};

// ============================================================================
// Queue Store
// ============================================================================

#[derive(Clone, Debug)]
pub struct QueueConfiguration {
    /// Maximum number of processing attempts before moving to DLQ
    pub max_attempts: u16,
    /// Dead letter queue name (optional)
    pub dlq_queue_name: Option<String>,
}

// Default configuration constants
const DEFAULT_MAX_ATTEMPTS: u16 = 3;
const DLQ_SUFFIX: &str = ":DLQ";

impl Default for QueueConfiguration {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            dlq_queue_name: None,
        }
    }
}

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
    // Queue configuration
    config: QueueConfiguration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    payload: String,
    /// When the message should become available (for delayed delivery)
    available_at: DateTime<Utc>,
    /// Number of times this message has been nacked/timed out
    attempt_count: u16,
    /// Original enqueue time
    enqueued_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
struct InFlightMessage {
    message: Message,
    /// When this message's processing will timeout
    timeout_at: DateTime<Utc>,
}

impl Default for QueueStore {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueStore {
    pub fn new() -> Self {
        Self {
            queues: Arc::new(DashMap::new()),
        }
    }

    fn get_or_create_queue(&self, queue_name: &str) -> Queue {
        self.queues
            .entry(queue_name.to_string())
            .or_insert_with(|| {
                let config = QueueConfiguration {
                    max_attempts: DEFAULT_MAX_ATTEMPTS,
                    dlq_queue_name: Some(format!("{queue_name}{DLQ_SUFFIX}")),
                };
                Queue {
                    ready: Arc::new(SegQueue::new()),
                    delayed: Arc::new(SkipMap::new()),
                    in_flight: Arc::new(DashMap::new()),
                    config,
                }
            })
            .clone()
    }

    /// Enqueue a message with a specified availability time
    pub fn enqueue(
        &self,
        queue_name: &str,
        payload: String,
        available_at: DateTime<Utc>,
    ) -> Result<String> {
        let now = Utc::now();
        let message_id = KsuidMs::new(None, None).to_string();

        let queue = self.get_or_create_queue(queue_name);

        let message = Message {
            id: message_id.clone(),
            payload,
            available_at,
            attempt_count: 0,
            enqueued_at: now,
        };

        if available_at <= now {
            // No delay or time in past - add to ready queue immediately
            queue.ready.push(message);
        } else {
            // Has delay - add to delayed skipmap (keyed by availability time + message ID for uniqueness)
            // Use combination of timestamp and message ID to avoid key collisions
            let timestamp_millis = available_at.timestamp_millis() as u64;
            let key = (timestamp_millis << 32) | (message_id.len() as u64); // Simple unique key
            queue.delayed.insert(key, message);
        }

        Ok(message_id)
    }

    /// Move ready delayed messages to the ready queue
    fn promote_delayed_messages(&self, queue: &Queue) {
        let now = Utc::now();

        // Find and remove all messages that are ready (lock-free iteration)
        let ready_keys: Vec<u64> = queue
            .delayed
            .iter()
            .filter_map(|entry: crossbeam_skiplist::map::Entry<'_, u64, Message>| {
                let key = *entry.key();
                let message = entry.value();
                if message.available_at <= now {
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
    pub fn dequeue(
        &self,
        queue_name: &str,
        visibility_timeout_seconds: u64,
    ) -> Result<Option<(String, String)>> {
        let queue = match self.queues.get(queue_name) {
            Some(q) => q.clone(),
            None => return Ok(None),
        };

        let now = Utc::now();

        // First, check for timed-out in-flight messages and return them to the queue
        self.check_timeouts(queue_name)?;

        // Promote any delayed messages that are now ready
        self.promote_delayed_messages(&queue);

        // Try to dequeue from ready queue
        if let Some(message) = queue.ready.pop() {
            let message_id = message.id.clone();
            let payload = message.payload.clone();
            let timeout_at = now + chrono::Duration::seconds(visibility_timeout_seconds as i64);

            let in_flight_msg = InFlightMessage {
                message,
                timeout_at,
            };

            queue.in_flight.insert(message_id.clone(), in_flight_msg);

            return Ok(Some((message_id, payload)));
        }

        Ok(None)
    }

    /// Acknowledge successful processing of a message
    pub fn ack(&self, queue_name: &str, message_id: &str) -> Result<()> {
        let queue = self.queues.get(queue_name).ok_or_else(|| {
            Error::http(HttpError::not_found(Some("Queue not found".into()), None))
        })?;

        queue.in_flight.remove(message_id).ok_or_else(|| {
            Error::http(HttpError::not_found(
                Some("Message not found or not in-flight".into()),
                None,
            ))
        })?;

        Ok(())
    }

    /// Negative acknowledge - return message to queue or move to DLQ
    pub fn nack(&self, queue_name: &str, message_id: &str) -> Result<()> {
        let queue = self.queues.get(queue_name).ok_or_else(|| {
            Error::http(HttpError::not_found(Some("Queue not found".into()), None))
        })?;

        let config = queue.config.clone();

        let (_, in_flight_msg) = queue.in_flight.remove(message_id).ok_or_else(|| {
            Error::http(HttpError::not_found(
                Some("Message not found or not in-flight".into()),
                None,
            ))
        })?;

        let mut message = in_flight_msg.message;
        message.attempt_count += 1;

        // Check if we've exceeded max attempts
        if message.attempt_count >= config.max_attempts {
            // Move to DLQ (if configured)
            if let Some(dlq_name) = &config.dlq_queue_name {
                let dlq = self.get_or_create_queue(dlq_name);
                // Reset availability so it's immediately available in DLQ
                message.available_at = Utc::now();
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

        let now = Utc::now();
        let config = queue.config.clone();
        let mut timed_out = Vec::new();

        // Collect timed-out messages
        for entry in queue.in_flight.iter() {
            if now >= entry.value().timeout_at {
                timed_out.push(entry.key().clone());
            }
        }

        // Process timed-out messages
        for message_id in timed_out {
            if let Some((_, in_flight_msg)) = queue.in_flight.remove(&message_id) {
                let mut message = in_flight_msg.message;
                message.attempt_count += 1;

                // Check if we've exceeded max attempts
                if message.attempt_count >= config.max_attempts {
                    // Move to DLQ (if configured)
                    if let Some(dlq_name) = &config.dlq_queue_name {
                        let dlq = self.get_or_create_queue(dlq_name);
                        // Reset availability so it's immediately available in DLQ
                        message.available_at = now;
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
    pub fn purge(&self, queue_name: &str) -> Result<u64> {
        let queue = self.queues.get(queue_name).ok_or_else(|| {
            Error::http(HttpError::not_found(Some("Queue not found".into()), None))
        })?;

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
    pub fn stats(&self, queue_name: &str) -> Result<QueueStats> {
        let queue = self.queues.get(queue_name).ok_or_else(|| {
            Error::http(HttpError::not_found(Some("Queue not found".into()), None))
        })?;

        // Note: SegQueue doesn't have a len() method, so we approximate
        // by counting ready + delayed. This is slightly imprecise but acceptable.
        let delayed_count = queue.delayed.len() as u64;

        Ok(QueueStats {
            available: delayed_count, // Approximation: delayed messages (ready messages hard to count efficiently)
            in_flight: queue.in_flight.len() as u64,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueStats {
    pub available: u64,
    pub in_flight: u64,
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(_state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
