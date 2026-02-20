use std::{
    fmt::{self, Debug, Display, Formatter},
    num::NonZeroU64,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

pub type ConfigGroupId = Uuid;
pub type ConfigGroupName = String;

#[derive(Serialize, Deserialize)]
#[repr(u8)]
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

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub enum StorageType {
    #[default]
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
    fn module() -> Module;

    fn eviction_policy(&self) -> EvictionPolicy {
        EvictionPolicy::NoEviction
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct KeyValueConfig {}

impl KeyValueConfig {
    pub const NAMESPACE: &'static str = "kv_store";
}

impl ModuleConfig for KeyValueConfig {
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
    fn module() -> Module {
        Module::Cache
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct StreamConfig {
    pub retention_period_seconds: Option<NonZeroU64>,
}

impl ModuleConfig for StreamConfig {
    fn module() -> Module {
        Module::Stream
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct IdempotencyConfig {}

impl ModuleConfig for IdempotencyConfig {
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
