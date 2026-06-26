use diom_core::{
    PersistableValue,
    types::{AsMillisecond, ByteString, UnixTimestampMs},
};
use diom_id::{NamespaceId, TopicId, UuidV7RandomBytes};
use std::collections::HashMap;

use diom_error::{OptionExt, Result};
use fjall_utils::{FjallKey, TableRow, WriteBatchExt};
use serde::{Deserialize, Serialize};

use crate::entities::{
    ConsumerGroup, MsgId, MsgsIdempotencyKey, Offset, Partition, SvixPollerListItem, TopicName,
    obfuscate_token,
};

/// Prefixes for rows stored in the `metadata_tables` keyspace.
#[repr(u8)]
enum MetadataPrefix {
    Topic = 0,
    StreamLease = 1,
    QueueLease = 3,
    QueueConfig = 4,
    Idempotency = 5,
    SvixPoller = 6,
}

/// Prefixes for rows stored in the `msg_table` keyspace.
#[repr(u8)]
enum MsgPrefix {
    Msg = 2,
    HighWaterMark = 0,
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct TopicRow {
    pub id: TopicId,
    pub name: TopicName,
    pub partitions: u16,
}

impl TableRow for TopicRow {
    const ROW_TYPE: u8 = MetadataPrefix::Topic as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::Topic)]
pub(crate) struct TopicKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) topic: TopicName,
}

impl TopicRow {
    pub(crate) fn new(
        name: TopicName,
        now: impl AsMillisecond,
        id_random_bytes: UuidV7RandomBytes,
    ) -> Self {
        Self {
            id: TopicId::new(now, id_random_bytes),
            name,
            partitions: 1,
        }
    }

    pub(crate) fn partitions(&self) -> impl Iterator<Item = Result<Partition>> {
        (0..self.partitions).map(|i| Partition::new(i).ok_or_internal_error("partition overflow"))
    }

    pub(crate) fn partitions_shuffled(&self, seed: u64) -> Result<Vec<Partition>> {
        use rand::{SeedableRng, seq::SliceRandom};
        let mut list = (0..self.partitions)
            .map(|i| Partition::new(i).ok_or_internal_error("partition overflow"))
            .collect::<Result<Vec<_>>>()?;
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        list.shuffle(&mut rng);
        Ok(list)
    }

    /// Returns the existing row, or creates a new one and inserts it into the batch.
    pub(crate) fn fetch_or_create(
        metadata_tables: &fjall::Keyspace,
        batch: &mut fjall::OwnedWriteBatch,
        namespace_id: NamespaceId,
        topic: &TopicName,
        now: impl AsMillisecond,
        id_random_bytes: UuidV7RandomBytes,
    ) -> Result<Self> {
        let key = TopicKey::build_key(&namespace_id, topic);
        if let Some(row) = Self::fetch(metadata_tables, key.clone())? {
            return Ok(row);
        }
        let row = Self::new(topic.clone(), now, id_random_bytes);
        batch.insert_row(metadata_tables, key, &row)?;
        Ok(row)
    }
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct StreamLeaseRow {
    pub offset: u64,
    pub expiry: UnixTimestampMs,
    /// Last offset in the current leased batch. The lease is only released
    /// when the committed offset reaches this value.
    pub end_offset: Offset,
}

impl StreamLeaseRow {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            offset: 0,
            expiry: UnixTimestampMs::UNIX_EPOCH,
            end_offset: 0,
        })
    }
}

impl TableRow for StreamLeaseRow {
    const ROW_TYPE: u8 = MetadataPrefix::StreamLease as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::StreamLease)]
pub(crate) struct StreamLeaseKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) consumer_group: ConsumerGroup,
}

