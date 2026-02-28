use diom_error::Error;
use diom_namespace::entities::NamespaceId;
use fjall::KeyspaceCreateOptions;
use fjall_utils::{ReadableDatabase, ReadableKeyspace};

pub mod entities;
pub mod operations;
pub(crate) mod tables;

const METADATA_KEYSPACE: &str = "_diom_msgs_metadata";

#[derive(Clone)]
pub struct State {
    pub(crate) db: fjall::Database,
    pub(crate) metadata_tables: fjall::Keyspace,
    pub(crate) msg_table: fjall::Keyspace,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
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

/// Reads the configured partition count for a topic from a database handle.
pub fn topic_partition_count(
    db: &impl ReadableDatabase,
    ns_id: NamespaceId,
    topic: &str,
) -> diom_error::Result<u16> {
    let metadata = db.keyspace(METADATA_KEYSPACE)?;
    let key = tables::topic_config_key(ns_id, topic);
    match metadata.get(&key)? {
        Some(val) => {
            let config: tables::TopicConfig =
                rmp_serde::from_slice(&val).map_err(Error::generic)?;
            Ok(config.partition_count)
        }
        None => Ok(entities::DEFAULT_PARTITION_COUNT),
    }
}

/// Checks whether a partition has any active (unexpired, unacked, non-DLQ) lease
/// for the given consumer group.
pub fn partition_has_active_lease(
    db: &impl ReadableDatabase,
    ns_id: NamespaceId,
    partition: entities::Partition,
    cg: &entities::ConsumerGroup,
    now: jiff::Timestamp,
) -> diom_error::Result<bool> {
    let metadata = db.keyspace(METADATA_KEYSPACE)?;
    tables::LeaseRow::has_active_lease_in(&metadata, ns_id, partition, cg, now)
}
