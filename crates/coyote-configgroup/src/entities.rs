use std::fmt::{self, Debug, Display, Formatter};
use std::num::NonZeroU64;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

pub type ConfigGroupId = Uuid;

#[derive(Serialize, Deserialize)]
#[repr(u32)]
pub enum Module {
    Cache = 1,
    Idempotency = 2,
    KeyValue = 3,
    RateLimiter = 4,
    Stream = 5,
}

// This shouldn't be needed when we're writing keys as bytes
impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = match self {
            Module::Cache => 1,
            Module::Idempotency => 2,
            Module::KeyValue => 3,
            Module::RateLimiter => 4,
            Module::Stream => 5,
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum StorageType {
    Persistent = 0,
    Ephemeral = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema)]
pub enum EvictionPolicy {
    #[default]
    NoEviction,
    LeastRecentlyUsed,
}

pub trait ModuleConfig:
    Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned + JsonSchema
{
    const TABLE_PREFIX: &'static str;

    fn module() -> Module;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct KeyValueConfig {}

impl KeyValueConfig {
    pub const NAMESPACE: &'static str = "kv_store";
}

impl KeyValueConfig {
    pub fn eviction_policy(&self) -> EvictionPolicy {
        EvictionPolicy::NoEviction
    }
}

impl ModuleConfig for KeyValueConfig {
    const TABLE_PREFIX: &'static str = "_CONFIG_KV_";

    fn module() -> Module {
        Module::KeyValue
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct CacheConfig {
    pub eviction_policy: EvictionPolicy,
}

impl CacheConfig {
    pub const NAMESPACE: &'static str = "cache_store";

    pub fn eviction_policy(&self) -> EvictionPolicy {
        self.eviction_policy
    }
}

impl ModuleConfig for CacheConfig {
    const TABLE_PREFIX: &'static str = "_CONFIG_CACHE_";

    fn module() -> Module {
        Module::Cache
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct StreamConfig {
    pub retention_period_seconds: Option<NonZeroU64>,
    pub max_bytes_size: u64,
    pub storage_type: Option<StorageType>,
}

impl ModuleConfig for StreamConfig {
    const TABLE_PREFIX: &'static str = "_CONFIG_STREAM_";

    fn module() -> Module {
        Module::Stream
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct IdempotencyConfig {}

impl ModuleConfig for IdempotencyConfig {
    const TABLE_PREFIX: &'static str = "_CONFIG_IDEM_";

    fn module() -> Module {
        Module::Idempotency
    }
}

impl IdempotencyConfig {
    pub const NAMESPACE: &'static str = "idempotency_store";

    pub fn eviction_policy(&self) -> EvictionPolicy {
        EvictionPolicy::NoEviction
    }
}