/// Per-message lease/ack tracking for queue semantics.
///
/// - `expiry > now` → message is leased (in-flight to a consumer)
/// - `expiry == UnixTimestampMs::MAX` → message is permanently acked
/// - `expiry <= now` → lease expired, message is available again
/// - No row → message was never leased, available
///
/// Rows below the queue cursor are deleted during cursor compaction to prevent unbounded growth.
#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct QueueLeaseRow {
    pub expiry: UnixTimestampMs,
    pub dlq: bool,
    pub attempt_count: u32,
}

impl QueueLeaseRow {
    /// Permanently acked — will never be re-delivered.
    pub(crate) fn acked() -> Self {
        Self {
            expiry: UnixTimestampMs::MAX,
            dlq: false,
            attempt_count: 0,
        }
    }

    /// Sent to the dead-letter queue.
    pub(crate) fn dlq_marker(attempt_count: u32) -> Self {
        Self {
            expiry: UnixTimestampMs::MAX,
            dlq: true,
            attempt_count,
        }
    }

    /// Writes an ack row into the batch, permanently marking the message as consumed.
    pub(crate) fn write_ack(
        batch: &mut fjall::OwnedWriteBatch,
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        msg_id: &MsgId,
        consumer_group: &ConsumerGroup,
    ) -> Result<()> {
        batch.insert_row(
            keyspace,
            QueueLeaseKey::build_key(&topic_id, &msg_id.partition, &msg_id.offset, consumer_group),
            &Self::acked(),
        )?;
        Ok(())
    }

    pub(crate) fn is_available(&self, now: UnixTimestampMs) -> bool {
        !self.dlq && self.expiry <= now
    }

    pub(crate) fn is_acked(&self) -> bool {
        !self.dlq && self.expiry == UnixTimestampMs::MAX
    }

    pub(crate) fn is_dlq(&self) -> bool {
        self.dlq
    }

    /// Returns all lease rows for a given (topic, partition, consumer_group) via prefix scan.
    pub(crate) fn scan_partition(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
        consumer_group: &ConsumerGroup,
    ) -> Result<Vec<(MsgId, Self)>> {
        let prefix = QueueLeaseKey::prefix_partition(&topic_id, &partition);
        let mut results = Vec::new();

        for guard in keyspace.prefix(&prefix) {
            let (key, val) = guard.into_inner()?;
            let cg = QueueLeaseKey::extract_consumer_group(&key)
                .expect("valid QueueLeaseKey in metadata table");
            if cg != *consumer_group {
                continue;
            }
            let offset =
                QueueLeaseKey::extract_offset(&key).expect("valid QueueLeaseKey in metadata table");

            let row = Self::from_fjall_value(val)?;
            results.push((MsgId::new(partition, offset), row));
        }

        Ok(results)
    }
}

impl TableRow for QueueLeaseRow {
    const ROW_TYPE: u8 = MetadataPrefix::QueueLease as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::QueueLease)]
pub(crate) struct QueueLeaseKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) offset: Offset,
    #[key(3)]
    pub(crate) consumer_group: ConsumerGroup,
}

/// Per-consumer-group queue configuration
#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct QueueConfigRow {
    pub retry_schedule: Vec<u64>,
    pub dlq_topic: Option<TopicName>,
}

impl TableRow for QueueConfigRow {
    const ROW_TYPE: u8 = MetadataPrefix::QueueConfig as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::QueueConfig)]
pub(crate) struct QueueConfigKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) consumer_group: ConsumerGroup,
}

#[derive(FjallKey)]
#[table_key(prefix = MsgPrefix::Msg)]
pub(crate) struct MsgKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
    #[key(2)]
    pub(crate) offset: Offset,
    #[key(3)]
    pub(crate) timestamp: UnixTimestampMs,
}

#[derive(Serialize, Deserialize, PersistableValue)]
pub(crate) struct MsgRow {
    pub value: ByteString,
    pub headers: HashMap<String, String>,
    pub timestamp: UnixTimestampMs,
    pub scheduled_at: Option<UnixTimestampMs>,
}

