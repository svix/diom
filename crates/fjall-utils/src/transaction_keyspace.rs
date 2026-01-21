//! Everything in this module is to make mirror the API for transactions in [fjall], except transactions are scoped to a single keyspace.
//! This avoids the pain of having to manually carry around a OptimisticTxDatabase handle, or manually tying transaction inserts/reads to
//! specific Keyspace.
//!
//! Pros: much more ergonomic if you only need a transaction for a single keyspace.
//!
//! Cons: doesn't support multikeyspace transaction ops.
use fjall::{
    Conflict, Keyspace, KeyspaceCreateOptions, OptimisticTxDatabase, OptimisticTxKeyspace,
    OptimisticWriteTx, PersistMode, Result, UserKey, UserValue,
};

#[derive(Clone)]
/// Wraps a [fjall::OptimisticTxKeyspace], except this makes it much more ergonomic to create transactions that are scoped to a single Keyspace.
pub struct ErgonomicOptimisticTxKeyspace {
    db: OptimisticTxDatabase,
    ks: OptimisticTxKeyspace,
}

impl ErgonomicOptimisticTxKeyspace {
    pub fn new(db: OptimisticTxDatabase, name: &str, opts: KeyspaceCreateOptions) -> Result<Self> {
        let ks = db.keyspace(name, || opts)?;
        Ok(Self { db, ks })
    }

    pub fn write_tx(&self) -> Result<SingleKeyspaceOptimisticWriteTx<'_>> {
        let tx = self.db.write_tx()?;
        Ok(SingleKeyspaceOptimisticWriteTx { ks: &self.ks, tx })
    }
}

impl AsRef<Keyspace> for ErgonomicOptimisticTxKeyspace {
    fn as_ref(&self) -> &Keyspace {
        self.ks.as_ref()
    }
}

/// Wraps a [fjall::OptimisticWriteTx], except this is only associated with a single keyspace.
///
/// This makes method calls a bit more ergonomic, as you don't have to manually pass in the correct keyspace.
pub struct SingleKeyspaceOptimisticWriteTx<'a> {
    ks: &'a OptimisticTxKeyspace,
    tx: OptimisticWriteTx,
}

impl<'a> SingleKeyspaceOptimisticWriteTx<'a> {
    pub fn durability(self, mode: Option<PersistMode>) -> Self {
        Self {
            ks: self.ks,
            tx: self.tx.durability(mode),
        }
    }

    pub fn take<K: Into<UserKey>>(&mut self, key: K) -> Result<Option<UserValue>> {
        self.tx.take(self.ks, key)
    }

    pub fn update_fetch<K: Into<UserKey>, F: FnMut(Option<&UserValue>) -> Option<UserValue>>(
        &mut self,
        key: K,
        f: F,
    ) -> Result<Option<UserValue>> {
        self.tx.update_fetch(self.ks, key, f)
    }

    pub fn fetch_update<K: Into<UserKey>, F: FnMut(Option<&UserValue>) -> Option<UserValue>>(
        &mut self,
        key: K,
        f: F,
    ) -> Result<Option<UserValue>> {
        self.tx.fetch_update(self.ks, key, f)
    }

    pub fn insert<K: Into<UserKey>, V: Into<UserValue>>(&mut self, key: K, value: V) {
        self.tx.insert(self.ks, key, value)
    }

    pub fn remove<K: Into<UserKey>>(&mut self, key: K) {
        self.tx.remove(self.ks, key)
    }

    pub fn commit(self) -> Result<std::result::Result<(), Conflict>> {
        self.tx.commit()
    }

    pub fn rollback(self) {
        self.tx.rollback()
    }
}
