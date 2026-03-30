use std::fmt::Debug;

use diom_core::types::DurationMs;
use diom_id::Module;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub type NamespaceName = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema)]
pub enum EvictionPolicy {
    #[default]
    NoEviction,
    LeastRecentlyUsed,
}

pub trait ModuleConfig:
    Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned + Send + Sync
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
        Module::Kv
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MsgsConfig {
    pub retention_period: DurationMs,
}

impl ModuleConfig for MsgsConfig {
    fn module() -> Module {
        Module::Msgs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct RateLimitConfig {}

impl ModuleConfig for RateLimitConfig {
    fn module() -> Module {
        Module::RateLimit
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct AuthTokenConfig {}

impl AuthTokenConfig {
    pub const NAMESPACE: &'static str = "auth_token_store";
}

impl ModuleConfig for AuthTokenConfig {
    fn module() -> Module {
        Module::AuthToken
    }
}