impl MsgRow {
    /// Returns the next offset to assign for a partition.
    ///
    /// Checks the message table first (backward scan for the last message).
    /// When the partition is empty (e.g. after retention cleanup), falls back
    /// to the persisted high-water mark.
    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn next_offset(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
    ) -> Result<Offset> {
        let range = MsgKey::prefix_partition(&topic_id, &partition);
        let item = keyspace.prefix(range).next_back();
        match item {
            Some(kv) => {
                let key = kv.key()?;
                let offset = MsgKey::extract_offset(&key).expect("valid MsgKey in msg table");
                Ok(offset + 1)
            }
            None => {
                let hwm = HighWaterMarkRow::fetch(
                    keyspace,
                    HighWaterMarkKey::build_key(&topic_id, &partition),
                )?;
                Ok(hwm.map(|h| h.next_offset).unwrap_or(0))
            }
        }
    }

    /// Finds the offset of the first message whose timestamp is >= `target_ts`.
    ///
    /// Returns `next_offset` if every message is older than the target (or the
    /// partition is empty), which positions the cursor at the tail — equivalent
    /// to a "latest" seek.
    ///
    /// Uses binary search over offsets, leveraging the invariant that timestamps
    /// increase monotonically with offsets.
    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn first_offset_at_or_after(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
        target_ts: UnixTimestampMs,
    ) -> Result<Offset> {
        let high = Self::next_offset(keyspace, topic_id, partition)?;
        if high == 0 {
            return Ok(0);
        }

        let mut lo: Offset = 0;
        let mut hi: Offset = high;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            let prefix = MsgKey::prefix_offset(&topic_id, &partition, &mid);
            let ts = if let Some(entry) = keyspace.prefix(prefix).next() {
                let k = entry.key()?;
                Some(MsgKey::extract_timestamp(&k).expect("valid MsgKey in msg table"))
            } else {
                None
            };

            match ts {
                Some(ts) if ts < target_ts => lo = mid + 1,
                Some(_) => hi = mid,
                // Offset has no entry (deleted by cleanup) — treat as older than target.
                None => lo = mid + 1,
            }
        }

        Ok(lo)
    }

    /// Fetch a single message by offset, skipping it if expired.
    #[tracing::instrument(skip_all, level = "debug")]
    pub(crate) fn fetch_by_offset(
        keyspace: &impl fjall_utils::ReadableKeyspace,
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
        expiry_cutoff: UnixTimestampMs,
    ) -> Result<Option<Self>> {
        let prefix = MsgKey::prefix_offset(&topic_id, &partition, &offset);
        for entry in keyspace.prefix(prefix) {
            let (_, val) = entry.into_inner_if(|k| {
                MsgKey::extract_timestamp(k).expect("valid MsgKey in msg table") >= expiry_cutoff
            })?;
            if let Some(v) = val {
                return Ok(Some(Self::from_fjall_value(v)?));
            }
        }
        Ok(None)
    }

    #[tracing::instrument(skip_all, level = "debug", fields(batch_size))]
    pub(crate) fn fetch_range(
        keyspace: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
        offset: Offset,
        batch_size: u16,
        expiry_cutoff: UnixTimestampMs,
    ) -> Result<Vec<(Offset, Self)>> {
        let mut results = Vec::with_capacity(batch_size as usize);
        let range = MsgKey::range(
            MsgKey {
                topic_id,
                partition,
                offset,
                timestamp: expiry_cutoff,
            }..MsgKey {
                topic_id,
                partition,
                offset: offset + batch_size as u64,
                timestamp: UnixTimestampMs::UNIX_EPOCH,
            },
        );
        for entry in keyspace.range(range) {
            let (key, val) = entry.into_inner_if(|k| {
                MsgKey::extract_timestamp(k).expect("valid MsgKey in msg table") >= expiry_cutoff
            })?;
            if let Some(val) = val {
                let msg_offset = MsgKey::extract_offset(&key).expect("valid MsgKey in msg table");
                let msg = Self::from_fjall_value(val)?;
                results.push((msg_offset, msg));
            }
        }

        tracing::Span::current().record("msgs_found", results.len());

        Ok(results)
    }
}

