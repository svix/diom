use std::num::NonZeroU64;

use diom::{bootstrap, cfg::DatabaseConfig};
use diom_configgroup::{
    BothDatabases,
    entities::{CacheConfig, EvictionPolicy, KeyValueConfig, StreamConfig},
};
use test_utils::TestResult;

use crate::TestContext;

#[tokio::test]
async fn test_bootstrap() -> TestResult {
    let TestContext {
        cfg,
        handle: _handle,
        ..
    } = super::start_server().await;

    bootstrap::run(Some("./tests/it/static/bootstrap.test.yaml"), cfg.clone());

    let persistent_db = DatabaseConfig::persistent(&cfg.persistent_db).expect("persistent db");
    let ephemeral_db = DatabaseConfig::ephemeral(&cfg.ephemeral_db).expect("ephemeral db");

    let configgroup_state = diom_configgroup::State::init(BothDatabases {
        persistent_db: persistent_db.clone(),
        ephemeral_db: ephemeral_db.clone(),
    })
    .expect("initializing configgroup state");

    let default_kv_group =
        configgroup_state.fetch_group::<KeyValueConfig>("default".to_string())?;

    let default_cache_group =
        configgroup_state.fetch_group::<CacheConfig>("default".to_string())?;

    let default_stream_group =
        configgroup_state.fetch_group::<StreamConfig>("default".to_string())?;

    assert!(default_kv_group.is_some());
    assert!(default_cache_group.is_some());
    assert!(default_stream_group.is_some());

    let default_kv_group = default_kv_group.unwrap();
    assert_eq!(
        default_kv_group.max_storage_bytes,
        Some(NonZeroU64::new(1000).unwrap())
    );

    let Some(kv1) = configgroup_state.fetch_group::<KeyValueConfig>("kv1".to_string())? else {
        panic!("kv1 missing");
    };
    assert_eq!(&kv1.name, "kv1");
    assert_eq!(kv1.max_storage_bytes, Some(NonZeroU64::new(2000).unwrap()));

    let Some(kv2) = configgroup_state.fetch_group::<KeyValueConfig>("kv2".to_string())? else {
        panic!("kv2 missing");
    };
    assert_eq!(&kv2.name, "kv2");
    assert_eq!(kv2.max_storage_bytes, Some(NonZeroU64::new(3000).unwrap()));

    let default_cache_group = default_cache_group.unwrap();
    assert_eq!(
        default_cache_group.config.eviction_policy,
        EvictionPolicy::NoEviction
    );

    let Some(cache1) = configgroup_state.fetch_group::<CacheConfig>("cache1".to_string())? else {
        panic!("cache1 missing");
    };
    assert_eq!(&cache1.name, "cache1");
    assert_eq!(
        cache1.config.eviction_policy(),
        EvictionPolicy::LeastRecentlyUsed
    );

    Ok(())
}
