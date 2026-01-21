use std::num::NonZeroU64;

use crate::entities::StreamId;

use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    NameToStreamRow::TABLE_PREFIX,
    StreamRow::TABLE_PREFIX,
]));

#[derive(Serialize, Deserialize)]
pub(crate) struct NameToStreamRow {
    pub name: String,
    pub id: StreamId,
}

impl TableRow for NameToStreamRow {
    const TABLE_PREFIX: &'static str = "_CID2NAME_";
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamRow {
    pub id: StreamId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_seconds: Option<NonZeroU64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl TableRow for StreamRow {
    const TABLE_PREFIX: &'static str = "_CSTRM_";

    type Key = StreamId;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}
