// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use coyote_error::Result;
use coyote_kv::{KvModel, KvStore, OperationBehavior};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) kv: KvStore,
}

impl CacheStore {
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    pub fn set(&mut self, key: &str, model: CacheModel) -> Result<()> {
        self.kv.set(key, &model.into(), OperationBehavior::Upsert)
    }

    pub fn get(&mut self, key: &str) -> Result<Option<CacheModel>> {
        self.kv.get(key).map(|m| m.map(Into::into))
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        self.kv.delete(key)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl From<CacheModel> for KvModel {
    fn from(model: CacheModel) -> Self {
        KvModel {
            value: model.value,
            expiry: model.expiry,
        }
    }
}

impl From<KvModel> for CacheModel {
    fn from(model: KvModel) -> Self {
        CacheModel {
            value: model.value,
            expiry: model.expiry,
        }
    }
}
