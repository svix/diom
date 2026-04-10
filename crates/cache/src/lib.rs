//! # Cache module.
//!
//! This module implements a cache store.

pub mod compaction;
pub mod operations;

mod controller;
mod tables;

use diom_core::types::ByteString;
use diom_error::Result;
use diom_namespace::{Namespace, entities::CacheConfig};
use fjall_utils::Databases;
use jiff::Timestamp;

use controller::CacheController;

pub type CacheNamespace = Namespace<CacheConfig>;

pub const CACHE_KEYSPACE: &str = "mod_cache";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheModel {
    pub value: ByteString,
    pub expiry: Timestamp,
}

#[derive(Clone)]
pub struct State {
    controller: CacheController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: CacheController::new(dbs.ephemeral, CACHE_KEYSPACE),
        })
    }

    pub fn controller(&self) -> &CacheController {
        &self.controller
    }
}
