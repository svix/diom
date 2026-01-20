use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::Result,
    v1::modules::kv::{Kv2Model, Kv2Store, OperationBehavior},
};

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) kv: Kv2Store,
}

impl CacheStore {
    pub fn new(kv: Kv2Store) -> Self {
        Self { kv }
    }

    pub fn set(&self, key: EntityKey, model: CacheModel) -> Result<()> {
        self.kv.set(&key, &model.into(), OperationBehavior::Upsert)
    }

    pub fn get(&self, key: &EntityKey) -> Result<Option<CacheModel>> {
        self.kv.get(&key.0).map(|m| m.map(Into::into))
    }

    pub fn delete(&self, key: &EntityKey) -> Result<()> {
        self.kv.delete(&key.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expires_at: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl From<CacheModel> for Kv2Model {
    fn from(model: CacheModel) -> Self {
        Kv2Model {
            value: model.value,
            expires_at: model.expires_at,
        }
    }
}

impl From<Kv2Model> for CacheModel {
    fn from(model: Kv2Model) -> Self {
        CacheModel {
            value: model.value,
            expires_at: model.expires_at,
        }
    }
}
