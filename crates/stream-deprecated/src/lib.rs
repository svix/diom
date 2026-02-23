use std::sync::Arc;

use coyote_error::Error;
use fjall::{KeyspaceCreateOptions, compaction::Fifo};

/// User facing resources/types.
pub mod entities;

/// Operations that the API will call.
pub mod operations;

// Remaining modules are private, as they strictly relate to internal stream implementation details.
// So neither the end user, nor other crates outside of stream should know about them.
mod tables;

#[derive(Clone)]
/// Tracks all internal state for Streams.
pub struct State {
    pub(crate) db: fjall::Database,

    /// Where metadata for streams (created_at, stream names, offsets, acks, etc.) are stored.
    pub(crate) metadata_tables: fjall::Keyspace,

    /// Where all messages are stored.
    /// We're opting for a separate keyspace here as the access patterns for msgs are very different,
    /// so there's likely room for optimizations.
    pub(crate) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
        const METADATA_KEYSPACE: &str = "_coyote_stream_metadata";
        const MSG_TABLE_KEYSPACE: &str = "_coyote_stream_msgs";

        // There's probably more tweaking we can do for each of these tables, but for now,
        // this should suffice.

        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        let msg_table = {
            // NOTE(@svix-gabriel) I'm not 100% certain Fifo compaction strategy is the right call, but
            // here's the thinking:
            // - msgs are append and delete only. No updates.
            // - the keyspace only grows monotonically.
            //
            // However, we'll have to manually delete data for expired streams, so it might end up being
            // the case that Fifo is suboptimal here. 🤷
            let opts = KeyspaceCreateOptions::default().compaction_strategy(Arc::new(Fifo {
                limit: u64::MAX,
                ttl_seconds: None, // we have to manually expire data.
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
