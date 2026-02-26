use fjall::{Database, Keyspace, KeyspaceCreateOptions};
use std::ops::RangeBounds;

/// A wrapper around a Fjall database that only exposes readonly actions
pub struct ReadonlyDatabase {
    inner: Database,
}

impl ReadonlyDatabase {
    pub fn new(inner: Database) -> Self {
        Self { inner }
    }
}

pub trait ReadableDatabase {
    type Keyspace: ReadableKeyspace;

    #[must_use]
    fn snapshot(&self) -> fjall::Snapshot;

    fn keyspace(&self, name: &str) -> fjall::Result<Self::Keyspace>;
}

impl ReadableDatabase for Database {
    type Keyspace = Keyspace;

    fn snapshot(&self) -> fjall::Snapshot {
        self.snapshot()
    }

    fn keyspace(&self, name: &str) -> fjall::Result<Self::Keyspace> {
        self.keyspace(name, KeyspaceCreateOptions::default)
    }
}

impl ReadableDatabase for ReadonlyDatabase {
    type Keyspace = ReadonlyKeyspace;

    fn snapshot(&self) -> fjall::Snapshot {
        self.inner.snapshot()
    }

    fn keyspace(&self, name: &str) -> fjall::Result<Self::Keyspace> {
        self.inner
            .keyspace(name, KeyspaceCreateOptions::default)
            .map(|inner| ReadonlyKeyspace { inner })
    }
}

pub trait ReadableKeyspace {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<fjall::UserValue>>;

    fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<bool>;

    fn size_of<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<u32>>;

    fn range<K: AsRef<[u8]>, R: RangeBounds<K>>(&self, range: R) -> fjall::Iter;

    fn prefix<K: AsRef<[u8]>>(&self, prefix: K) -> fjall::Iter;
}

impl ReadableKeyspace for Keyspace {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<fjall::UserValue>> {
        self.get(key)
    }

    fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<bool> {
        self.contains_key(key)
    }

    fn size_of<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<u32>> {
        self.size_of(key)
    }

    fn range<K: AsRef<[u8]>, R: RangeBounds<K>>(&self, range: R) -> fjall::Iter {
        self.range(range)
    }

    fn prefix<K: AsRef<[u8]>>(&self, prefix: K) -> fjall::Iter {
        self.prefix(prefix)
    }
}

#[derive(Clone)]
pub struct ReadonlyKeyspace {
    inner: Keyspace,
}

impl From<Keyspace> for ReadonlyKeyspace {
    fn from(inner: Keyspace) -> Self {
        Self { inner }
    }
}

impl ReadableKeyspace for ReadonlyKeyspace {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<fjall::UserValue>> {
        self.inner.get(key)
    }

    fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<bool> {
        self.inner.contains_key(key)
    }

    fn size_of<K: AsRef<[u8]>>(&self, key: K) -> fjall::Result<Option<u32>> {
        self.inner.size_of(key)
    }

    fn range<K: AsRef<[u8]>, R: RangeBounds<K>>(&self, range: R) -> fjall::Iter {
        self.inner.range(range)
    }

    fn prefix<K: AsRef<[u8]>>(&self, prefix: K) -> fjall::Iter {
        self.inner.prefix(prefix)
    }
}
