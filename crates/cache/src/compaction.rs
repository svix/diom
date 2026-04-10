//! [Compaction filters](https://fjall-rs.github.io/post/fjall-3-1/) for cache key expiry.
//!
//! Expired cache rows are dropped during LSM compactions instead of a scan-based
//! background job.

use diom_core::Monotime;
use fjall::compaction::filter::{
    CompactionFilter, CompactionFilterResult, Context, Factory, ItemAccessor, Verdict,
};
use fjall_utils::TableRow;
use jiff::Timestamp;

use crate::tables::CacheRow;

struct CacheExpiryFilter {
    now: Timestamp,
}

impl CompactionFilter for CacheExpiryFilter {
    fn filter_item(&mut self, item: ItemAccessor<'_>, _ctx: &Context) -> CompactionFilterResult {
        let key = item.key();
        if key.first().copied() != Some(CacheRow::ROW_TYPE) {
            return Ok(Verdict::Keep);
        }

        let value = item.value()?;
        let Ok(row) = CacheRow::from_fjall_value(value) else {
            tracing::error!("failed to deserialize row during compaction");
            return Ok(Verdict::Remove);
        };

        if row.expiry < self.now {
            Ok(Verdict::Remove)
        } else {
            Ok(Verdict::Keep)
        }
    }
}

pub struct CacheExpiryFilterFactory {
    time: Monotime,
}

impl CacheExpiryFilterFactory {
    pub fn new(time: Monotime) -> Self {
        Self { time }
    }
}

impl Factory for CacheExpiryFilterFactory {
    fn make_filter(&self, _ctx: &Context) -> Box<dyn CompactionFilter> {
        Box::new(CacheExpiryFilter {
            now: self.time.now(), // snapshot time when compaction starts
        })
    }

    fn name(&self) -> &str {
        "cache_expiry"
    }
}

#[cfg(test)]
mod tests {
    use super::CacheExpiryFilterFactory;
    use crate::{CACHE_KEYSPACE, tables::CacheRow};
    use diom_core::{Monotime, types::ByteString};
    use diom_id::NamespaceId;
    use fjall::{Database, KeyspaceCreateOptions};
    use fjall_utils::TableRow;
    use jiff::Timestamp;
    use std::sync::Arc;
    use test_utils::TestResult;

    #[test]
    fn compaction_drops_expired_cache_entries() -> TestResult {
        let dir = tempfile::tempdir()?;
        let time = Monotime::initial();
        time.update_from_other(Timestamp::from_second(10)?);

        let db = Database::builder(dir.path())
            .temporary(true)
            .with_compaction_filter_factories(Arc::new(move |name| {
                (name == CACHE_KEYSPACE)
                    .then(|| Arc::new(CacheExpiryFilterFactory::new(time.clone())) as _)
            }))
            .open()?;

        let ks = db.keyspace(CACHE_KEYSPACE, KeyspaceCreateOptions::default)?;
        let ns = NamespaceId::nil();

        // Expired entry (expiry=1s, now=10s)
        CacheRow::insert(
            &ks,
            CacheRow::key_for(ns, "expired"),
            &CacheRow {
                value: ByteString::from(b"old".as_slice()),
                expiry: Timestamp::from_second(1)?,
            },
        )?;

        // Fresh entry (expiry=20s, now=10s)
        CacheRow::insert(
            &ks,
            CacheRow::key_for(ns, "fresh"),
            &CacheRow {
                value: ByteString::from(b"new".as_slice()),
                expiry: Timestamp::from_second(20)?,
            },
        )?;

        // Force compaction to run
        ks.rotate_memtable_and_wait()?;
        ks.major_compact()?;

        assert!(CacheRow::fetch(&ks, CacheRow::key_for(ns, "expired"))?.is_none());
        assert!(CacheRow::fetch(&ks, CacheRow::key_for(ns, "fresh"))?.is_some());

        Ok(())
    }
}
