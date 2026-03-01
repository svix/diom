use coyote_error::Error;
use coyote_namespace::Namespace;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use stream_internals::entities::StreamConfig;

pub mod entities;
pub mod operations;
pub(crate) mod tables;

const MSG_KEYSPACE: &str = "mod_msgs";
const METADATA_KEYSPACE: &str = "mod_msgs_metadata";

pub type MsgsNamespace = Namespace<StreamConfig>;

#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    pub(crate) metadata_tables: fjall::Keyspace,
    pub(crate) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        let msg_table = {
            let opts = KeyspaceCreateOptions::default()
                .expect_point_read_hits(true)
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(MSG_KEYSPACE, || opts)?
        };

        Ok(Self {
            db,
            metadata_tables,
            msg_table,
        })
    }
}
