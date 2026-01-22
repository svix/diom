use std::{num::NonZeroU64, ops::RangeInclusive};

use crate::{
    State,
    entities::{MsgHeaders, MsgId, StreamId},
};

use diom_error::{Error, Result};
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

/// MsgRow is slightly different from the other models in that:
///
/// * Its in its own keyspace (so need for a unique TABLE_PREFIX)
/// * The keys need to support iterating through all msgs for a stream in order of msg-id.
///
/// This necessitates rolling a slightly boutique key format, so we don't implement [TableRow] for this
/// struct.
#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub payload: Vec<u8>,
    #[serde(skip_serializing_if = "MsgHeaders::is_empty")]
    pub headers: MsgHeaders,
    pub created_at: Timestamp,
}

const MSG_KEY_LEN: usize = size_of::<StreamId>() + size_of::<MsgId>();

/// The MsgRowKey is effectively a (stream_id, msg_id), serialized so that they
/// are ordered by msg_id in fjall, and grouped by stream_ids.
type MsgRowKey = [u8; MSG_KEY_LEN];

pub(crate) fn msg_row_key(s_id: StreamId, m_id: MsgId) -> MsgRowKey {
    let mut key: [u8; _] = Default::default();
    {
        let (prefix, suffix) = key.split_at_mut(size_of::<StreamId>());

        let stream_id = s_id.as_u128().to_be_bytes();
        prefix.copy_from_slice(&stream_id);

        let msg_id = m_id.to_be_bytes();
        suffix.copy_from_slice(&msg_id);
    }
    key
}

fn parse_msg_id(key: MsgRowKey) -> Result<MsgId> {
    let bytes = (&key[size_of::<StreamId>()..])
        .try_into()
        .map_err(Error::generic)?;
    Ok(MsgId::from_be_bytes(bytes))
}

fn msg_row_key_range(s_id: StreamId, range: RangeInclusive<MsgId>) -> RangeInclusive<MsgRowKey> {
    msg_row_key(s_id, *range.start())..=msg_row_key(s_id, *range.end())
}

impl MsgRow {
    /// Returns the *next* id for a msg in the stream.
    pub(crate) fn get_next_msg_id_in_stream(state: &State, stream_id: StreamId) -> Result<MsgId> {
        let range = msg_row_key_range(stream_id, MsgId::MIN..=MsgId::MAX);

        let Some(max_entry) = state.msg_table.range(range).next_back() else {
            return Ok(MsgId::MIN);
        };

        let key: MsgRowKey = max_entry
            .key()?
            .as_ref()
            .try_into()
            .map_err(Error::generic)?;

        let msg_id = parse_msg_id(key)?;

        Ok(msg_id + 1)
    }

    pub(crate) fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec(&self)
            .map(fjall::UserValue::from)
            .map_err(Error::generic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn msg_row_keys_are_sorted_correctly() {
        let db = fjall::Database::builder("msg_row_keys_are_sorted_correctly_test")
            .temporary(true)
            .open()
            .unwrap();
        let ks = db
            .keyspace("test", fjall::KeyspaceCreateOptions::default)
            .unwrap();

        let s1 = StreamId::new_v4();
        let s2 = StreamId::new_v4();

        // Insert IDs in an arbitrary order.
        let ids = [100, 2, 1, MsgId::MAX, MsgId::MAX - 1, 999];
        for id in ids {
            ks.insert(msg_row_key(s1, id), []).unwrap();
            ks.insert(msg_row_key(s2, id), []).unwrap();
        }
        ks.insert(msg_row_key(s1, 1234), []).unwrap();
        ks.insert(msg_row_key(s2, 5678), []).unwrap();

        // We should be able to do range queries for all messages within a single stream.
        // AND msgs should be returned, ordered by ID.

        let mut s1_msgs = ks
            .range(msg_row_key_range(s1, MsgId::MIN..=MsgId::MAX))
            .map(|g| g.key().unwrap());
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, 1));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, 2));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, 100));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, 999));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, 1234));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, MsgId::MAX - 1));
        assert_eq!(s1_msgs.next().unwrap(), msg_row_key(s1, MsgId::MAX));
        assert_eq!(s1_msgs.next(), None);

        let mut s2_msgs = ks
            .range(msg_row_key_range(s2, MsgId::MIN..=MsgId::MAX))
            .map(|g| g.key().unwrap());
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, 1));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, 2));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, 100));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, 999));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, 5678));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, MsgId::MAX - 1));
        assert_eq!(s2_msgs.next().unwrap(), msg_row_key(s2, MsgId::MAX));
        assert_eq!(s2_msgs.next(), None);
    }
}
