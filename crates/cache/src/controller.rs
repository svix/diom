use diom_core::{
    instrumented_mutex::InstrumentedMutex, task::spawn_blocking_in_current_span, types::ByteString,
};
use diom_error::Result;
use diom_id::NamespaceId;
use fjall::{Database, Keyspace};
use fjall_utils::{SerializableKeyspaceCreateOptions, TableRow, WriteBatchExt};
use jiff::Timestamp;

use crate::{CacheModel, tables::CacheRow};

#[derive(Clone)]
pub struct CacheController {
    db: InstrumentedMutex<Database>,
    keyspace: Keyspace,
}

impl CacheController {
    pub fn new(db: Database, keyspace_name: &'static str) -> Self {
        let keyspace = SerializableKeyspaceCreateOptions::default()
            .with_default_kv_separation()
            .create_and_record(&db, keyspace_name)
            .expect("should be able to open keyspace");

        Self {
            db: InstrumentedMutex::new(db),
            keyspace,
        }
    }

    fn fetch_inner(
        keyspace: &Keyspace,
        namespace_id: NamespaceId,
        key: &str,
        now: Timestamp,
    ) -> Result<Option<CacheModel>> {
        let Some(row) = CacheRow::fetch(keyspace, CacheRow::key_for(namespace_id, key))? else {
            return Ok(None);
        };

        if row.expiry < now {
            return Ok(None);
        }

        Ok(Some(CacheModel {
            value: row.value,
            expiry: row.expiry,
        }))
    }

    #[tracing::instrument(skip_all)]
    pub async fn fetch<K: AsRef<str> + std::fmt::Debug + 'static + Send>(
        &self,
        namespace_id: NamespaceId,
        key: K,
        now: Timestamp,
    ) -> Result<Option<CacheModel>> {
        let keyspace = self.keyspace.clone();
        spawn_blocking_in_current_span(move || {
            Self::fetch_inner(&keyspace, namespace_id, key.as_ref(), now)
        })
        .await?
    }

    #[tracing::instrument(skip_all)]
    pub async fn set<K: AsRef<str> + std::fmt::Debug + 'static + Send>(
        &self,
        namespace_id: NamespaceId,
        key: K,
        value: ByteString,
        expiry: Timestamp,
    ) -> Result<()> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();

        spawn_blocking_in_current_span(move || {
            let key = key.as_ref();
            let row = CacheRow { value, expiry };
            let db = db.lock("cache_controller::set");
            let mut batch = db.batch();
            batch.insert_row(&keyspace, CacheRow::key_for(namespace_id, key), &row)?;
            batch.commit()?;
            Ok(())
        })
        .await?
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete<K: AsRef<str> + std::fmt::Debug + 'static + Send>(
        &self,
        namespace_id: NamespaceId,
        key: K,
        now: Timestamp,
    ) -> Result<bool> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();

        spawn_blocking_in_current_span(move || {
            let key = key.as_ref();
            if Self::fetch_inner(&keyspace, namespace_id, key, now)?.is_none() {
                return Ok(false);
            }
            let db = db.lock("cache_controller::delete");
            let mut batch = db.batch();
            batch.remove_row(&keyspace, CacheRow::key_for(namespace_id, key))?;
            batch.commit()?;
            Ok(true)
        })
        .await?
    }
}
