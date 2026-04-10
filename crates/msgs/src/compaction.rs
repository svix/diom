//! [Compaction filters](https://fjall-rs.github.io/post/fjall-3-1/) for msgs metadata.
//!
//! Expired idempotency rows are dropped during LSM compactions instead of a scan-based
//! background job.

use diom_core::Monotime;
use fjall::compaction::filter::{
    CompactionFilter, CompactionFilterResult, Context, Factory, ItemAccessor, Verdict,
};
use fjall_utils::TableRow;
use jiff::Timestamp;

use crate::tables::IdempotencyRow;

struct IdempotencyExpiryFilter {
    now: Timestamp,
}

// Removes expired idempotency rows during compaction.
impl CompactionFilter for IdempotencyExpiryFilter {
    fn filter_item(&mut self, item: ItemAccessor<'_>, _ctx: &Context) -> CompactionFilterResult {
        let key = item.key();
        if key.first().copied() != Some(IdempotencyRow::ROW_TYPE) {
            return Ok(Verdict::Keep);
        }

        let value = item.value()?;
        let Ok(row) = IdempotencyRow::from_fjall_value(value) else {
            return Ok(Verdict::Keep);
        };

        if row.expiry < self.now {
            Ok(Verdict::Remove)
        } else {
            Ok(Verdict::Keep)
        }
    }
}

pub struct IdempotencyExpiryFilterFactory {
    time: Monotime,
}

impl IdempotencyExpiryFilterFactory {
    pub fn new(time: Monotime) -> Self {
        Self { time }
    }
}

impl Factory for IdempotencyExpiryFilterFactory {
    fn make_filter(&self, _ctx: &Context) -> Box<dyn CompactionFilter> {
        Box::new(IdempotencyExpiryFilter {
            now: self.time.now(), // snapshot time when compaction starts
        })
    }

    fn name(&self) -> &str {
        "msgs_idempotency_expiry"
    }
}

#[cfg(test)]
mod tests {
    use super::IdempotencyExpiryFilterFactory;
    use crate::METADATA_KEYSPACE;
    use crate::entities::{MsgsIdempotencyKey, TopicName};
    use crate::tables::{IdempotencyRow, TopicRow};
    use diom_core::Monotime;
    use diom_id::{NamespaceId, UuidV7RandomBytes};
    use fjall::{Database, KeyspaceCreateOptions};
    use fjall_utils::TableRow;
    use jiff::Timestamp;
    use std::sync::Arc;
    use test_utils::TestResult;

    #[test]
    fn compaction_drops_expired_idempotency() -> TestResult {
        let dir = tempfile::tempdir()?;
        let time = Monotime::initial();
        time.update_from_other(Timestamp::from_second(10)?);

        let db = Database::builder(dir.path())
            .temporary(true)
            .with_compaction_filter_factories(Arc::new(move |name| {
                (name == METADATA_KEYSPACE)
                    .then(|| Arc::new(IdempotencyExpiryFilterFactory::new(time.clone())) as _)
            }))
            .open()?;

        let ks = db.keyspace(METADATA_KEYSPACE, KeyspaceCreateOptions::default)?;

        let ns = NamespaceId::nil();
        let topic = TopicName::new("t".into())?;

        TopicRow::insert(
            &ks,
            TopicRow::key_for(ns, &topic),
            &TopicRow::new(
                topic.clone(),
                Timestamp::UNIX_EPOCH,
                UuidV7RandomBytes::new_random(),
            ),
        )?;

        let expired_key = MsgsIdempotencyKey::new(None, &topic, "old");
        IdempotencyRow::insert(
            &ks,
            IdempotencyRow::key_for(ns, &expired_key),
            &IdempotencyRow {
                expiry: Timestamp::from_second(1)?,
            },
        )?;

        let fresh_key = MsgsIdempotencyKey::new(None, &topic, "new");
        IdempotencyRow::insert(
            &ks,
            IdempotencyRow::key_for(ns, &fresh_key),
            &IdempotencyRow {
                expiry: Timestamp::from_second(20)?,
            },
        )?;

        // Force compaction to run
        ks.rotate_memtable_and_wait()?;
        ks.major_compact()?;

        assert!(TopicRow::fetch(&ks, TopicRow::key_for(ns, &topic))?.is_some());
        assert!(IdempotencyRow::fetch(&ks, IdempotencyRow::key_for(ns, &expired_key))?.is_none());
        assert!(IdempotencyRow::fetch(&ks, IdempotencyRow::key_for(ns, &fresh_key))?.is_some());

        Ok(())
    }
}
