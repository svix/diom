// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use coyote_error::Result;
use coyote_kv::kvcontroller::KvController;
use coyote_namespace::entities::NamespaceId;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone)]
pub struct State {
    pub controller: KvController,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(db, "mod_cache"),
        })
    }
}

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) controller: KvController,
    pub(crate) namespace_id: NamespaceId,
}

impl CacheStore {
    pub fn new(controller: KvController, namespace_id: NamespaceId) -> Self {
        Self {
            controller,
            namespace_id,
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<CacheModel>> {
        self.controller
            .fetch(self.namespace_id, key, Timestamp::now())
            .map(|m| {
                m.map(|m| CacheModel {
                    value: m.value,
                    expiry: m.expiry,
                })
            })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}
