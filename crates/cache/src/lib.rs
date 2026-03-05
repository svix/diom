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

const CACHE_KEYSPACE: &str = "mod_cache";

#[derive(Clone)]
pub struct State {
    pub controller: KvController,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(db, CACHE_KEYSPACE),
        })
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
/// It deletes expired entries from the database and evicts entries if the Cache is configured to do so.
pub async fn worker<F>(db: fjall::Database, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
    let controller = KvController::new(db, CACHE_KEYSPACE);

    loop {
        if is_shutting_down() {
            break;
        }

        timer.tick().await;

        let now = Timestamp::now();
        match controller.clear_expired(now) {
            Ok(()) => {}
            Err(e) => {
                tracing::error!(error = ?e, "Failed to clean.");
            }
        };

        // FIXME: also do cache eviction once that's implemented
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}
