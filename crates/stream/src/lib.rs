use coyote_error::Error;
use fjall::KeyspaceCreateOptions;

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
    /// Where metadata for streams (created_at, stream names, offsets, acks, etc.) are stored.
    pub(crate) metadata_tables: fjall::Keyspace,
}

impl State {
    pub fn init(db: &fjall::Database) -> Result<Self, Error> {
        const METADATA_KEYSPACE: &str = "_coyote_stream_metadata";

        // There's probably more tweaking we can do for each of these tables, but for now,
        // this should suffice.

        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        Ok(Self { metadata_tables })
    }
}
