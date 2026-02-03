use std::{
    borrow::Cow,
    num::{NonZeroU64, NonZeroUsize},
    ops::RangeInclusive,
};

use coyote_error::{Error, HttpError, Result};
use fjall::OwnedWriteBatch;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, MsgHeaders, MsgId, StreamId},
};

// IMPORTANT. Since these are all shared in the same fjall::Keyspace, the table prefixes must be unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    NameToStreamRow::TABLE_PREFIX,
    StreamRow::TABLE_PREFIX,
    LeaseRow::TABLE_PREFIX,
]));

use crate::entities::StreamName;

#[derive(Serialize, Deserialize)]
pub(crate) struct NameToStreamRow {
    pub name: StreamName,
    pub id: StreamId,
}

impl TableRow for NameToStreamRow {
    const TABLE_PREFIX: &'static str = "_CID2NAME_";
    type Key = String;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.name)
    }
}

impl NameToStreamRow {
    /// Looks up the `StreamId` for a given stream name.
    pub(crate) fn get_stream_id(state: &State, name: &str) -> Result<StreamId> {
        Self::fetch(&state.metadata_tables, &name.to_owned())?
            .map(|row| row.id)
            .ok_or_else(|| {
                HttpError::not_found(
                    Some("stream_not_found".to_owned()),
                    Some(format!("Stream with name '{name}' not found")),
                )
                .into()
            })
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StreamRow {
    pub id: StreamId,
    pub name: StreamName,
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

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(self.id)
    }
}

/// A lease represents a consumer group's "hold" on a block of messages. This is used to prevent multiple consumers in the same consumer group
/// from touching the same messages.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct LeaseRow {
    pub stream_id: StreamId,
    pub cg: ConsumerGroup,
    /// The first id of the message in the block held by the LeaseRow.
    pub block_start: MsgId,
    /// The last id of the message in the block held by the LeaseRow.
    pub block_end: MsgId,
    pub leased_at: Timestamp,
    /// LeaseRows expire based on the visibility timeout.
    pub expires_at: Timestamp,
    /// When the block of messages was acknowledged / processed by a consumer in the group.
    #[serde(default)]
    pub acked_at: Option<Timestamp>,
    /// DLQ'd messages are saved as a lease.
    #[serde(default)]
    pub dlq_at: Option<Timestamp>,
}

/// For the key, we use a composite of (stream_id, consumer_group, block_end).
/// This ensures we can easily fetch all of the leases for a (stream_id, consumer_group).
#[derive(Clone)]
pub(crate) struct LeaseKey(Vec<u8>);

impl LeaseKey {
    pub(crate) fn new(id: StreamId, cg: &ConsumerGroup, block_end: MsgId) -> Self {
        let msg_id = block_end.to_be_bytes();
        let id = id.as_bytes();

        let mut key = Vec::with_capacity(id.len() + cg.len() + msg_id.len() + b"\0\0".len());
        key.extend_from_slice(id);
        key.extend_from_slice(b"\0");
        key.extend_from_slice(cg.as_bytes());
        key.extend_from_slice(b"\0");
        key.extend_from_slice(&msg_id);

        Self(key)
    }

    fn prefix(id: StreamId, cg: &ConsumerGroup) -> Vec<u8> {
        let id_bytes = id.as_bytes();
        let cg_bytes = cg.as_bytes();

        let mut prefix = Vec::with_capacity(
            LeaseRow::TABLE_PREFIX.len()
                + b"\0".len()
                + id_bytes.len()
                + b"\0".len()
                + cg_bytes.len()
                + b"\0".len(),
        );
        prefix.extend_from_slice(LeaseRow::TABLE_PREFIX.as_bytes());
        prefix.extend_from_slice(b"\0");
        prefix.extend_from_slice(id_bytes);
        prefix.extend_from_slice(b"\0");
        prefix.extend_from_slice(cg_bytes);
        prefix.extend_from_slice(b"\0");

        prefix
    }
}

pub(crate) struct LeaseDiff {
    pub to_delete: Vec<LeaseRow>,
    pub to_insert: Vec<LeaseRow>,
}

impl LeaseDiff {
    pub(crate) fn apply_diff(self, state: &State, batch: &mut OwnedWriteBatch) -> Result<()> {
        for lease in self.to_delete {
            let key = LeaseRow::make_fjall_key(lease.get_key().as_ref());
            batch.remove(&state.metadata_tables, key);
        }

        for lease in self.to_insert {
            let (key, val) = lease.to_fjall_entry()?;
            batch.insert(&state.metadata_tables, key, val);
        }

        Ok(())
    }
}

