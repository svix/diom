pub mod algorithms;
pub mod controller;
pub mod operations;
pub mod tables;

use std::time::Duration;

pub use crate::{algorithms::TokenBucket, controller::RateLimitController};
use coyote_error::Result;
use coyote_namespace::Namespace;
pub use coyote_namespace::entities::RateLimitConfig;
use fjall_utils::Databases;

pub type RateLimitNamespace = Namespace<RateLimitConfig>;

const RATE_LIMIT_KEYSPACE: &str = "mod_rate_limit";

#[derive(Clone)]
pub struct State {
    controller: RateLimitController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            controller: RateLimitController::new(dbs.ephemeral)?,
        })
    }

    pub fn controller(&self) -> &RateLimitController {
        &self.controller
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