impl TableRow for MsgRow {
    const ROW_TYPE: u8 = MsgPrefix::Msg as u8;
}

const CLEANUP_BATCH_SIZE: usize = 1_000;

/// Deletes expired messages for a single topic partition, returning the number deleted.
///
/// Scans from the beginning of the partition and stops at the first non-expired message,
/// since timestamps increase monotonically with offsets.
#[tracing::instrument(skip_all, level = "debug")]
pub(crate) fn delete_expired_partition(
    db: &fjall::Database,
    msg_table: &fjall::Keyspace,
    topic_id: TopicId,
    partition: Partition,
    cutoff: UnixTimestampMs,
) -> Result<usize> {
    let prefix = MsgKey::prefix_partition(&topic_id, &partition);
    let mut deleted = 0;

    loop {
        let mut hit_non_expired = false;
        let mut keys = Vec::with_capacity(CLEANUP_BATCH_SIZE);

        for entry in msg_table.prefix(&prefix) {
            let k = entry.key()?;
            let ts = MsgKey::extract_timestamp(&k).expect("valid MsgKey in msg table");
            if ts >= cutoff {
                hit_non_expired = true;
                break;
            }
            keys.push(k);
            if keys.len() >= CLEANUP_BATCH_SIZE {
                break;
            }
        }

        if keys.is_empty() {
            break;
        }

        let batch_len = keys.len();
        let mut batch = db.batch();
        for key in keys {
            batch.remove(msg_table, key);
        }
        batch.commit().map_err(diom_error::Error::from)?;
        deleted += batch_len;

        if hit_non_expired || batch_len < CLEANUP_BATCH_SIZE {
            break;
        }
    }

    Ok(deleted)
}

/// Returns the offset of the earliest message in a partition, or `None` if the partition is empty.
#[tracing::instrument(skip_all, level = "debug")]
pub(crate) fn earliest_offset(
    msg_table: &impl fjall_utils::ReadableKeyspace,
    topic_id: TopicId,
    partition: Partition,
) -> Result<Option<Offset>> {
    let prefix = MsgKey::prefix_partition(&topic_id, &partition);
    match msg_table.prefix(prefix).next() {
        Some(kv) => {
            let key = kv.key()?;
            let offset = MsgKey::extract_offset(&key)
                .map_err(|e| diom_error::Error::internal(e.to_string()))?;
            Ok(Some(offset))
        }
        None => Ok(None),
    }
}

/// Deletes QueueLeaseRows for a partition whose offset is below `earliest_offset`.
#[tracing::instrument(skip_all, level = "debug")]
pub(crate) fn delete_stale_queue_leases(
    db: &fjall::Database,
    metadata_tables: &fjall::Keyspace,
    topic_id: TopicId,
    partition: Partition,
    earliest_offset: Offset,
) -> Result<usize> {
    let prefix = QueueLeaseKey::prefix_partition(&topic_id, &partition);
    let mut deleted = 0;

    loop {
        let mut keys = Vec::with_capacity(CLEANUP_BATCH_SIZE);

        for entry in metadata_tables.prefix(&prefix) {
            let k = entry.key()?;
            let offset = QueueLeaseKey::extract_offset(&k)
                .map_err(|e| diom_error::Error::internal(e.to_string()))?;
            if offset >= earliest_offset {
                break;
            }
            keys.push(k);
            if keys.len() >= CLEANUP_BATCH_SIZE {
                break;
            }
        }

        if keys.is_empty() {
            break;
        }

        let batch_len = keys.len();
        let mut batch = db.batch();
        for key in keys {
            batch.remove(metadata_tables, key);
        }
        batch.commit().map_err(diom_error::Error::from)?;
        deleted += batch_len;

        if batch_len < CLEANUP_BATCH_SIZE {
            break;
        }
    }

    Ok(deleted)
}

