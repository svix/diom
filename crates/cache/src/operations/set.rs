use super::{CacheRequest, SetResponse};
use crate::{CacheModel, CacheStore};
use coyote_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    pub(crate) key: String,
    pub(crate) model: CacheModel,
}

impl SetOperation {
    pub fn new(key: String, model: CacheModel) -> Self {
        Self { key, model }
    }
}

impl SetOperation {
    fn apply_real(self, state: &mut CacheStore) -> Result<()> {
        state.set(&self.key, self.model)?;
        Ok(())
    }
}

impl CacheRequest for SetOperation {
    fn apply(self, state: &mut CacheStore) -> SetResponse {
        SetResponse(self.apply_real(state))
    }
}
