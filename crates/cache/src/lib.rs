// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Cache module.
//!
//! This module implements a cache store.

pub mod operations;

use coyote_error::Result;
use coyote_kv::kvcontroller::KvController;
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}