impl LeaseRow {
    pub(crate) fn fetch_all(state: &State, id: StreamId, cg: &ConsumerGroup) -> Result<Vec<Self>> {
        let prefix = LeaseKey::prefix(id, cg);

        state
            .metadata_tables
            .prefix(prefix)
            .map(|entry| {
                let value = entry.value()?;
                Self::from_fjall_value(value)
            })
            .collect()
    }

    /// Given a set of Leases, returns a diff describing
    ///     1. Leases to delete from the database.
    ///     2. Leases to insert.
    ///
    /// Any Lease that has expired (it's expiration time is past `now`), can be trivially
    /// deleted. DLQ'd leases are never deleted by expiration.
    ///
    /// Any acked or DLQ'd leases where the message ranges are adjacent or overlapping
    /// can be compacted. This involves deleting the old leases, and replacing them with
    /// a merged lease.
    pub(crate) fn cull_and_compact(leases: Vec<Self>, now: Timestamp) -> LeaseDiff {
        let mut to_delete = Vec::new();
        let mut to_insert = Vec::new();

        let mut acked_leases: Vec<Self> = Vec::new();
        let mut dlq_leases: Vec<Self> = Vec::new();

        for lease in leases {
            if lease.dlq_at.is_some() {
                dlq_leases.push(lease);
            } else if lease.expires_at < now {
                to_delete.push(lease);
            } else if lease.acked_at.is_some() {
                acked_leases.push(lease);
            }
        }

        Self::compact_adjacent(acked_leases, &mut to_delete, &mut to_insert);
        Self::compact_adjacent(dlq_leases, &mut to_delete, &mut to_insert);

        LeaseDiff {
            to_delete,
            to_insert,
        }
    }

    /// Compacts adjacent or overlapping leases by merging them into a single lease.
    /// Groups of size 1 are left untouched.
    fn compact_adjacent(
        mut leases: Vec<Self>,
        to_delete: &mut Vec<Self>,
        to_insert: &mut Vec<Self>,
    ) {
        if leases.is_empty() {
            return;
        }

        leases.sort_by_key(|l| l.block_start);

        let mut groups: Vec<Vec<Self>> = Vec::new();
        let mut current_group = vec![leases.remove(0)];

        for lease in leases {
            let last = current_group.last().unwrap();
            if last.block_end + 1 >= lease.block_start {
                current_group.push(lease);
            } else {
                groups.push(std::mem::take(&mut current_group));
                current_group.push(lease);
            }
        }
        groups.push(current_group);

        for group in groups {
            if group.len() == 1 {
                continue;
            }

            let stream_id = group[0].stream_id;
            let cg = group[0].cg.clone();

            let mut merged = LeaseRow {
                stream_id,
                cg,
                block_start: MsgId::MAX,
                block_end: MsgId::MIN,
                leased_at: Timestamp::MAX,
                expires_at: Timestamp::MIN,
                acked_at: None,
                dlq_at: None,
            };

            for lease in group {
                merged.block_start = merged.block_start.min(lease.block_start);
                merged.block_end = merged.block_end.max(lease.block_end);
                merged.leased_at = merged.leased_at.min(lease.leased_at);
                merged.expires_at = merged.expires_at.max(lease.expires_at);
                if let Some(ack) = lease.acked_at {
                    merged.acked_at = Some(merged.acked_at.map_or(ack, |a| a.max(ack)));
                }
                if let Some(dlq) = lease.dlq_at {
                    merged.dlq_at = Some(merged.dlq_at.map_or(dlq, |d| d.max(dlq)));
                }
                to_delete.push(lease);
            }

            to_insert.push(merged);
        }
    }

    pub(crate) fn is_active(&self, now: Timestamp) -> bool {
        self.acked_at.is_none() && self.dlq_at.is_none() && self.expires_at > now
    }

    pub(crate) fn is_dlq(&self) -> bool {
        self.dlq_at.is_some()
    }