/// Deletes StreamLeaseRows for consumer groups on fully-empty partitions.
///
/// A stream lease is only considered stale when its partition has no messages
/// remaining (all deleted by retention) and a HWM exists proving messages
/// once existed. This avoids deleting leases for groups that are still
/// actively consuming from partitions with retained messages.
#[tracing::instrument(skip_all, level = "debug")]
pub(crate) fn delete_stale_stream_leases(
    metadata_tables: &fjall::Keyspace,
    msg_table: &impl fjall_utils::ReadableKeyspace,
    topic_id: TopicId,
) -> Result<usize> {
    let prefix = StreamLeaseKey::prefix_topic_id(&topic_id);
    let mut deleted = 0;

    for entry in metadata_tables.prefix(prefix) {
        let (key, val) = entry.into_inner()?;
        let partition = StreamLeaseKey::extract_partition(&key)
            .map_err(|e| diom_error::Error::internal(e.to_string()))?;
        let lease = StreamLeaseRow::from_fjall_value(val)?;

        // Only delete if partition is completely empty
        if earliest_offset(msg_table, topic_id, partition)?.is_some() {
            continue;
        }

        // Partition is empty — use HWM as cutoff. If no HWM either,
        // this is a truly empty partition that never had messages, skip.
        let hwm = HighWaterMarkRow::fetch(
            msg_table,
            HighWaterMarkKey::build_key(&topic_id, &partition),
        )?;
        let Some(h) = hwm else { continue };

        if lease.offset < h.next_offset {
            metadata_tables.remove(key)?;
            deleted += 1;
        }
    }

    Ok(deleted)
}

#[derive(Clone, Serialize, Deserialize, PersistableValue)]
pub(crate) struct IdempotencyRow {
    pub expiry: UnixTimestampMs,
}

impl TableRow for IdempotencyRow {
    const ROW_TYPE: u8 = MetadataPrefix::Idempotency as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::Idempotency)]
pub(crate) struct IdempotencyKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) key: MsgsIdempotencyKey,
}

/// Persists the high-water mark (next offset to assign) for a partition.
///
/// Survives message deletion so that offsets remain monotonically increasing
/// even after all messages in a partition have been removed by retention cleanup.
#[derive(Clone, Serialize, Deserialize, PersistableValue)]
pub(crate) struct HighWaterMarkRow {
    pub next_offset: Offset,
}

impl TableRow for HighWaterMarkRow {
    const ROW_TYPE: u8 = MsgPrefix::HighWaterMark as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MsgPrefix::HighWaterMark)]
pub(crate) struct HighWaterMarkKey {
    #[key(0)]
    pub(crate) topic_id: TopicId,
    #[key(1)]
    pub(crate) partition: Partition,
}

// --- Svix Poller configuration ---

#[derive(Clone, Serialize, Deserialize, PersistableValue)]
pub(crate) struct SvixPollerRow {
    // Technically redundant with the TopicId, but saving the name
    // let's us avoid a costly lookup of the name in the background worker.
    pub topic: TopicName,
    pub token: String,
}

impl TableRow for SvixPollerRow {
    const ROW_TYPE: u8 = MetadataPrefix::SvixPoller as u8;
}

#[derive(FjallKey)]
#[table_key(prefix = MetadataPrefix::SvixPoller)]
pub(crate) struct SvixPollerKey {
    #[key(0)]
    pub(crate) namespace_id: NamespaceId,
    #[key(1)]
    pub(crate) topic_id: TopicId,
    #[key(2)]
    pub(crate) poller_id: String,
}

impl SvixPollerKey {
    /// The consumer id used when polling and committing against the Svix
    /// Autoconfig endpoint for this poller.
    ///
    /// IMPORTANT: changes to may break extant autoconfig integrations, since the
    /// consumer group is how svix tracks state positions.
    pub(crate) fn consumer_id(&self) -> String {
        format!("diom_{}", self.poller_id)
    }
}

