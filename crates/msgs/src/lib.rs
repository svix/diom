use diom_error::Error;
use fjall::KeyspaceCreateOptions;

pub mod entities;
pub mod operations;
pub(crate) mod tables;

#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    pub(crate) metadata_tables: fjall::Keyspace,
    pub(crate) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
        const METADATA_KEYSPACE: &str = "_diom_msgs_metadata";
        const MSG_TABLE_KEYSPACE: &str = "_diom_msgs";

        let metadata_tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(METADATA_KEYSPACE, || opts)?
        };

        let msg_table = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(MSG_TABLE_KEYSPACE, || opts)?
        };

        Ok(Self {
            db,
            metadata_tables,
            msg_table,
        })
    }
}
