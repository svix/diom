use std::{borrow::Cow, collections::HashMap, ops::RangeInclusive};

use coyote_error::{Error, Result};
use coyote_namespace::entities::NamespaceId;
use fjall::OwnedWriteBatch;
use fjall_utils::{TableKey, TableRow};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ConsumerGroup, Offset, PartitionIndex},
};

#[derive(Serialize, Deserialize)]
pub(crate) struct MsgRow {
    pub value: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub created_at: Timestamp,
}

const MSG_KEY_LEN: usize = size_of::<NamespaceId>() + size_of::<u16>() + size_of::<Offset>();

type MsgRowKey = [u8; MSG_KEY_LEN];

pub(crate) fn msg_row_key(
    ns_id: NamespaceId,
    partition: PartitionIndex,
    offset: Offset,
) -> MsgRowKey {
    let mut key = [0u8; MSG_KEY_LEN];
    let ns_bytes = ns_id.as_u128().to_be_bytes();
    key[..16].copy_from_slice(&ns_bytes);
    key[16..18].copy_from_slice(&partition.get().to_be_bytes());
    key[18..26].copy_from_slice(&offset.to_be_bytes());
    key
}

fn parse_msg_offset(key: MsgRowKey) -> Result<Offset> {
    let bytes: [u8; 8] = key[18..26].try_into().map_err(Error::generic)?;
    Ok(Offset::from_be_bytes(bytes))
}

pub(crate) fn msg_row_key_range(
    ns_id: NamespaceId,
    partition: PartitionIndex,
    offsets: RangeInclusive<Offset>,
) -> RangeInclusive<MsgRowKey> {
    msg_row_key(ns_id, partition, *offsets.start())..=msg_row_key(ns_id, partition, *offsets.end())
}

impl MsgRow {
    pub(crate) fn max_offset(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
    ) -> Result<Option<Offset>> {
        let range = msg_row_key_range(ns_id, partition, Offset::MIN..=Offset::MAX);

        let Some(max_entry) = state.msg_table.range(range).next_back() else {
            return Ok(None);
        };

        let key: MsgRowKey = max_entry
            .key()?
            .as_ref()
            .try_into()
            .map_err(Error::generic)?;

        let offset = parse_msg_offset(key)?;
        Ok(Some(offset))
    }

    pub(crate) fn next_offset(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
    ) -> Result<Offset> {
        match Self::max_offset(state, ns_id, partition)? {
            None => Ok(Offset::MIN),
            Some(id) => Ok(id + 1),
        }
    }

    pub(crate) fn to_fjall_value(&self) -> Result<fjall::UserValue> {
        rmp_serde::to_vec(&self)
            .map(fjall::UserValue::from)
            .map_err(Error::generic)
    }

    fn fetch_in_range(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
        offsets: RangeInclusive<Offset>,
        buf: &mut Vec<(Offset, MsgRow)>,
        batch_size: usize,
    ) -> Result<usize> {
        let range = msg_row_key_range(ns_id, partition, offsets);
        let mut n = 0;

        for entry in state.msg_table.range(range).take(batch_size) {
            let (key_slice, val_slice) = entry.into_inner()?;
            let key: MsgRowKey = key_slice.as_ref().try_into().map_err(Error::generic)?;
            let offset = parse_msg_offset(key)?;
            let row: MsgRow = rmp_serde::from_slice(&val_slice).map_err(Error::generic)?;
            buf.push((offset, row));
            n += 1;
        }

        Ok(n)
    }

