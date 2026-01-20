


use std::sync::Arc;

use fjall::KeyspaceCreateOptions;

use crate::error::Error;

#[derive(Clone)]
/// Tracks all internal state for Streams.
pub struct State {
    /// Where metadata for streams (created_at, stream names, offsets, acks, etc.) are stored.
    pub(self) metadata_tables: fjall::Keyspace,

    /// Where messages in a stream are stored.
    pub(self) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: &fjall::Database) -> Result<Self, Error> {
        const METADATA_KEYSPACE: &str = "_diom_stream_metadata";
        const MSG_TABLE_KEYSPACE: &str = "_diom_stream_msg_table";

        // There's probably more tweaking we can do for each of these tables, but for now,
        // this should suffice.

        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };
        
        let msg_table = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(MSG_TABLE_KEYSPACE, || opts)?
        };

        Ok(Self {
            metadata_tables, msg_table
        })
    }
}