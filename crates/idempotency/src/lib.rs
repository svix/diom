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

use coyote_error::{Error, HttpError, Result};
use coyote_kv::{KvModel, KvStore, OperationBehavior};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

pub mod operations;

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
    kv: KvStore,
}

impl IdempotencyStore {
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    /// Try to acquire the lock for a request with a pre-computed timestamp.
    /// Returns:
    /// - Ok(None) if lock was acquired (request should proceed)
    /// - Ok(Some(response)) if request was already completed (return cached response)
    /// - Err if request is already in progress (conflict)
    pub fn try_start(
        &mut self,
        key: &str,
        ttl_seconds: u64,
        now: Timestamp,
    ) -> Result<Option<Vec<u8>>> {
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

    /// Complete a request with a successful response using a pre-computed timestamp
    pub fn complete(
        &mut self,
        key: &str,
        response: Vec<u8>,
        ttl_seconds: u64,
        now: Timestamp,
    ) -> Result<()> {
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

    pub fn start_operation(key: String, ttl_seconds: u64) -> operations::StartOperation {
        let now = Timestamp::now();
        operations::StartOperation::new(key, ttl_seconds, now)
    }

    pub fn complete_operation(
        key: String,
        response: Vec<u8>,
        ttl_seconds: u64,
    ) -> operations::CompleteOperation {
        let now = Timestamp::now();
        operations::CompleteOperation::new(key, response, ttl_seconds, now)
    }

    pub fn abandon_operation(key: String) -> operations::AbandonOperation {
        operations::AbandonOperation::new(key)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use coyote_configgroup::entities::EvictionPolicy;
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
            let kv = KvStore::new("test", db, EvictionPolicy::NoEviction, None);
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
        let now = Timestamp::now();
        let result = store.try_start("test", 10, now)?;
        assert_eq!(result, None);

        let result = store.try_start("test", 10, now);
        assert!(result.is_err());

        store.abandon("test")?;

        let result = store.try_start("test", 10, now)?;
        assert_eq!(result, None);

        store.complete("test", value.clone(), 10, now)?;

        let result = store.try_start("test", 10, now)?;
        assert_eq!(result, Some(value));
        Ok(())
    }

    #[test]
    fn test_idempotency_missing() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];
        let now = Timestamp::now();

        // Can complete a request without starting it first
        store.complete("test", value.clone(), 10, now)?;

        let result = store.try_start("test", 10, now)?;
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

        store.try_start("test", 1, now)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1, now)?;
        assert_eq!(result, None);

        store.complete("test", value, 1, now)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1, now)?;
        assert_eq!(result, None);

        Ok(())
    }
}