    /// Fetch up to `batch_size` available messages from a single partition, starting from
    /// `start_offset`. Messages covered by the given leases are excluded.
    pub(crate) fn fetch_available<'a>(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
        start_offset: Offset,
        blocked_leases: impl IntoIterator<Item = &'a LeaseRow>,
        batch_size: usize,
    ) -> Result<Vec<(Offset, MsgRow)>> {
        let blocked_ranges = merge_lease_ranges(blocked_leases.into_iter());
        let mut msgs_left = batch_size;
        let mut results = Vec::with_capacity(msgs_left);

        let mut min = start_offset;
        for block in blocked_ranges {
            // Skip blocks that end before our start position.
            if block.end < min {
                continue;
            }
            if block.start > min {
                let max = block.start - 1;
                let n = Self::fetch_in_range(
                    state,
                    ns_id,
                    partition,
                    min..=max,
                    &mut results,
                    msgs_left,
                )?;
                msgs_left -= n;
                if msgs_left == 0 {
                    return Ok(results);
                }
            }
            min = block.end + 1;
        }

        if msgs_left > 0 {
            Self::fetch_in_range(
                state,
                ns_id,
                partition,
                min..=Offset::MAX,
                &mut results,
                msgs_left,
            )?;
        }

        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// LeaseRow
// ---------------------------------------------------------------------------

/// A lease represents a consumer group's hold on a block of messages within a single partition.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct LeaseRow {
    pub namespace_id: NamespaceId,
    pub partition: PartitionIndex,
    pub cg: ConsumerGroup,
    pub block_start: Offset,
    pub block_end: Offset,
    pub leased_at: Timestamp,
    pub expires_at: Timestamp,
    #[serde(default)]
    pub acked_at: Option<Timestamp>,
    #[serde(default)]
    pub dlq_at: Option<Timestamp>,
}

/// Composite key: `[ns_id(16B)][partition(2B)][\0][cg_bytes][\0][block_end(8B)]`.
#[derive(Clone)]
pub(crate) struct LeaseKey(Vec<u8>);

impl LeaseKey {
    pub(crate) fn new(
        id: NamespaceId,
        partition: PartitionIndex,
        cg: &ConsumerGroup,
        block_end: Offset,
    ) -> Self {
        let id_bytes = id.as_u128().to_be_bytes();
        let part_bytes = partition.get().to_be_bytes();
        let end_bytes = block_end.to_be_bytes();

        let mut key = Vec::with_capacity(
            id_bytes.len() + part_bytes.len() + 1 + cg.len() + 1 + end_bytes.len(),
        );
        key.extend_from_slice(&id_bytes);
        key.extend_from_slice(&part_bytes);
        key.extend_from_slice(b"\0");
        key.extend_from_slice(cg.as_bytes());
        key.extend_from_slice(b"\0");
        key.extend_from_slice(&end_bytes);

        Self(key)
    }

    fn prefix(id: NamespaceId, partition: PartitionIndex, cg: &ConsumerGroup) -> Vec<u8> {
        let id_bytes = id.as_u128().to_be_bytes();
        let part_bytes = partition.get().to_be_bytes();

        let mut prefix = Vec::with_capacity(
            LeaseRow::TABLE_PREFIX.len() + 1 + id_bytes.len() + part_bytes.len() + 1 + cg.len() + 1,
        );
        prefix.extend_from_slice(LeaseRow::TABLE_PREFIX.as_bytes());
        prefix.extend_from_slice(b"\0");
        prefix.extend_from_slice(&id_bytes);
        prefix.extend_from_slice(&part_bytes);
        prefix.extend_from_slice(b"\0");
        prefix.extend_from_slice(cg.as_bytes());
        prefix.extend_from_slice(b"\0");

        prefix
    }
}

impl TableKey for LeaseKey {
    fn as_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self(bytes.to_owned()))
    }
}

impl TableRow for LeaseRow {
    const TABLE_PREFIX: &'static str = "_MSGLEASE_";

    type Key = LeaseKey;

    fn get_key(&self) -> Cow<'_, Self::Key> {
        Cow::Owned(LeaseKey::new(
            self.namespace_id,
            self.partition,
            &self.cg,
            self.block_end,
        ))
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
    pub(crate) fn fetch_all(
        state: &State,
        id: NamespaceId,
        partition: PartitionIndex,
        cg: &ConsumerGroup,
    ) -> Result<Vec<Self>> {
        let prefix = LeaseKey::prefix(id, partition, cg);

        state
            .metadata_tables
            .prefix(prefix)
            .map(|entry| {
                let value = entry.value()?;
                Self::from_fjall_value(value)
            })
            .collect()
    }