    /// Shrinks or splits active leases to exclude a range of message IDs.
    /// This is called when messages are acked or DLQ'd to ensure the active lease
    /// no longer blocks those messages.
    pub(crate) fn shrink_active_leases_for_range(
        leases: &[Self],
        min_msg_id: MsgId,
        max_msg_id: MsgId,
        now: Timestamp,
        lease_diff: &mut LeaseDiff,
    ) {
        for lease in leases {
            if !lease.is_active(now) {
                continue;
            }

            // It only makes sense to shrink the lease if it overlaps with the msg range.
            if lease.block_end < min_msg_id || max_msg_id < lease.block_start {
                continue;
            }

            lease_diff.to_delete.push(lease.clone());

            // Create replacement lease(s) for any parts that don't overlap with the range
            // Left remainder: [lease.block_start, min_msg_id - 1] if lease starts before range
            if lease.block_start < min_msg_id {
                lease_diff.to_insert.push(Self {
                    block_end: min_msg_id - 1,
                    ..lease.clone()
                });
            }

            // Right remainder: [max_msg_id + 1, lease.block_end] if lease ends after range
            if lease.block_end > max_msg_id {
                lease_diff.to_insert.push(Self {
                    block_start: max_msg_id + 1,
                    ..lease.clone()
                });
            }
        }
    }
}

impl AsRef<[u8]> for LeaseKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TableRow for LeaseRow {
    const TABLE_PREFIX: &'static str = "_CSLEASE_";

    type Key = LeaseKey;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        let key = LeaseKey::new(self.stream_id, &self.cg, self.block_end);
        Cow::Owned(key)
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
    pub(crate) fn max_id_in_stream(state: &State, stream_id: StreamId) -> Result<Option<MsgId>> {
        let range = msg_row_key_range(stream_id, MsgId::MIN..=MsgId::MAX);

        let Some(max_entry) = state.msg_table.range(range).next_back() else {
            return Ok(None);
        };

        let key: MsgRowKey = max_entry
            .key()?
            .as_ref()
            .try_into()
            .map_err(Error::generic)?;

        let msg_id = parse_msg_id(key)?;

        Ok(Some(msg_id))
    }

    /// Returns the *next* id for a msg in the stream.
    pub(crate) fn get_next_msg_id_in_stream(state: &State, stream_id: StreamId) -> Result<MsgId> {
        match Self::max_id_in_stream(state, stream_id)? {
            None => Ok(MsgId::MIN),
            Some(id) => Ok(id + 1),
        }
    }

    pub(crate) fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec(&self)
            .map(fjall::UserValue::from)
            .map_err(Error::generic)
    }

    /// Fetches messages in the specified range, filling up the buffer with at most batch_size msgs.
    ///
    /// Returns the number msgs fetched successfully.
    fn fetch_in_range(
        state: &State,
        range: RangeInclusive<MsgRowKey>,
        buf: &mut Vec<(MsgId, MsgRow)>,
        batch_size: usize,
    ) -> Result<usize> {
        let mut n = 0;

        let msgs = state.msg_table.range(range).take(batch_size);

        for entry in msgs {
            let (key_slice, val_slice) = entry.into_inner()?;

            let key: MsgRowKey = key_slice.as_ref().try_into().map_err(Error::generic)?;

            let msg_id = parse_msg_id(key)?;

            let msg_row: MsgRow = rmp_serde::from_slice(&val_slice).map_err(Error::generic)?;

            buf.push((msg_id, msg_row));
            n += 1;
        }

        Ok(n)
    }

    /// Fetch up to `batch_size` available messages, in order, from the stream.
    /// Messages captured by the leases are excluded.
    pub(crate) fn fetch_available<'a>(
        state: &State,
        stream_id: StreamId,
        leases: impl IntoIterator<Item = &'a LeaseRow>,
        batch_size: NonZeroUsize,
    ) -> Result<Vec<(MsgId, MsgRow)>> {
        let blocked_ranges = merge_lease_ranges(leases.into_iter());
        let mut msgs_left = batch_size.into();
        let mut results = Vec::with_capacity(msgs_left);

        let mut min = MsgId::MIN;
        for block in blocked_ranges {
            if block.start > min {
                let max = block.start - 1;
                let range = msg_row_key_range(stream_id, min..=max);

                let n = MsgRow::fetch_in_range(state, range, &mut results, msgs_left)?;
                msgs_left -= n;

                if msgs_left == 0 {
                    return Ok(results);
                }
            }
            min = block.end + 1;
        }

        if msgs_left > 0 {
            let range = msg_row_key_range(stream_id, min..=MsgId::MAX);
            MsgRow::fetch_in_range(state, range, &mut results, msgs_left)?;
        }

        Ok(results)
    }
}

