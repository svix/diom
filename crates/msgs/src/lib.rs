use std::sync::Arc;

use coyote_error::Error;
use fjall::{KeyspaceCreateOptions, compaction::Fifo};

pub mod entities;
pub mod operations;

// FIXME(@svix-gabriel): These fields will be used once publish/receive operations are added.
#[allow(dead_code)]
#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    pub(crate) metadata_tables: fjall::Keyspace,
    pub(crate) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
        const METADATA_KEYSPACE: &str = "_coyote_msgs_metadata";
        const MSG_TABLE_KEYSPACE: &str = "_coyote_msgs";

        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        let msg_table = {
            let opts = KeyspaceCreateOptions::default().compaction_strategy(Arc::new(Fifo {
                limit: u64::MAX,
                ttl_seconds: None,
            }));
            db.keyspace(MSG_TABLE_KEYSPACE, || opts)?
        };

        Ok(Self {
            db,
            metadata_tables,
            msg_table,
        })
    }
}
