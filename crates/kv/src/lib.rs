use coyote_core::Monotime;
use coyote_error::Result;
use coyote_namespace::{Namespace, entities::KeyValueConfig};
use fjall_utils::{Databases, StorageType};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod kvcontroller;
pub mod operations;
pub mod tables;

use crate::kvcontroller::KvController;

pub type KvNamespace = Namespace<KeyValueConfig>;
const KV_KEYSPACE: &str = "mod_kv";

#[derive(Clone)]
pub struct State {
    persistent_controller: KvController,
    ephemeral_controller: KvController,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        Ok(Self {
            persistent_controller: KvController::new(dbs.persistent, KV_KEYSPACE),
            ephemeral_controller: KvController::new(dbs.ephemeral, KV_KEYSPACE),
        })
    }

    pub fn controller(&self, storage_type: StorageType) -> &KvController {
        match storage_type {
            StorageType::Persistent => &self.persistent_controller,
            StorageType::Ephemeral => &self.ephemeral_controller,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OperationBehavior {
    #[default]
    Upsert,
    Insert,
    Update,
}

/// This is the worker function for this module, it does background cleanup and accounting.
///
/// It deletes expired entries from the database and evicts entries if the KvStore is configured to do so.
/// This function may never make any user-visible changes, under penalty of breaking replication.
pub async fn worker(state: State, time: Monotime) {
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(10));

    let shutting_down = coyote_core::shutdown::shutting_down_token();
    while shutting_down
        .run_until_cancelled(timer.tick())
        .await
        .is_some()
    {
        let controllers = [
            state.persistent_controller.clone(),
            state.ephemeral_controller.clone(),
        ];
        let tasks = controllers.into_iter().map(|c| {
            let now = time.last();
            tokio::task::spawn_blocking(move || {
                match c.clear_expired(now) {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!(error = ?e, "Failed to clean store.");
                    }
                };
            })
        });
        for result in futures_util::future::join_all(tasks).await {
            if let Err(e) = result {
                tracing::error!(error = ?e, "Failed to join cleanup task");
            }
        }
    }
}