/// Lists Svix poller configurations for a topic, ordered by poller id.
///
/// Returns up to `limit` items whose poller id sorts after `iterator`
/// (exclusive). An unknown topic yields an empty list. Result tokens are
/// obfuscated so the stored secret is never returned in full.
#[tracing::instrument(skip(metadata_tables))]
pub fn list_svix_pollers(
    metadata_tables: &impl fjall_utils::ReadableKeyspace,
    namespace_id: NamespaceId,
    topic: &TopicName,
    limit: usize,
    iterator: Option<&str>,
) -> Result<Vec<SvixPollerListItem>> {
    let Some(topic_row) =
        TopicRow::fetch(metadata_tables, TopicKey::build_key(&namespace_id, topic))?
    else {
        return Ok(Vec::new());
    };

    let prefix = SvixPollerKey::prefix_topic_id(&namespace_id, &topic_row.id);
    let iterator = iterator.map(|poller_id| {
        SvixPollerKey::build_key(&namespace_id, &topic_row.id, poller_id).to_vec()
    });

    SvixPollerRow::list_range(metadata_tables, &prefix, iterator, limit)?
        .into_iter()
        .map(|(key, row)| {
            let poller_id = SvixPollerKey::extract_poller_id(&key)
                .map_err(|e| diom_error::Error::internal(e.to_string()))?
                .to_owned();
            Ok(SvixPollerListItem {
                topic: row.topic,
                poller_id,
                token: obfuscate_token(&row.token),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::entities::{ConsumerGroup, Partition};

    fn ts(millis: i64) -> UnixTimestampMs {
        UnixTimestampMs::try_from_millisecond(millis).unwrap()
    }

    /// Helper: insert a message at a given offset and timestamp.
    fn insert_msg(
        db: &fjall::Database,
        ks: &fjall::Keyspace,
        topic_id: TopicId,
        partition: Partition,
        offset: u64,
        timestamp: UnixTimestampMs,
    ) {
        use fjall_utils::WriteBatchExt as _;
        let mut batch = db.batch();
        batch
            .insert_row(
                ks,
                MsgKey {
                    topic_id,
                    partition,
                    offset,
                    timestamp,
                },
                &MsgRow {
                    value: b"msg".into(),
                    headers: HashMap::new(),
                    timestamp,
                    scheduled_at: None,
                },
            )
            .unwrap();
        batch.commit().unwrap();
    }

    #[test]
    fn first_offset_at_or_after_binary_search() {
        use TopicId;

        let dir = tempfile::tempdir().unwrap();
        let db = fjall::Database::builder(dir.path())
            .temporary(true)
            .open()
            .unwrap();
        let ks = db
            .keyspace("msgs", fjall::KeyspaceCreateOptions::default)
            .unwrap();
        let topic_id = TopicId::new(ts(0), UuidV7RandomBytes::new_random());
        let p = Partition::ZERO;

        // Empty partition returns 0
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(5000)).unwrap(),
            0
        );

        // Insert messages: offset 0 @ t=1000, 1 @ t=2000, 2 @ t=3000, 3 @ t=5000
        insert_msg(&db, &ks, topic_id, p, 0, ts(1000));
        insert_msg(&db, &ks, topic_id, p, 1, ts(2000));
        insert_msg(&db, &ks, topic_id, p, 2, ts(3000));
        insert_msg(&db, &ks, topic_id, p, 3, ts(5000));

        // Before all -> offset 0
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(0)).unwrap(),
            0
        );
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(500)).unwrap(),
            0
        );

        // Exact match on first -> offset 0
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(1000)).unwrap(),
            0
        );

        // Between t=1000 and t=2000 -> offset 1
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(1500)).unwrap(),
            1
        );

        // Exact match on middle-> offset 2
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(3000)).unwrap(),
            2
        );

        // Between t=3000 and t=5000 -> offset 3
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(4000)).unwrap(),
            3
        );

        // Exact match on last -> offset 3
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(5000)).unwrap(),
            3
        );

        // After all-> next_offset (4)
        assert_eq!(
            MsgRow::first_offset_at_or_after(&ks, topic_id, p, ts(9999)).unwrap(),
            4
        );
    }

    #[test]
    fn test_consumer_group_from_key() {
        use TopicId;
        let topic_id = TopicId::new(UnixTimestampMs::UNIX_EPOCH, UuidV7RandomBytes::new_random());
        let partition = Partition::ZERO;
        let cg = ConsumerGroup::try_from("my-group").unwrap();
        let key = StreamLeaseKey::build_key(&topic_id, &partition, &cg);
        assert_eq!(
            &*StreamLeaseKey::extract_consumer_group(&key).unwrap(),
            "my-group"
        );
    }

    #[test]
    fn fetch_range_filters_expired_messages() {
        use std::collections::HashMap;

        use TopicId;
        use fjall_utils::WriteBatchExt as _;

        let dir = tempfile::tempdir().unwrap();
        let db = fjall::Database::builder(dir.path())
            .temporary(true)
            .open()
            .unwrap();
        let ks = db
            .keyspace("msgs", fjall::KeyspaceCreateOptions::default)
            .unwrap();
        let topic_id = TopicId::new(UnixTimestampMs::UNIX_EPOCH, UuidV7RandomBytes::new_random());
        let partition = Partition::ZERO;

        let t1 = UnixTimestampMs::try_from_millisecond(1000).unwrap();
        let t2 = UnixTimestampMs::try_from_millisecond(2000).unwrap();
        let t3 = UnixTimestampMs::try_from_millisecond(3000).unwrap();

        for (ts, offset) in [(t1, 0), (t2, 1), (t3, 2)] {
            let mut batch = db.batch();
            batch
                .insert_row(
                    &ks,
                    MsgKey {
                        topic_id,
                        partition,
                        timestamp: ts,
                        offset,
                    },
                    &MsgRow {
                        value: b"msg".into(),
                        headers: HashMap::new(),
                        timestamp: ts,
                        scheduled_at: None,
                    },
                )
                .unwrap();
            batch.commit().unwrap();
        }

        // No expiry: all 3 messages returned
        let msgs =
            MsgRow::fetch_range(&ks, topic_id, partition, 0, 10, UnixTimestampMs::UNIX_EPOCH)
                .unwrap();
        assert_eq!(msgs.len(), 3, "no expiry should return all messages");

        // Cutoff at 1500: only t2 and t3 survive
        let cutoff = UnixTimestampMs::try_from_millisecond(1500).unwrap();
        let msgs = MsgRow::fetch_range(&ks, topic_id, partition, 0, 10, cutoff).unwrap();
        assert_eq!(msgs.len(), 2, "cutoff should filter out the oldest message");

        // Cutoff past all: nothing returned
        let cutoff = UnixTimestampMs::try_from_millisecond(5000).unwrap();
        let msgs = MsgRow::fetch_range(&ks, topic_id, partition, 0, 10, cutoff).unwrap();
        assert_eq!(msgs.len(), 0, "cutoff past all should return nothing");

        // fetch_by_offset: non-expired
        let msg = MsgRow::fetch_by_offset(&ks, topic_id, partition, 1, UnixTimestampMs::UNIX_EPOCH)
            .unwrap();
        assert!(msg.is_some());

        // fetch_by_offset: expired
        let cutoff = UnixTimestampMs::try_from_millisecond(1500).unwrap();
        let msg = MsgRow::fetch_by_offset(&ks, topic_id, partition, 0, cutoff).unwrap();
        assert!(msg.is_none(), "expired message should not be returned");
    }
}
