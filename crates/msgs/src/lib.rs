use std::sync::Arc;

use coyote_error::Error;
use fjall::{KeyspaceCreateOptions, compaction::Fifo};

pub mod entities;
pub mod operations;
pub(crate) mod tables;

#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    // FIXME(@svix-gabriel): Will be used for lease/offset storage.
    #[allow(dead_code)]
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
