use diom_error::Result;
use diom_namespace::{Namespace, entities::KeyValueConfig};
use fjall_utils::{Databases, StorageType};
use jiff::Timestamp;
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
/// It deletes expired entries from the database and evicts entries if the KvStore is configured to do so.
pub async fn worker<F>(dbs: Databases, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
    // FIXME: handle both!
    let controller = KvController::new(dbs.persistent, KV_KEYSPACE);

    loop {
        if is_shutting_down() {
            break;
        }

        timer.tick().await;

        let now = Timestamp::now();
        match controller.clear_expired(now) {
            Ok(_) => {}
            Err(e) => {
                tracing::error!(error = ?e, "Failed to clean.");
            }
        };
    }
}
