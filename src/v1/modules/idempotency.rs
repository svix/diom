// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Idempotency module.
//!
//! This module implements idempotency, so people can use it to implement idempotency in their web
//! services.
//!
//! ## TODO FIXME
//! - The API probably needs changing.

use std::time::Duration;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::error::{Error, HttpError, Result};
use coyote_kv::{KvModel, KvStore, OperationBehavior};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress,
    /// Request completed successfully with a response
    Completed { response: Vec<u8> },
}

impl From<IdempotencyState> for Vec<u8> {
    fn from(state: IdempotencyState) -> Self {
        rmp_serde::to_vec(&state).expect("Failed to serialize IdempotencyState")
    }
}

impl From<Vec<u8>> for IdempotencyState {
    fn from(value: Vec<u8>) -> Self {
        rmp_serde::from_slice(&value).expect("Failed to deserialize IdempotencyState")
    }
}

#[derive(Clone)]
pub struct IdempotencyStore {
    pub(crate) kv: KvStore,
}

impl IdempotencyStore {
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    /// Try to acquire the lock for a request.
    /// Returns:
    /// - Ok(None) if lock was acquired (request should proceed)
    /// - Ok(Some(response)) if request was already completed (return cached response)
    /// - Err if request is already in progress (conflict)
    pub fn try_start(&mut self, key: &str, ttl_seconds: u64) -> Result<Option<Vec<u8>>> {
        let now = Timestamp::now();
        let expires_at = now + Duration::from_secs(ttl_seconds);

        match self.kv.get(key)? {
            None => {
                // No existing entry - acquire lock
                let kv_model = KvModel {
                    value: IdempotencyState::InProgress.into(),
                    expires_at: Some(expires_at),
                };
                self.kv.set(key, &kv_model, OperationBehavior::Insert)?;
                Ok(None)
            }
            Some(kv_model) => {
                let state: IdempotencyState = kv_model.value.into();
                match state {
                    IdempotencyState::InProgress => {
                        // Still in progress by another request
                        Err(Error::http(HttpError::conflict(
                            Some("Request is already in progress".into()),
                            None,
                        )))
                    }
                    IdempotencyState::Completed { response } => {
                        // Return cached response
                        Ok(Some(response))
                    }
                }
            }
        }
    }

    /// Complete a request with a successful response
    pub fn complete(&mut self, key: &str, response: Vec<u8>, ttl_seconds: u64) -> Result<()> {
        let now = Timestamp::now();
        let expires_at = now + Duration::from_secs(ttl_seconds);

        let kv_model = KvModel {
            value: IdempotencyState::Completed { response }.into(),
            expires_at: Some(expires_at),
        };
        self.kv.set(key, &kv_model, OperationBehavior::Upsert)?;

        Ok(())
    }

    /// Abandon a request (remove the lock without saving response)
    pub fn abandon(&mut self, key: &str) -> Result<()> {
        self.kv.delete(key)
    }
}

#[cfg(test)]
mod tests {
    use fjall::Database;
    use test_utils::TestResult;

    use super::*;

    struct SetupFixture {
        store: IdempotencyStore,
    }

    impl SetupFixture {
        fn new() -> TestResult<Self> {
            let workdir = tempfile::tempdir()?;
            let db = Database::builder(workdir.as_ref()).temporary(true).open()?;
            let kv = KvStore::new("test", db, coyote_kv::EvictionPolicy::NoEviction);
            let store = IdempotencyStore::new(kv);
            Ok(Self { store })
        }
    }

    #[test]
    fn test_idempotency_state_serialization() {
        let state = IdempotencyState::InProgress;
        let serialized: Vec<u8> = state.clone().into();
        let deserialized: IdempotencyState = serialized.into();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_idempotency_state_serialization_completed() {
        let state = IdempotencyState::Completed {
            response: vec![1, 2, 3],
        };
        let serialized: Vec<u8> = state.clone().into();
        let deserialized: IdempotencyState = serialized.into();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_idempotency_try_start() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];
        let result = store.try_start("test", 10)?;
        assert_eq!(result, None);

        let result = store.try_start("test", 10);
        assert!(result.is_err());

        store.abandon("test")?;

        let result = store.try_start("test", 10)?;
        assert_eq!(result, None);

        store.complete("test", value.clone(), 10)?;

        let result = store.try_start("test", 10)?;
        assert_eq!(result, Some(value));
        Ok(())
    }

    #[test]
    fn test_idempotency_missing() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];

        // Can complete a request without starting it first
        store.complete("test", value.clone(), 10)?;

        let result = store.try_start("test", 10)?;
        assert_eq!(result, Some(value));

        // Can abandon a request without starting it first
        store.abandon("test2")?;
        let result2 = store.try_start("test2", 10)?;
        assert_eq!(result2, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_idempotency_ttl() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];

        store.try_start("test", 1)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1)?;
        assert_eq!(result, None);

        store.complete("test", value, 1)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1)?;
        assert_eq!(result, None);

        Ok(())
    }
}
