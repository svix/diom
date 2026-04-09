use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// this is super-hacky but necessary until
// https://github.com/fjall-rs/fjall/issues/262 is done. Rip it out at that point

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchemaManifest {
    keyspaces: BTreeMap<String, SerializableKeyspaceCreateOptions>,
}

impl SchemaManifest {
    pub fn load_from_db(database: &fjall::Database) -> anyhow::Result<Self> {
        let mut keyspaces = BTreeMap::new();
        let keyspace = database.keyspace("_schema", fjall::KeyspaceCreateOptions::default)?;
        for row in keyspace.iter() {
            let (key, value) = row.into_inner()?;
            let keyspace_name = std::str::from_utf8(&key)?.to_owned();
            let value = postcard::from_bytes::<crate::V0Wrapper<_>>(&value)
                .map(|crate::V0Wrapper::V0(inner)| inner)?;
            tracing::debug!(?keyspace_name, "loaded schema for keyspace from db");
            keyspaces.insert(keyspace_name, value);
        }
        Ok(Self { keyspaces })
    }

    pub fn keyspace(
        &self,
        database: &fjall::Database,
        keyspace_name: &str,
    ) -> fjall::Result<fjall::Keyspace> {
        database.keyspace(keyspace_name, || self.options_for_keyspace(keyspace_name))
    }

    pub fn contains(&self, keyspace_name: &str) -> bool {
        self.keyspaces.contains_key(keyspace_name)
    }

    pub fn options_for_keyspace(&self, keyspace_name: &str) -> fjall::KeyspaceCreateOptions {
        if let Some(schema) = self.keyspaces.get(keyspace_name) {
            tracing::debug!(?keyspace_name, "found create options");
            schema.clone().into()
        } else {
            fjall::KeyspaceCreateOptions::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone)]
pub enum SerializableCompressionType {
    Lz4,
    #[default]
    None,
}

impl From<SerializableCompressionType> for fjall::CompressionType {
    fn from(value: SerializableCompressionType) -> Self {
        match value {
            SerializableCompressionType::None => Self::None,
            SerializableCompressionType::Lz4 => Self::Lz4,
        }
    }
}

impl From<fjall::CompressionType> for SerializableCompressionType {
    fn from(value: fjall::CompressionType) -> Self {
        match value {
            fjall::CompressionType::None => Self::None,
            fjall::CompressionType::Lz4 => Self::Lz4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerializableKvSeparationOptions {
    compression: SerializableCompressionType,
    file_target_size: u64,
    separation_threshold: u32,
    staleness_threshold: f32,
    age_cutoff: f32,
}

impl Default for SerializableKvSeparationOptions {
    fn default() -> Self {
        fjall::KvSeparationOptions::default().into()
    }
}

impl From<fjall::KvSeparationOptions> for SerializableKvSeparationOptions {
    fn from(value: fjall::KvSeparationOptions) -> Self {
        Self {
            compression: value.compression.into(),
            file_target_size: value.file_target_size,
            separation_threshold: value.separation_threshold,
            staleness_threshold: value.staleness_threshold,
            age_cutoff: value.age_cutoff,
        }
    }
}

impl From<SerializableKvSeparationOptions> for fjall::KvSeparationOptions {
    fn from(value: SerializableKvSeparationOptions) -> Self {
        Self::default()
            .compression(value.compression.into())
            .file_target_size(value.file_target_size)
            .separation_threshold(value.separation_threshold)
            .staleness_threshold(value.staleness_threshold)
            .age_cutoff(value.age_cutoff)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SerializableKeyspaceCreateOptions {
    kv_separation_options: Option<SerializableKvSeparationOptions>,
    expect_point_read_hits: bool,
}

impl From<SerializableKeyspaceCreateOptions> for fjall::KeyspaceCreateOptions {
    fn from(value: SerializableKeyspaceCreateOptions) -> Self {
        Self::default()
            .with_kv_separation(value.kv_separation_options.map(Into::into))
            .expect_point_read_hits(value.expect_point_read_hits)
    }
}

impl SerializableKeyspaceCreateOptions {
    pub fn with_kv_separation(mut self, opts: Option<SerializableKvSeparationOptions>) -> Self {
        self.kv_separation_options = opts;
        self
    }

    pub fn with_default_kv_separation(mut self) -> Self {
        self.kv_separation_options = Some(SerializableKvSeparationOptions::default());
        self
    }

    pub fn expect_point_read_hits(mut self, expect: bool) -> Self {
        self.expect_point_read_hits = expect;
        self
    }

    /// Get or create the given keyspace name in the given database,
    ///
    /// recording the currente operations into the special _schema keyspace.
    pub fn create_and_record(
        self,
        database: &fjall::Database,
        keyspace_name: &str,
    ) -> anyhow::Result<fjall::Keyspace> {
        let schema = database.keyspace("_schema", fjall::KeyspaceCreateOptions::default)?;
        let serialized =
            postcard::to_allocvec(&crate::V0Wrapper::V0(&self)).context("serializing schema")?;
        schema.insert(keyspace_name, serialized)?;
        database
            .keyspace(keyspace_name, || self.into())
            .context("creating target keyspace")
    }
}
