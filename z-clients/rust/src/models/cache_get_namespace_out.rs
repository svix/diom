// this file is @generated
use serde::{Deserialize, Serialize};

use super::eviction_policy::EvictionPolicy;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetNamespaceOut {
    pub name: String,

    pub eviction_policy: EvictionPolicy,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub created: jiff::Timestamp,

    #[serde(with = "crate::unix_timestamp_ms_serde")]
    pub updated: jiff::Timestamp,
}

impl CacheGetNamespaceOut {
    pub fn new(
        name: String,
        eviction_policy: EvictionPolicy,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            eviction_policy,
            created,
            updated,
        }
    }
}
