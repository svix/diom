// this file is @generated
use serde::{Deserialize, Serialize};

use super::eviction_policy::EvictionPolicy;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheCreateNamespaceOut {
    pub name: String,

    pub eviction_policy: EvictionPolicy,

    pub created: u64,

    pub updated: u64,
}

impl CacheCreateNamespaceOut {
    pub fn new(name: String, eviction_policy: EvictionPolicy, created: u64, updated: u64) -> Self {
        Self {
            name,
            eviction_policy,
            created,
            updated,
        }
    }
}