#[derive(Debug)]
/// A range of message IDs that are blocked by leases.
struct BlockedRange {
    start: MsgId,
    end: MsgId,
}

/// Merge and sort lease ranges for the given stream into non-overlapping blocked ranges.
fn merge_lease_ranges<'a>(leases: impl Iterator<Item = &'a LeaseRow>) -> Vec<BlockedRange> {
    let mut ranges: Vec<BlockedRange> = leases
        .map(|l| BlockedRange {
            start: l.block_start,
            end: l.block_end,
        })
        .collect();

    if ranges.len() <= 1 {
        return ranges;
    }

    ranges.sort_by_key(|r| r.start);

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges.remove(0);

    for range in ranges {
        // Check for overlap or adjacency.
        if range.start <= current.end.saturating_add(1) {
            current.end = current.end.max(range.end);
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts(secs: i64) -> Timestamp {
        Timestamp::from_second(secs).unwrap()
    }

    #[test]
    fn cull_and_compact_expired_lease_is_deleted() {
        let stream_id = StreamId::new_v4();
        let lease = LeaseRow {
            stream_id,
            cg: "cg1".to_string(),
            block_start: 1,
            block_end: 5,
            leased_at: ts(0),
            expires_at: ts(50),
            acked_at: None,
            dlq_at: None,
        };

        let diff = LeaseRow::cull_and_compact(vec![lease.clone()], ts(100));

        assert_eq!(diff.to_delete.len(), 1);
        assert_eq!(diff.to_delete[0], lease);
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_active_lease_unchanged() {
        let stream_id = StreamId::new_v4();
        let lease = LeaseRow {
            stream_id,
            cg: "cg1".to_string(),
            block_start: 1,
            block_end: 5,
            leased_at: ts(0),
            expires_at: ts(200),
            acked_at: None,
            dlq_at: None,
        };

        let diff = LeaseRow::cull_and_compact(vec![lease], ts(100));

        assert!(diff.to_delete.is_empty());
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_single_acked_lease_unchanged() {
        let stream_id = StreamId::new_v4();
        let lease = LeaseRow {
            stream_id,
            cg: "cg1".to_string(),
            block_start: 1,
            block_end: 5,
            leased_at: ts(0),
            expires_at: ts(200),
            acked_at: Some(ts(50)),
            dlq_at: None,
        };

        let diff = LeaseRow::cull_and_compact(vec![lease], ts(100));

        assert!(diff.to_delete.is_empty());
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_multiple_expired_leases_all_deleted() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(0),
                expires_at: ts(50),
                acked_at: None,
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(0),
                expires_at: ts(60),
                acked_at: None,
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 20,
                block_end: 25,
                leased_at: ts(0),
                expires_at: ts(70),
                acked_at: None,
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 3);
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_multiple_active_leases_unchanged() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(0),
                expires_at: ts(200),
                acked_at: None,
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(0),
                expires_at: ts(200),
                acked_at: None,
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert!(diff.to_delete.is_empty());
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_two_adjacent_acked_leases_are_compacted() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 2);
        assert_eq!(diff.to_insert.len(), 1);

        let merged = &diff.to_insert[0];
        assert_eq!(merged.block_start, 1);
        assert_eq!(merged.block_end, 10);
        assert_eq!(merged.acked_at, Some(ts(25)));
    }

    #[test]
    fn cull_and_compact_two_overlapping_acked_leases_are_compacted() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 7,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 5,
                block_end: 12,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 2);
        assert_eq!(diff.to_insert.len(), 1);

        let merged = &diff.to_insert[0];
        assert_eq!(merged.block_start, 1);
        assert_eq!(merged.block_end, 12);
    }

    #[test]
    fn cull_and_compact_two_non_adjacent_acked_leases_not_compacted() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 10,
                block_end: 15,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert!(diff.to_delete.is_empty());
        assert!(diff.to_insert.is_empty());
    }

    #[test]
    fn cull_and_compact_chain_of_adjacent_acked_leases_all_compacted() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 11,
                block_end: 15,
                leased_at: ts(18),
                expires_at: ts(200),
                acked_at: Some(ts(30)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 3);
        assert_eq!(diff.to_insert.len(), 1);

        let merged = &diff.to_insert[0];
        assert_eq!(merged.block_start, 1);
        assert_eq!(merged.block_end, 15);
        assert_eq!(merged.acked_at, Some(ts(30)));
    }

    #[test]
    fn cull_and_compact_mixed_expired_active_and_acked() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(0),
                expires_at: ts(50),
                acked_at: None,
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(0),
                expires_at: ts(200),
                acked_at: None,
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 20,
                block_end: 25,
                leased_at: ts(0),
                expires_at: ts(200),
                acked_at: Some(ts(50)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 26,
                block_end: 30,
                leased_at: ts(0),
                expires_at: ts(200),
                acked_at: Some(ts(55)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 3);
        assert_eq!(diff.to_insert.len(), 1);

        let merged = &diff.to_insert[0];
        assert_eq!(merged.block_start, 20);
        assert_eq!(merged.block_end, 30);
    }

    #[test]
    fn cull_and_compact_multiple_separate_groups_of_acked_leases() {
        let stream_id = StreamId::new_v4();
        let leases = vec![
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 1,
                block_end: 5,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 6,
                block_end: 10,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 50,
                block_end: 55,
                leased_at: ts(10),
                expires_at: ts(200),
                acked_at: Some(ts(20)),
                dlq_at: None,
            },
            LeaseRow {
                stream_id,
                cg: ConsumerGroup::from("cg1".to_string()),
                block_start: 56,
                block_end: 60,
                leased_at: ts(15),
                expires_at: ts(200),
                acked_at: Some(ts(25)),
                dlq_at: None,
            },
        ];

        let diff = LeaseRow::cull_and_compact(leases, ts(100));

        assert_eq!(diff.to_delete.len(), 4);
        assert_eq!(diff.to_insert.len(), 2);

        let mut merged: Vec<_> = diff.to_insert.iter().collect();
        merged.sort_by_key(|l| l.block_start);

        assert_eq!(merged[0].block_start, 1);
        assert_eq!(merged[0].block_end, 10);
        assert_eq!(merged[1].block_start, 50);
        assert_eq!(merged[1].block_end, 60);
    }

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

    mod fetch_available_tests {
        use super::*;

        fn test_state(name: &str) -> State {
            let db = fjall::Database::builder(name)
                .temporary(true)
                .open()
                .unwrap();
            State::init(db).unwrap()
        }

        fn insert_msg(state: &State, stream_id: StreamId, msg_id: MsgId) {
            let row = MsgRow {
                payload: format!("msg-{msg_id}").into_bytes(),
                headers: MsgHeaders::new(),
                created_at: ts(msg_id as i64),
            };
            let key = msg_row_key(stream_id, msg_id);
            let val = row.to_fjall_value().unwrap();
            state.msg_table.insert(key, val).unwrap();
        }

        fn insert_msgs(state: &State, stream_id: StreamId, ids: &[MsgId]) {
            for &id in ids {
                insert_msg(state, stream_id, id);
            }
        }

        fn lease(stream_id: StreamId, block_start: MsgId, block_end: MsgId) -> LeaseRow {
            LeaseRow {
                stream_id,
                cg: "test-cg".to_string(),
                block_start,
                block_end,
                leased_at: ts(0),
                expires_at: ts(1000),
                acked_at: None,
                dlq_at: None,
            }
        }

        fn batch(n: usize) -> NonZeroUsize {
            NonZeroUsize::new(n).unwrap()
        }

        #[test]
        fn empty_stream_returns_empty() {
            let state = test_state("fetch_available_empty_stream");
            let stream_id = StreamId::new_v4();

            let result = MsgRow::fetch_available(&state, stream_id, [], batch(10)).unwrap();

            assert!(result.is_empty());
        }

        #[test]
        fn no_leases_returns_messages_in_order() {
            let state = test_state("fetch_available_no_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            let result = MsgRow::fetch_available(&state, stream_id, [], batch(10)).unwrap();

            assert_eq!(result.len(), 5);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 2);
            assert_eq!(result[2].0, 3);
            assert_eq!(result[3].0, 4);
            assert_eq!(result[4].0, 5);
        }

        #[test]
        fn batch_size_limits_results() {
            let state = test_state("fetch_available_batch_size_limits");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            let result = MsgRow::fetch_available(&state, stream_id, [], batch(3)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 2);
            assert_eq!(result[2].0, 3);
        }

        #[test]
        fn all_messages_leased_returns_empty() {
            let state = test_state("fetch_available_all_leased");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            let leases = vec![lease(stream_id, 1, 5)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert!(result.is_empty());
        }

        #[test]
        fn lease_at_start_skips_leased_messages() {
            let state = test_state("fetch_available_lease_at_start");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            let leases = vec![lease(stream_id, 1, 2)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 3);
            assert_eq!(result[1].0, 4);
            assert_eq!(result[2].0, 5);
        }

        #[test]
        fn lease_at_end_returns_messages_before_lease() {
            let state = test_state("fetch_available_lease_at_end");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            let leases = vec![lease(stream_id, 4, 5)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 2);
            assert_eq!(result[2].0, 3);
        }

        #[test]
        fn lease_in_middle_returns_messages_on_both_sides() {
            let state = test_state("fetch_available_lease_in_middle");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            let leases = vec![lease(stream_id, 2, 4)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 5);
        }

        #[test]
        fn multiple_leases_with_gap_returns_gap_messages() {
            let state = test_state("fetch_available_multiple_leases_gap");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            let leases = vec![lease(stream_id, 1, 3), lease(stream_id, 7, 10)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 4);
            assert_eq!(result[1].0, 5);
            assert_eq!(result[2].0, 6);
        }

        #[test]
        fn overlapping_leases_are_handled() {
            let state = test_state("fetch_available_overlapping_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            // Overlapping leases: 1-5 and 3-7
            let leases = vec![lease(stream_id, 1, 5), lease(stream_id, 3, 7)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 8);
            assert_eq!(result[1].0, 9);
            assert_eq!(result[2].0, 10);
        }

        #[test]
        fn adjacent_leases_block_full_range() {
            let state = test_state("fetch_available_adjacent_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            // Adjacent leases: 1-5 and 6-10
            let leases = vec![lease(stream_id, 1, 5), lease(stream_id, 6, 10)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert!(result.is_empty());
        }

        #[test]
        fn batch_size_respects_limit_with_leases() {
            let state = test_state("fetch_available_batch_with_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            // Lease at start
            let leases = vec![lease(stream_id, 1, 3)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(2)).unwrap();

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0, 4);
            assert_eq!(result[1].0, 5);
        }

        #[test]
        fn sparse_messages_with_leases() {
            let state = test_state("fetch_available_sparse_messages");
            let stream_id = StreamId::new_v4();
            // Sparse message IDs
            insert_msgs(&state, stream_id, &[10, 20, 30, 40, 50]);

            // Lease covers 15-35, blocking messages 20 and 30
            let leases = vec![lease(stream_id, 15, 35)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].0, 10);
            assert_eq!(result[1].0, 40);
            assert_eq!(result[2].0, 50);
        }

        #[test]
        fn single_message_leased() {
            let state = test_state("fetch_available_single_msg_leased");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            // Lease covers only message 3
            let leases = vec![lease(stream_id, 3, 3)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 4);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 2);
            assert_eq!(result[2].0, 4);
            assert_eq!(result[3].0, 5);
        }

        #[test]
        fn unsorted_leases_handled_correctly() {
            let state = test_state("fetch_available_unsorted_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            // Leases provided out of order
            let leases = vec![lease(stream_id, 7, 8), lease(stream_id, 2, 3)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 6);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 4);
            assert_eq!(result[2].0, 5);
            assert_eq!(result[3].0, 6);
            assert_eq!(result[4].0, 9);
            assert_eq!(result[5].0, 10);
        }

        #[test]
        fn returned_msg_payload_is_correct() {
            let state = test_state("fetch_available_payload_correct");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3]);

            let result = MsgRow::fetch_available(&state, stream_id, [], batch(10)).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result[0].1.payload, b"msg-1");
            assert_eq!(result[1].1.payload, b"msg-2");
            assert_eq!(result[2].1.payload, b"msg-3");
        }

        #[test]
        fn lease_boundary_exactly_at_message() {
            let state = test_state("fetch_available_boundary_exact");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            // Lease starts exactly at msg 2 and ends exactly at msg 4
            let leases = vec![lease(stream_id, 2, 4)];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 5);
        }

        #[test]
        fn many_small_leases() {
            let state = test_state("fetch_available_many_small_leases");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

            // Many single-message leases blocking even numbers
            let leases = vec![
                lease(stream_id, 2, 2),
                lease(stream_id, 4, 4),
                lease(stream_id, 6, 6),
                lease(stream_id, 8, 8),
                lease(stream_id, 10, 10),
            ];
            let result = MsgRow::fetch_available(&state, stream_id, &leases, batch(10)).unwrap();

            assert_eq!(result.len(), 5);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 3);
            assert_eq!(result[2].0, 5);
            assert_eq!(result[3].0, 7);
            assert_eq!(result[4].0, 9);
        }

        #[test]
        fn dlq_lease_blocks_message() {
            let state = test_state("fetch_available_dlq_blocks");
            let stream_id = StreamId::new_v4();
            insert_msgs(&state, stream_id, &[1, 2, 3, 4, 5]);

            // Create a DLQ lease for message 3
            let dlq_lease = LeaseRow {
                stream_id,
                cg: "test-cg".to_string(),
                block_start: 3,
                block_end: 3,
                leased_at: ts(0),
                expires_at: Timestamp::MAX,
                acked_at: None,
                dlq_at: Some(ts(100)),
            };
            let result =
                MsgRow::fetch_available(&state, stream_id, &[dlq_lease], batch(10)).unwrap();

            assert_eq!(result.len(), 4);
            assert_eq!(result[0].0, 1);
            assert_eq!(result[1].0, 2);
            assert_eq!(result[2].0, 4);
            assert_eq!(result[3].0, 5);
        }

        #[test]
        fn cull_and_compact_preserves_dlq_leases() {
            let stream_id = StreamId::new_v4();
            let dlq_lease = LeaseRow {
                stream_id,
                cg: "cg1".to_string(),
                block_start: 3,
                block_end: 3,
                leased_at: ts(0),
                expires_at: ts(50), // Even if expired time-wise, DLQ should be preserved
                acked_at: None,
                dlq_at: Some(ts(10)),
            };

            let diff = LeaseRow::cull_and_compact(vec![dlq_lease], ts(100));

            // DLQ lease should NOT be deleted even though it's "expired"
            assert!(diff.to_delete.is_empty());
            assert!(diff.to_insert.is_empty());
        }

        #[test]
        fn cull_and_compact_adjacent_dlq_leases_are_compacted() {
            let stream_id = StreamId::new_v4();
            let leases = vec![
                LeaseRow {
                    stream_id,
                    cg: "cg1".to_string(),
                    block_start: 1,
                    block_end: 3,
                    leased_at: ts(0),
                    expires_at: Timestamp::MAX,
                    acked_at: None,
                    dlq_at: Some(ts(10)),
                },
                LeaseRow {
                    stream_id,
                    cg: "cg1".to_string(),
                    block_start: 4,
                    block_end: 6,
                    leased_at: ts(5),
                    expires_at: Timestamp::MAX,
                    acked_at: None,
                    dlq_at: Some(ts(20)),
                },
            ];

            let diff = LeaseRow::cull_and_compact(leases, ts(100));

            assert_eq!(diff.to_delete.len(), 2);
            assert_eq!(diff.to_insert.len(), 1);

            let merged = &diff.to_insert[0];
            assert_eq!(merged.block_start, 1);
            assert_eq!(merged.block_end, 6);
            assert_eq!(merged.dlq_at, Some(ts(20)));
            assert!(merged.acked_at.is_none());
        }

        #[test]
        fn cull_and_compact_non_adjacent_dlq_leases_not_compacted() {
            let stream_id = StreamId::new_v4();
            let leases = vec![
                LeaseRow {
                    stream_id,
                    cg: "cg1".to_string(),
                    block_start: 1,
                    block_end: 3,
                    leased_at: ts(0),
                    expires_at: Timestamp::MAX,
                    acked_at: None,
                    dlq_at: Some(ts(10)),
                },
                LeaseRow {
                    stream_id,
                    cg: "cg1".to_string(),
                    block_start: 10,
                    block_end: 12,
                    leased_at: ts(5),
                    expires_at: Timestamp::MAX,
                    acked_at: None,
                    dlq_at: Some(ts(20)),
                },
            ];

            let diff = LeaseRow::cull_and_compact(leases, ts(100));

            assert!(diff.to_delete.is_empty());
            assert!(diff.to_insert.is_empty());
        }
    }
}
