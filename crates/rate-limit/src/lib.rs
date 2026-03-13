pub mod algorithms;
pub mod controller;
pub mod operations;
pub mod tables;

use std::time::Duration;

pub use crate::{algorithms::TokenBucket, controller::RateLimitController};
use diom_error::{Result, ResultExt as _};
use diom_namespace::Namespace;
pub use diom_namespace::entities::RateLimitConfig;
use fjall::KeyspaceCreateOptions;
use fjall_utils::{Databases, StorageType};

pub type RateLimitNamespace = Namespace<RateLimitConfig>;

const RATE_LIMIT_KEYSPACE: &str = "mod_rate_limit";

#[derive(Clone)]
pub struct State {
    persistent: RateLimitController,
    ephemeral: RateLimitController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        let opts = || KeyspaceCreateOptions::default();
        let persistent_tables = dbs
            .persistent
            .keyspace(RATE_LIMIT_KEYSPACE, opts)
            .or_internal_error()?;
        let ephemeral_tables = dbs
            .ephemeral
            .keyspace(RATE_LIMIT_KEYSPACE, opts)
            .or_internal_error()?;
        Ok(Self {
            persistent: RateLimitController::new(dbs.persistent, persistent_tables),
            ephemeral: RateLimitController::new(dbs.ephemeral, ephemeral_tables),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &RateLimitController {
        match storage_type {
            StorageType::Persistent => &self.persistent,
            StorageType::Ephemeral => &self.ephemeral,
        }
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker<F>(_dbs: Databases, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    loop {
        if is_shutting_down() {
            break;
        }
        // FIXME(@svix-lucho): add background cleanup task. Cleanup logic depends on the algorithm.
        // Delete the expired/non-rate-limited entries first.
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
