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

use coyote_error::Result;
use coyote_kv::kvcontroller::{KvController, OperationBehavior};
use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(crate) enum IdempotencyState {
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
pub struct State {
    pub(crate) controller: KvController,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(db, "mod_idempotency"),
        })
    }
}

#[derive(Clone)]
pub struct IdempotencyStore {
    pub(crate) controller: KvController,
    pub(crate) namespace_id: NamespaceId,
}

impl IdempotencyStore {
    pub fn new(controller: KvController, namespace_id: NamespaceId) -> Self {
        Self {
            controller,
            namespace_id,
        }
    }

    /// Try to acquire the lock for a request.
    pub fn try_start(&self, key: &str, ttl_seconds: u64) -> Result<IdempotencyStartResult> {
        let now = Timestamp::now();
        let expiry = now + Duration::from_secs(ttl_seconds);

        match self.controller.fetch(self.namespace_id, key, now)? {
            None => {
                // No existing entry - acquire lock
                self.controller.set(
                    self.namespace_id,
                    key,
                    IdempotencyState::InProgress.into(),
                    Some(expiry),
                    OperationBehavior::Insert,
                    now,
                )?;
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
    pub fn complete(&self, key: &str, response: Vec<u8>, ttl_seconds: u64) -> Result<()> {
        let now = Timestamp::now();
        let expiry = now + Duration::from_secs(ttl_seconds);

        self.controller.set(
            self.namespace_id,
            key,
            IdempotencyState::Completed { response }.into(),
            Some(expiry),
            OperationBehavior::Upsert,
            now,
        )
    }

    /// Abandon a request (remove the lock without saving response)
    pub fn abort(&self, key: &str) -> Result<()> {
        self.controller.delete(self.namespace_id, key)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use coyote_namespace::entities::NamespaceId;
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
            let controller = KvController::new(db, "test");
            let store = IdempotencyStore::new(controller, NamespaceId::nil());
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
        let store = SetupFixture::new()?.store;
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
        let store = SetupFixture::new()?.store;
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
        let store = SetupFixture::new()?.store;
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
