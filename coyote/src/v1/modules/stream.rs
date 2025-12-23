// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Stream Module
//!
//! This module implements a simple log-based stream system similar to Apache Kafka.
//!
//! ## Data Structure Design
//!
//! The stream store uses a simple append-only log per topic:
//!
//! 1. Stream Store (DashMap) - Maps topic names to Topic instances
//! 2. Topic - Contains an append-only vector of messages with sequential offsets
//!
//! ## How It Works
//!
//! ### Message Lifecycle
//!
//! - Publish: Messages are appended to the end of a topic's log and assigned sequential offsets
//! - Read: Consumers read messages starting from a specific offset
//! - No deletion: Messages remain in the log (append-only)
//! - Offset tracking: Consumers are responsible for tracking their read position
//!
//! ### Key Features
//!
//! - Append-Only Log: Messages are never modified or deleted after publishing
//! - Sequential Offsets: Each message gets a monotonically increasing offset
//! - Multiple Readers: Many consumers can independently read from the same topic
//! - Offset-Based Reading: Consumers specify where to start reading
//!
//! ## TODO FIXME
//! - Consider adding consumer groups for distributed processing
//! - Consider adding offset commit tracking

use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use svix_ksuid::{KsuidLike as _, KsuidMs};

use crate::{
    error::{Error, HttpError, Result},
    AppState,
};

// ============================================================================
// Stream Store
// ============================================================================

#[derive(Clone)]
pub struct StreamStore {
    // Map of topic_name -> Topic
    topics: Arc<DashMap<String, Topic>>,
}

#[derive(Clone)]
struct Topic {
    // Append-only log of messages
    messages: Arc<RwLock<Vec<Message>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    /// Unique message ID
    id: String,
    /// Sequential offset within the topic (0-indexed)
    offset: u64,
    /// Message payload
    payload: String,
    /// When the message was published
    published_at: DateTime<Utc>,
}

impl Default for StreamStore {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamStore {
    pub fn new() -> Self {
        Self {
            topics: Arc::new(DashMap::new()),
        }
    }

    fn get_or_create_topic(&self, topic_name: &str) -> Topic {
        self.topics
            .entry(topic_name.to_string())
            .or_insert_with(|| Topic {
                messages: Arc::new(RwLock::new(Vec::new())),
            })
            .clone()
    }

    /// Publish one or more messages to a topic
    pub fn publish(&self, topic_name: &str, payloads: Vec<String>) -> Result<()> {
        if payloads.is_empty() {
            return Err(Error::http(HttpError::bad_request(
                Some("Cannot publish empty message list".into()),
                None,
            )));
        }

        let topic = self.get_or_create_topic(topic_name);
        let mut messages = topic.messages.write().unwrap();

        let now = Utc::now();
        let starting_offset = messages.len() as u64;

        for (i, payload) in payloads.into_iter().enumerate() {
            let message = Message {
                id: KsuidMs::new(None, None).to_string(),
                offset: starting_offset + i as u64,
                payload,
                published_at: now,
            };
            messages.push(message);
        }

        Ok(())
    }

    /// Read messages from a topic starting at a specific offset
    /// Returns messages and whether there are more messages available
    pub fn read(
        &self,
        topic_name: &str,
        start_offset: u64,
        limit: u32,
    ) -> Result<(Vec<StreamMessage>, bool)> {
        let topic = match self.topics.get(topic_name) {
            Some(t) => t.clone(),
            None => {
                // Topic doesn't exist, return empty result
                return Ok((Vec::new(), false));
            }
        };

        let messages = topic.messages.read().unwrap();
        let total_messages = messages.len() as u64;

        // Check if start_offset is beyond available messages
        if start_offset >= total_messages {
            return Ok((Vec::new(), false));
        }

        let start_idx = start_offset as usize;
        let end_idx = std::cmp::min(start_idx + limit as usize, messages.len());

        let result: Vec<StreamMessage> = messages[start_idx..end_idx]
            .iter()
            .map(|msg| StreamMessage {
                id: msg.id.clone(),
                offset: msg.offset,
                payload: msg.payload.clone(),
                published_at: msg.published_at,
            })
            .collect();

        let has_more = end_idx < messages.len();

        Ok((result, has_more))
    }

    /// Get information about a topic
    pub fn topic_info(&self, topic_name: &str) -> Result<TopicInfo> {
        let topic = match self.topics.get(topic_name) {
            Some(t) => t.clone(),
            None => {
                return Err(Error::http(HttpError::not_found(
                    Some("Topic not found".into()),
                    None,
                )))
            }
        };

        let messages = topic.messages.read().unwrap();
        let message_count = messages.len() as u64;

        Ok(TopicInfo {
            message_count,
            earliest_offset: if message_count > 0 { 0 } else { 0 },
            latest_offset: if message_count > 0 {
                message_count - 1
            } else {
                0
            },
        })
    }

    /// Purge all messages from a topic
    pub fn purge(&self, topic_name: &str) -> Result<u64> {
        let topic = self.topics.get(topic_name).ok_or_else(|| {
            Error::http(HttpError::not_found(Some("Topic not found".into()), None))
        })?;

        let mut messages = topic.messages.write().unwrap();
        let count = messages.len() as u64;
        messages.clear();

        Ok(count)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamMessage {
    pub id: String,
    pub offset: u64,
    pub payload: String,
    pub published_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicInfo {
    pub message_count: u64,
    pub earliest_offset: u64,
    pub latest_offset: u64,
}

/// This is the worker function for this module, currently a no-op
pub async fn worker(_state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
