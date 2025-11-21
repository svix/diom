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
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    AppState, core::types::EntityKey, v1::utils::{ValidatedJson, openapi_tag},
    error::{Result, Error, HttpError},
};

#[derive(Clone)]
pub struct LockStore {
    locks: Arc<DashMap<String, LockEntry>>,
    semaphores: Arc<DashMap<String, SemaphoreEntry>>,
}

impl LockStore {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(DashMap::new()),
            semaphores: Arc::new(DashMap::new()),
        }
    }
}

#[derive(Clone, Debug)]
struct LockEntry {
    expires_at: u64,
}

#[derive(Clone, Debug)]
struct SemaphoreEntry {
    max_permits: u32,
    current_permits: u32,
    holders: Vec<SemaphoreHolder>,
}

#[derive(Clone, Debug)]
struct SemaphoreHolder {
    expires_at: u64,
}

// ============================================================================
// Lock Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct LockAcquireIn {
    #[validate]
    pub key: EntityKey,

    /// Time of expiry (Unix timestamp in seconds)
    pub expires_at: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LockAcquireOut {
    /// Whether the lock was successfully acquired
    pub acquired: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct LockReleaseIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LockReleaseOut {
    /// Whether the lock was successfully released
    pub released: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct LockStatusIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LockStatusOut {
    /// Whether the lock is currently held
    pub locked: bool,

    /// Time when lock expires (if locked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
}

// ============================================================================
// Semaphore Types
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct SemaphoreCreateIn {
    #[validate]
    pub key: EntityKey,

    /// Maximum number of permits
    pub max_permits: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SemaphoreCreateOut {
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct SemaphoreAcquireIn {
    #[validate]
    pub key: EntityKey,

    /// Time of expiry (Unix timestamp in seconds)
    pub expires_at: u64,

    /// Number of permits to acquire (default: 1)
    #[serde(default = "default_permits")]
    pub permits: u32,
}

fn default_permits() -> u32 {
    1
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SemaphoreAcquireOut {
    /// Whether the permits were successfully acquired
    pub acquired: bool,

    /// Current available permits
    pub available: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct SemaphoreReleaseIn {
    #[validate]
    pub key: EntityKey,

    /// Number of permits to release (default: 1)
    #[serde(default = "default_permits")]
    pub permits: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SemaphoreReleaseOut {
    /// Whether the permits were successfully released
    pub released: bool,

    /// Current available permits
    pub available: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct SemaphoreStatusIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SemaphoreStatusOut {
    /// Maximum number of permits
    pub max_permits: u32,

    /// Currently available permits
    pub available: u32,

    /// Number of current holders
    pub holders: u32,
}

// ============================================================================
// Lock Implementation
// ============================================================================

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Lock Acquire
#[aide_annotate(op_id = "v1.lock.acquire")]
async fn lock_acquire(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<LockAcquireIn>,
) -> Result<Json<LockAcquireOut>> {
    let key_str = data.key.to_string();
    let now = current_timestamp();

    use dashmap::mapref::entry::Entry;

    match lock_store.locks.entry(key_str) {
        Entry::Vacant(entry) => {
            // Lock is available, acquire it
            entry.insert(LockEntry {
                expires_at: data.expires_at,
            });
            Ok(Json(LockAcquireOut { acquired: true }))
        }
        Entry::Occupied(mut entry) => {
            // Check if lock has expired
            if entry.get().expires_at <= now {
                // Lock expired, can be acquired
                *entry.get_mut() = LockEntry {
                    expires_at: data.expires_at,
                };
                Ok(Json(LockAcquireOut { acquired: true }))
            } else {
                // Lock is held
                Ok(Json(LockAcquireOut { acquired: false }))
            }
        }
    }
}

/// Lock Release
#[aide_annotate(op_id = "v1.lock.release")]
async fn lock_release(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<LockReleaseIn>,
) -> Result<Json<LockReleaseOut>> {
    let key_str = data.key.to_string();
    let deleted = lock_store.locks.remove(&key_str).is_some();
    Ok(Json(LockReleaseOut { released: deleted }))
}

/// Lock Status
#[aide_annotate(op_id = "v1.lock.status")]
async fn lock_status(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<LockStatusIn>,
) -> Result<Json<LockStatusOut>> {
    let key_str = data.key.to_string();
    let now = current_timestamp();

    if let Some(entry) = lock_store.locks.get(&key_str) {
        if entry.expires_at > now {
            Ok(Json(LockStatusOut {
                locked: true,
                expires_at: Some(entry.expires_at),
            }))
        } else {
            // Lock expired, remove it
            drop(entry);
            lock_store.locks.remove(&key_str);
            Ok(Json(LockStatusOut {
                locked: false,
                expires_at: None,
            }))
        }
    } else {
        Ok(Json(LockStatusOut {
            locked: false,
            expires_at: None,
        }))
    }
}

// ============================================================================
// Semaphore Implementation
// ============================================================================

/// Semaphore Create
#[aide_annotate(op_id = "v1.semaphore.create")]
async fn semaphore_create(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<SemaphoreCreateIn>,
) -> Result<Json<SemaphoreCreateOut>> {
    let key_str = data.key.to_string();

    use dashmap::mapref::entry::Entry;
    match lock_store.semaphores.entry(key_str) {
        Entry::Vacant(entry) => {
            entry.insert(SemaphoreEntry {
                max_permits: data.max_permits,
                current_permits: data.max_permits,
                holders: Vec::new(),
            });
            Ok(Json(SemaphoreCreateOut {}))
        }
        Entry::Occupied(_) => {
            Err(Error::http(HttpError::conflict(None, None)))
        }
    }
}

/// Semaphore Acquire
#[aide_annotate(op_id = "v1.semaphore.acquire")]
async fn semaphore_acquire(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<SemaphoreAcquireIn>,
) -> Result<Json<SemaphoreAcquireOut>> {
    let key_str = data.key.to_string();
    let now = current_timestamp();

    let mut entry = lock_store
        .semaphores
        .get_mut(&key_str)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    // Clean up expired holders
    entry.holders.retain(|h| h.expires_at > now);
    entry.current_permits = entry.max_permits - entry.holders.len() as u32;

    if entry.current_permits >= data.permits {
        // Add permits
        for _ in 0..data.permits {
            entry.holders.push(SemaphoreHolder {
                expires_at: data.expires_at,
            });
        }
        entry.current_permits -= data.permits;

        Ok(Json(SemaphoreAcquireOut {
            acquired: true,
            available: entry.current_permits,
        }))
    } else {
        Ok(Json(SemaphoreAcquireOut {
            acquired: false,
            available: entry.current_permits,
        }))
    }
}

/// Semaphore Release
#[aide_annotate(op_id = "v1.semaphore.release")]
async fn semaphore_release(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<SemaphoreReleaseIn>,
) -> Result<Json<SemaphoreReleaseOut>> {
    let key_str = data.key.to_string();

    let mut entry = lock_store
        .semaphores
        .get_mut(&key_str)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    // Release the specified number of permits (or just 1 by default)
    let to_release = data.permits.min(entry.holders.len() as u32);
    for _ in 0..to_release {
        entry.holders.pop();
    }

    entry.current_permits = entry.max_permits - entry.holders.len() as u32;

    Ok(Json(SemaphoreReleaseOut {
        released: to_release > 0,
        available: entry.current_permits,
    }))
}

/// Semaphore Status
#[aide_annotate(op_id = "v1.semaphore.status")]
async fn semaphore_status(
    State(AppState { lock_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<SemaphoreStatusIn>,
) -> Result<Json<SemaphoreStatusOut>> {
    let key_str = data.key.to_string();
    let now = current_timestamp();

    let mut entry = lock_store
        .semaphores
        .get_mut(&key_str)
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    // Clean up expired holders
    entry.holders.retain(|h| h.expires_at > now);
    entry.current_permits = entry.max_permits - entry.holders.len() as u32;

    Ok(Json(SemaphoreStatusOut {
        max_permits: entry.max_permits,
        available: entry.current_permits,
        holders: entry.holders.len() as u32,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Locks and Semaphores");

    ApiRouter::new()
        .api_route("/lock/acquire", post(lock_acquire))
        .api_route("/lock/release", post(lock_release))
        .api_route("/lock/status", post(lock_status))
        .api_route("/semaphore/create", post(semaphore_create))
        .api_route("/semaphore/acquire", post(semaphore_acquire))
        .api_route("/semaphore/release", post(semaphore_release))
        .api_route("/semaphore/status", post(semaphore_status))
}
