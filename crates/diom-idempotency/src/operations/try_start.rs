use super::{IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyStartResult, IdempotencyStore};
use diom_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    pub(crate) key: String,
    pub(crate) ttl_seconds: u64,
}

impl TryStartOperation {
    pub fn new(key: String, ttl_seconds: u64) -> Self {
        Self { key, ttl_seconds }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartResponseData {
    pub result: IdempotencyStartResult,
}

impl TryStartOperation {
    fn apply_real(self, state: &mut IdempotencyStore) -> Result<TryStartResponseData> {
        let result = state.try_start(&self.key, self.ttl_seconds)?;
        Ok(TryStartResponseData { result })
    }
}

impl IdempotencyRequest for TryStartOperation {
    fn apply(self, state: &mut IdempotencyStore) -> TryStartResponse {
        TryStartResponse(self.apply_real(state))
    }
}
