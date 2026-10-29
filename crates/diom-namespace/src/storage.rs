use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::NamespaceId;
use fjall::Keyspace;
use fjall_utils::{FjallKey, TableRow};
use serde::{Deserialize, Serialize};

use crate::entities::{ModuleConfig, NamespaceName};

/// These values can never change. Only additions are allowed.
#[repr(u8)]
enum RowType {
    Namespace = 0,
}

#[derive(FjallKey)]
#[table_key(prefix = RowType::Namespace)]
pub(crate) struct NamespaceKey {
    #[key(0)]
    pub(crate) module: u32,
    #[key(1)]
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug, PersistableValue)]
#[serde(bound = "C: ModuleConfig")]
pub struct Namespace<C: ModuleConfig> {
    pub id: NamespaceId,
    pub name: NamespaceName,

    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,

    // Module-specific
    pub config: C,
}

impl<C: ModuleConfig> TableRow for Namespace<C> {
    const ROW_TYPE: u8 = RowType::Namespace as u8;
}

impl<C: ModuleConfig> Namespace<C> {
    pub(crate) fn module_id() -> u32 {
        C::module() as u32
    }

    pub(crate) fn fetch(keyspace: &Keyspace, namespace_name: &str) -> Result<Option<Self>> {
        let key = NamespaceKey::build_key(&Self::module_id(), namespace_name);
        <Self as TableRow>::fetch(keyspace, key)
    }

    pub(crate) fn fetch_all(keyspace: &Keyspace) -> Result<impl Iterator<Item = Self>> {
        let prefix = NamespaceKey::prefix_module(&Self::module_id());
        Ok(keyspace.prefix(prefix).map(|g| {
            let v = g.value().expect("iter error?");
            Self::from_fjall_value(v).expect("deserialize error?")
        }))
    }
}

#[cfg(test)]
mod byte_fixtures {
    use super::*;
    use crate::entities::{
        AuthTokenConfig, CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig,
        MsgsConfig, RateLimitConfig,
    };
    use fjall_utils::fixtures::decode_row;
    const WRAPPER: &str =
        "0010000000000000000000000000000000000c6d792d6e616d65737061636580d095ffbc3181d095ffbc31";
    #[test]
    fn namespace_kv_v0() {
        // bytes come from a serialized Namespace<KeyValueConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<KeyValueConfig> = decode_row(WRAPPER);
        assert_eq!(ns.id, NamespaceId::nil());
        assert_eq!(ns.config, KeyValueConfig {});
    }
    #[test]
    fn namespace_cache_v0() {
        // bytes come from a serialized Namespace<CacheConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<CacheConfig> = decode_row(&format!("{WRAPPER}00"));
        assert_eq!(
            ns.config,
            CacheConfig {
                eviction_policy: EvictionPolicy::NoEviction
            }
        );
    }
    #[test]
    fn namespace_msgs_v0() {
        // bytes come from a serialized Namespace<MsgsConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<MsgsConfig> = decode_row(&format!("{WRAPPER}0000"));
        assert_eq!(
            ns.config,
            MsgsConfig {
                retention_period: None,
                retention_bytes: None
            }
        );
    }
    #[test]
    fn namespace_rate_limit_v0() {
        // bytes come from a serialized Namespace<RateLimitConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<RateLimitConfig> = decode_row(WRAPPER);
        assert_eq!(ns.config, RateLimitConfig {});
    }
    #[test]
    fn namespace_idempotency_v0() {
        // bytes come from a serialized Namespace<IdempotencyConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<IdempotencyConfig> = decode_row(WRAPPER);
        assert_eq!(ns.config, IdempotencyConfig {});
    }
    #[test]
    fn namespace_auth_token_v0() {
        // bytes come from a serialized Namespace<AuthTokenConfig>.
        // If a backwards INcompatible change to the row is introduced, this test will fail.
        let ns: Namespace<AuthTokenConfig> = decode_row(WRAPPER);
        assert_eq!(ns.config, AuthTokenConfig {});
    }
}
