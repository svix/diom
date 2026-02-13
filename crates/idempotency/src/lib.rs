// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Idempotency module.
//!
//! This module implements idempotency, so people can use it to implement idempotency in their web
//! services.
//!
//! ## TODO FIXME
//! - The API probably needs changing.

pub mod operations;

use std::time::Duration;

use diom_error::Result;
use diom_kv::{KvModel, KvStore, OperationBehavior};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum IdempotencyState {
    /// Request is in progress (locked)
    InProgress,
    /// Request completed successfully with a response
    Completed { response: Vec<u8> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum IdempotencyStartResult {
    Started,
    Locked,
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

    /// Try to acquire the lock for a request.
    pub fn try_start(&mut self, key: &str, ttl_seconds: u64) -> Result<IdempotencyStartResult> {
        let now = Timestamp::now();
        let expiry = now + Duration::from_secs(ttl_seconds);

        match self.kv.get(key)? {
            None => {
                // No existing entry - acquire lock
                let kv_model = KvModel {
                    value: IdempotencyState::InProgress.into(),
                    expiry: Some(expiry),
                };
                self.kv.set(key, &kv_model, OperationBehavior::Insert)?;
                Ok(IdempotencyStartResult::Started)
            }
            Some(kv_model) => {
                let state: IdempotencyState = kv_model.value.into();
                match state {
                    IdempotencyState::InProgress => {
                        // Still in progress by another request
                        Ok(IdempotencyStartResult::Locked)
                    }
                    IdempotencyState::Completed { response } => {
                        // Return cached response
                        Ok(IdempotencyStartResult::Completed { response })
                    }
                }
            }
        }
    }

    /// Complete a request with a successful response
    pub fn complete(&mut self, key: &str, response: Vec<u8>, ttl_seconds: u64) -> Result<()> {
        let now = Timestamp::now();
        let expiry = now + Duration::from_secs(ttl_seconds);

        let kv_model = KvModel {
            value: IdempotencyState::Completed { response }.into(),
            expiry: Some(expiry),
        };
        self.kv.set(key, &kv_model, OperationBehavior::Upsert)?;

        Ok(())
    }

    /// Abandon a request (remove the lock without saving response)
    pub fn abort(&mut self, key: &str) -> Result<()> {
        self.kv.delete(key)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use diom_configgroup::entities::EvictionPolicy;
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
        let result = store.try_start("test", 10)?;
        assert_eq!(result, IdempotencyStartResult::Started);

        let result = store.try_start("test", 10)?;
        assert_eq!(result, IdempotencyStartResult::Locked);

        store.abort("test")?;

        let result = store.try_start("test", 10)?;
        assert_eq!(result, IdempotencyStartResult::Started);

        store.complete("test", value.clone(), 10)?;

        let result = store.try_start("test", 10)?;
        assert_eq!(
            result,
            IdempotencyStartResult::Completed { response: value }
        );
        Ok(())
    }

    #[test]
    fn test_idempotency_missing() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];

        // Can complete a request without starting it first
        store.complete("test", value.clone(), 10)?;

        let result = store.try_start("test", 10)?;
        assert_eq!(
            result,
            IdempotencyStartResult::Completed { response: value }
        );

        // Can abort a request without starting it first
        store.abort("test2")?;
        let result2 = store.try_start("test2", 10)?;
        assert_eq!(result2, IdempotencyStartResult::Started);

        Ok(())
    }

    #[tokio::test]
    async fn test_idempotency_ttl() -> TestResult {
        let mut store = SetupFixture::new()?.store;
        let value = vec![1, 2, 3];

        store.try_start("test", 1)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1)?;
        assert_eq!(result, IdempotencyStartResult::Started);

        store.complete("test", value, 1)?;

        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = store.try_start("test", 1)?;
        assert_eq!(result, IdempotencyStartResult::Started);

        Ok(())
    }
}
