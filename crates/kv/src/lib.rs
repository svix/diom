use coyote_error::Result;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod kvcontroller;
pub mod operations;
pub mod tables;

use crate::{kvcontroller::KvController, tables::KvPairRow};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub struct KvModel {
    pub expiry: Option<Timestamp>,
    pub value: Vec<u8>,
}

impl From<KvPairRow> for KvModel {
    fn from(row: KvPairRow) -> Self {
        Self {
            expiry: row.expiry,
            value: row.value,
        }
    }
}

const KV_KEYSPACE: &str = "mod_kv";

#[derive(Clone)]
pub struct State {
    pub controller: KvController,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(db, KV_KEYSPACE),
        })
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
pub async fn worker<F>(db: fjall::Database, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));
    let controller = KvController::new(db, KV_KEYSPACE);

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
    }
}
