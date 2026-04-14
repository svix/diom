use std::{fmt::Debug, num::NonZeroU64};

use diom_core::{PersistableValue, string_wrapper, types::DurationMs};
use diom_id::Module;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

string_wrapper!(NamespaceName {
    min_length: 1,
    max_length: 256,
    pattern: r"^[a-zA-Z0-9\-/_.=+]+$",
    example: "some_namespace"
});

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, JsonSchema, PersistableValue,
)]
#[serde(rename_all = "kebab-case")]
pub enum EvictionPolicy {
    #[default]
    NoEviction,
}

pub trait ModuleConfig:
    Clone + Debug + PartialEq + Eq + Serialize + DeserializeOwned + Send + Sync + PersistableValue
{
    fn module() -> Module;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
pub struct KeyValueConfig {}

impl KeyValueConfig {
    pub const NAMESPACE: &'static str = "kv_store";
}

impl ModuleConfig for KeyValueConfig {
    fn module() -> Module {
        Module::Kv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
pub struct CacheConfig {
    pub eviction_policy: EvictionPolicy,
}

impl CacheConfig {
    pub const NAMESPACE: &'static str = "cache_store";
}

impl ModuleConfig for CacheConfig {
    fn module() -> Module {
        Module::Cache
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, PersistableValue)]
pub struct MsgsConfig {
    pub retention_period: Option<DurationMs>,
    pub retention_bytes: Option<NonZeroU64>,
}

impl ModuleConfig for MsgsConfig {
    fn module() -> Module {
        Module::Msgs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
pub struct RateLimitConfig {}

impl ModuleConfig for RateLimitConfig {
    fn module() -> Module {
        Module::RateLimit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
pub struct IdempotencyConfig {}

impl ModuleConfig for IdempotencyConfig {
    fn module() -> Module {
        Module::Idempotency
    }
}

impl IdempotencyConfig {
    pub const NAMESPACE: &'static str = "idempotency_store";
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, JsonSchema, PersistableValue)]
pub struct AuthTokenConfig {}

impl AuthTokenConfig {
    pub const NAMESPACE: &'static str = "auth_token_store";
}

impl ModuleConfig for AuthTokenConfig {
    fn module() -> Module {
        Module::AuthToken
    }
}
