// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use coyote_derive::aide_annotate;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::{Error, HttpError, Result},
    v1::utils::{openapi_tag, ValidatedJson},
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
    fn try_start(&self, key: &str, ttl_seconds: u64) -> Result<Option<String>> {
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
    fn complete(&self, key: &str, response: String, ttl_seconds: u64) -> Result<()> {
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
    fn abandon(&self, key: &str) -> Result<()> {
        self.store.remove(key);
        Ok(())
    }
}

// ============================================================================
// API Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyStartIn {
    #[validate]
    pub key: EntityKey,

    /// TTL in seconds for the lock/response
    #[validate(range(min = 1))]
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartOut {
    /// Lock acquired, request should proceed
    Locked,
    /// Request was already completed, cached response returned
    Completed { response: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyCompleteIn {
    #[validate]
    pub key: EntityKey,

    /// The response to cache
    pub response: String,

    /// TTL in seconds for the cached response
    #[validate(range(min = 1))]
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyCompleteOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct IdempotencyAbandonIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IdempotencyAbandonOut {}

// ============================================================================
// API Endpoints
// ============================================================================

/// Start an idempotent request
#[aide_annotate(op_id = "v1.idempotency.start")]
async fn idempotency_start(
    State(AppState {
        idempotency_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<IdempotencyStartIn>,
) -> Result<Json<IdempotencyStartOut>> {
    let key_str = data.key.to_string();

    match idempotency_store.try_start(&key_str, data.ttl_seconds)? {
        None => Ok(Json(IdempotencyStartOut::Locked)),
        Some(response) => Ok(Json(IdempotencyStartOut::Completed { response })),
    }
}

/// Complete an idempotent request with a response
#[aide_annotate(op_id = "v1.idempotency.complete")]
async fn idempotency_complete(
    State(AppState {
        idempotency_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<IdempotencyCompleteIn>,
) -> Result<Json<IdempotencyCompleteOut>> {
    let key_str = data.key.to_string();

    idempotency_store.complete(&key_str, data.response, data.ttl_seconds)?;

    Ok(Json(IdempotencyCompleteOut {}))
}

/// Abandon an idempotent request (remove lock without saving response)
#[aide_annotate(op_id = "v1.idempotency.abandon")]
async fn idempotency_abandon(
    State(AppState {
        idempotency_store, ..
    }): State<AppState>,
    ValidatedJson(data): ValidatedJson<IdempotencyAbandonIn>,
) -> Result<Json<IdempotencyAbandonOut>> {
    let key_str = data.key.to_string();

    idempotency_store.abandon(&key_str)?;

    Ok(Json(IdempotencyAbandonOut {}))
}

// ============================================================================
// Router
// ============================================================================

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

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Idempotency");

    ApiRouter::new()
        .api_route_with(
            "/idempotency/start",
            post_with(idempotency_start, idempotency_start_operation),
            &tag,
        )
        .api_route_with(
            "/idempotency/complete",
            post_with(idempotency_complete, idempotency_complete_operation),
            &tag,
        )
        .api_route_with(
            "/idempotency/abandon",
            post_with(idempotency_abandon, idempotency_abandon_operation),
            &tag,
        )
}
