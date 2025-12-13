// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Idempotency module.
//!
//! This module implements idempotency, so people can use it to implement idempotency in their web
//! services.
//!
//! ## TODO FIXME
//! - Actually need to implement it. I guess it can use KV as its backend.
//! - The API probably needs changing.

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::sync::Arc;

use crate::{
    error::{Error, HttpError, Result},
    AppState,
};

/// Get current time in milliseconds since Unix epoch
fn now_millis() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

// ============================================================================
// Idempotency Store
// ============================================================================

#[derive(Clone)]
pub struct IdempotencyStore {
    store: Arc<DashMap<String, IdempotencyState>>,
}

#[derive(Clone, Debug)]
enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress { expires_at_millis: u64 },
    /// Request completed successfully with a response
    Completed {
        expires_at_millis: u64,
        // FIXME: Should be at least bytes. Though maybe we need to make it more generic like store
        // bytes or something?
        response: String,
    },
}

impl Default for IdempotencyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl IdempotencyStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    /// Atomically try to acquire the lock for a request.
    /// Returns:
    /// - Ok(None) if lock was acquired (request should proceed)
    /// - Ok(Some(response)) if request was already completed (return cached response)
    /// - Err if request is already in progress (conflict)
    pub fn try_start(&self, key: &str, ttl_seconds: u64) -> Result<Option<String>> {
        let now = now_millis();
        let expires_at_millis = now + (ttl_seconds * 1000);

        match self.store.entry(key.to_string()) {
            Entry::Vacant(entry) => {
                // No existing entry - acquire lock
                entry.insert(IdempotencyState::InProgress { expires_at_millis });
                Ok(None)
            }
            Entry::Occupied(entry) => {
                let state = entry.get();

                match state {
                    IdempotencyState::InProgress {
                        expires_at_millis: exp,
                    } => {
                        // Check if expired
                        if now >= *exp {
                            // Lock expired, replace with new lock
                            drop(entry);
                            self.store.insert(
                                key.to_string(),
                                IdempotencyState::InProgress { expires_at_millis },
                            );
                            Ok(None)
                        } else {
                            // Still in progress by another request
                            Err(Error::http(HttpError::conflict(
                                Some("Request is already in progress".into()),
                                None,
                            )))
                        }
                    }
                    IdempotencyState::Completed {
                        expires_at_millis: exp,
                        response,
                    } => {
                        // Check if expired
                        if now >= *exp {
                            // Response expired, acquire new lock
                            drop(entry);
                            self.store.insert(
                                key.to_string(),
                                IdempotencyState::InProgress { expires_at_millis },
                            );
                            Ok(None)
                        } else {
                            // Return cached response
                            Ok(Some(response.clone()))
                        }
                    }
                }
            }
        }
    }

    /// Complete a request with a successful response
    pub fn complete(&self, key: &str, response: String, ttl_seconds: u64) -> Result<()> {
        let now = now_millis();
        let expires_at_millis = now + (ttl_seconds * 1000);

        self.store.insert(
            key.to_string(),
            IdempotencyState::Completed {
                expires_at_millis,
                response,
            },
        );

        Ok(())
    }

    /// Abandon a request (remove the lock without saving response)
    pub fn abandon(&self, key: &str) -> Result<()> {
        self.store.remove(key);
        Ok(())
    }
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