    /// Cull expired leases and compact adjacent acked/DLQ'd leases.
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

            let namespace_id = group[0].namespace_id;
            let partition = group[0].partition;
            let cg = group[0].cg.clone();

            let mut merged = LeaseRow {
                namespace_id,
                partition,
                cg,
                block_start: Offset::MAX,
                block_end: Offset::MIN,
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

    /// Shrinks or splits active leases to exclude a range of offsets.
    // FIXME(@svix-gabriel): Used by stream.commit / queue.ack (PRs 4-5).
    #[allow(dead_code)]
    pub(crate) fn shrink_active_leases_for_range(
        leases: &[Self],
        min_offset: Offset,
        max_offset: Offset,
        now: Timestamp,
        lease_diff: &mut LeaseDiff,
    ) {
        for lease in leases {
            if !lease.is_active(now) {
                continue;
            }

            if lease.block_end < min_offset || max_offset < lease.block_start {
                continue;
            }

            lease_diff.to_delete.push(lease.clone());

            if lease.block_start < min_offset {
                lease_diff.to_insert.push(Self {
                    block_end: min_offset - 1,
                    ..lease.clone()
                });
            }

            if lease.block_end > max_offset {
                lease_diff.to_insert.push(Self {
                    block_start: max_offset + 1,
                    ..lease.clone()
                });
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Blocked ranges (used by fetch_available)
// ---------------------------------------------------------------------------

struct BlockedRange {
    start: Offset,
    end: Offset,
}

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

// ---------------------------------------------------------------------------
// OffsetRow — committed stream offset per (namespace, partition, consumer_group)
// ---------------------------------------------------------------------------

const OFFSET_PREFIX: &[u8] = b"_MSGOFFSET_\0";

fn offset_key(ns_id: NamespaceId, partition: PartitionIndex, cg: &ConsumerGroup) -> Vec<u8> {
    let mut key = Vec::with_capacity(OFFSET_PREFIX.len() + 16 + 2 + 1 + cg.len());
    key.extend_from_slice(OFFSET_PREFIX);
    key.extend_from_slice(&ns_id.as_u128().to_be_bytes());
    key.extend_from_slice(&partition.get().to_be_bytes());
    key.extend_from_slice(b"\0");
    key.extend_from_slice(cg.as_bytes());
    key
}

pub(crate) struct OffsetRow;

impl OffsetRow {
    pub(crate) fn fetch(
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
        cg: &ConsumerGroup,
    ) -> Result<Option<Offset>> {
        let key = offset_key(ns_id, partition, cg);
        match state.metadata_tables.get(&key)? {
            Some(val) => {
                let offset: Offset = rmp_serde::from_slice(&val).map_err(Error::generic)?;
                Ok(Some(offset))
            }
            None => Ok(None),
        }
    }

    // FIXME(@svix-gabriel): Used by stream.commit (PR 4).
    #[allow(dead_code)]
    pub(crate) fn store(
        batch: &mut OwnedWriteBatch,
        state: &State,
        ns_id: NamespaceId,
        partition: PartitionIndex,
        cg: &ConsumerGroup,
        offset: Offset,
    ) -> Result<()> {
        let key = offset_key(ns_id, partition, cg);
        let val: Vec<u8> = rmp_serde::to_vec(&offset).map_err(Error::generic)?;
        let val: fjall::UserValue = val.into();
        batch.insert(&state.metadata_tables, key, val);
        Ok(())
    }
}

// Compile-time check that table prefixes used in the metadata keyspace are unique.
static_assertions::const_assert!(fjall_utils::are_all_unique(&[
    LeaseRow::TABLE_PREFIX,
    "_MSGOFFSET_",
]));
