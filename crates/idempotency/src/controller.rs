use diom_core::types::DurationMs;
use diom_error::Result;
use diom_id::NamespaceId;
use diom_kv::kvcontroller::{KvController, KvModelIn, OperationBehavior};
use jiff::Timestamp;

use crate::{IdempotencyStartResult, IdempotencyState};

#[derive(Clone)]
pub struct IdempotencyController {
    kv: KvController,
}

impl IdempotencyController {
    pub fn new(kv: KvController) -> Self {
        Self { kv }
    }

    pub async fn try_start(
        &self,
        namespace_id: NamespaceId,
        key: String,
        ttl: DurationMs,
        now: Timestamp,
        log_index: u64,
    ) -> Result<IdempotencyStartResult> {
        match self.kv.fetch(namespace_id, key.clone(), now).await? {
            None => {
                self.kv
                    .set(
                        namespace_id,
                        key.clone(),
                        KvModelIn {
                            value: IdempotencyState::InProgress.into(),
                            expiry: Some(now + ttl),
                            version: None,
                        },
                        OperationBehavior::Insert,
                        now,
                        log_index,
                    )
                    .await?;
                Ok(IdempotencyStartResult::Started)
            }
            Some(kv_model) => {
                let idem_state: IdempotencyState = kv_model.value.into();
                let result = match idem_state {
                    IdempotencyState::InProgress => IdempotencyStartResult::Locked,
                    IdempotencyState::Completed { response } => {
                        IdempotencyStartResult::Completed { response }
                    }
                };
                Ok(result)
            }
        }
    }

    pub async fn complete(
        &self,
        namespace_id: NamespaceId,
        key: String,
        response: Vec<u8>,
        ttl: DurationMs,
        now: Timestamp,
        log_index: u64,
    ) -> Result<()> {
        let expiry = now + ttl;

        self.kv
            .set(
                namespace_id,
                key,
                KvModelIn {
                    value: IdempotencyState::Completed { response }.into(),
                    expiry: Some(expiry),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                log_index,
            )
            .await?;

        Ok(())
    }

    pub async fn abort(&self, namespace_id: NamespaceId, key: String) -> Result<()> {
        self.kv.delete(namespace_id, key).await?;
        Ok(())
    }

    pub(crate) fn clear_expired_in_background(&self, now: Timestamp) -> Result<usize> {
        self.kv.clear_expired_in_background(now)
    }
}
