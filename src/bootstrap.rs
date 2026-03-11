use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    num::NonZeroU64,
    time::Instant,
};

use crate::{cfg::Configuration as AppConfig, core::cluster::RaftState};
use anyhow::Context;
use diom_msgs::entities::{Retention, default_retention_bytes, default_retention_millis};
use diom_namespace::entities::{EvictionPolicy, StorageType};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageTypeIn {
    #[default]
    Persistent,
    Ephemeral,
}

impl From<StorageTypeIn> for StorageType {
    fn from(value: StorageTypeIn) -> Self {
        match value {
            StorageTypeIn::Persistent => StorageType::Persistent,
            StorageTypeIn::Ephemeral => StorageType::Ephemeral,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvictionPolicyIn {
    #[default]
    NoEviction,
    LeastRecentlyUsed,
}

impl From<EvictionPolicyIn> for EvictionPolicy {
    fn from(value: EvictionPolicyIn) -> Self {
        match value {
            EvictionPolicyIn::NoEviction => EvictionPolicy::NoEviction,
            EvictionPolicyIn::LeastRecentlyUsed => EvictionPolicy::LeastRecentlyUsed,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct NamespaceIn<C> {
    #[serde(default)]
    pub storage_type: StorageTypeIn,
    pub max_storage_bytes: Option<NonZeroU64>,

    #[serde(flatten)]
    pub config: C,
}

#[derive(Debug, Default, Deserialize)]
struct KeyValueConfigIn {}

#[derive(Debug, Default, Deserialize)]
struct IdempotencyConfigIn {}

#[derive(Debug, Default, Deserialize)]
struct CacheConfigIn {
    pub eviction_policy: Option<EvictionPolicyIn>,
}

#[derive(Debug, Default, Deserialize)]
struct RateLimitConfigIn {}

#[derive(Debug, Default, Deserialize)]
struct StreamConfigIn {
    pub retention_period_ms: Option<NonZeroU64>,
}

#[derive(Debug, Default, Deserialize)]
struct BootstrapConfig {
    cache: Option<HashMap<String, NamespaceIn<CacheConfigIn>>>,
    idempotency: Option<HashMap<String, NamespaceIn<IdempotencyConfigIn>>>,
    rate_limit: Option<HashMap<String, NamespaceIn<RateLimitConfigIn>>>,
    kv: Option<HashMap<String, NamespaceIn<KeyValueConfigIn>>>,
    stream: Option<HashMap<String, NamespaceIn<StreamConfigIn>>>,
}

impl BootstrapConfig {
    fn load(config_path: Option<&str>) -> anyhow::Result<BootstrapConfig> {
        let mut config = match config_path {
            Some(path) => {
                let config_file = File::open(path).context("opening bootstrap config")?;
                yaml_serde::from_reader(config_file).context("parsing bootstrap config")?
            }
            None => BootstrapConfig::default(),
        };

        // Configure default namespace for cache, if not part of the config file
        if let Entry::Vacant(v) = config
            .cache
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(NamespaceIn::default());
        }

        // Configure default namespace for idempotency, if not part of the config file
        if let Entry::Vacant(v) = config
            .idempotency
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(NamespaceIn::default());
        }

        // Configure default namespace for kv, if not part of the config file
        if let Entry::Vacant(v) = config
            .kv
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(NamespaceIn::default());
        }

        // Configure default namespace for idempotency, if not part of the config file
        if let Entry::Vacant(v) = config
            .rate_limit
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(NamespaceIn::default());
        }

        // Configure default namespace for stream (msgs), if not part of the config file
        if let Entry::Vacant(v) = config
            .stream
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(NamespaceIn::default());
        }

        Ok(config)
    }
}

pub async fn run(app_config: AppConfig, raft_state: RaftState) -> anyhow::Result<()> {
    let t = Instant::now();
    // FIXME: Do something smarter here:
    let mut retries = 100;
    let shutdown = crate::shutting_down_token();
    while !raft_state.is_up().await && retries > 0 {
        retries -= 1;
        if shutdown
            .run_until_cancelled(tokio::time::sleep(std::time::Duration::from_millis(100)))
            .await
            .is_none()
        {
            anyhow::bail!("shut down before bootstrap finished");
        }
    }

    let bootstrap = BootstrapConfig::load(app_config.bootstrap_cfg_path.as_deref())
        .expect("Failed to load bootstrap config");

    tracing::debug!(
        ?bootstrap,
        persistent_db=?app_config.persistent_db,
        ephemeral_db=?app_config.ephemeral_db,
        "Starting bootstrapping."
    );

    if let Some(kv) = bootstrap.kv {
        for (name, cfg) in kv {
            tracing::debug!(?name, "bootstrapping kv");
            let operation = diom_kv::operations::CreateKvOperation::new(
                name,
                cfg.storage_type.into(),
                cfg.max_storage_bytes,
            );
            raft_state.client_write(operation).await?;
        }
    }

    if let Some(cache) = bootstrap.cache {
        for (name, cfg) in cache {
            tracing::debug!(?name, "bootstrapping cache");
            let operation = diom_cache::operations::CreateCacheOperation::new(
                name,
                cfg.config.eviction_policy.unwrap_or_default().into(),
                cfg.storage_type.into(),
                cfg.max_storage_bytes,
            );
            raft_state.client_write(operation).await?;
        }
    }

    if let Some(idempotency) = bootstrap.idempotency {
        for (name, cfg) in idempotency {
            tracing::debug!(?name, "bootstrapping idemptency");
            let operation = diom_idempotency::operations::CreateIdempotencyOperation::new(
                name,
                cfg.storage_type.into(),
                cfg.max_storage_bytes,
            );
            raft_state.client_write(operation).await?;
        }
    }

    if let Some(rate_limit) = bootstrap.rate_limit {
        for (name, cfg) in rate_limit {
            tracing::debug!(?name, "bootstrapping idemptency");
            let operation = diom_rate_limit::operations::CreateRateLimitOperation::new(
                name,
                cfg.storage_type.into(),
                cfg.max_storage_bytes,
            );
            raft_state.client_write(operation).await?;
        }
    }

    if let Some(stream) = bootstrap.stream {
        for (name, cfg) in stream {
            tracing::debug!(?name, "bootstrapping stream");
            let retention = Retention {
                millis: cfg
                    .config
                    .retention_period_ms
                    .unwrap_or_else(default_retention_millis),
                bytes: cfg
                    .max_storage_bytes
                    .unwrap_or_else(default_retention_bytes),
            };
            let operation = diom_msgs::operations::CreateNamespaceOperation::new(
                name,
                retention,
                cfg.storage_type.into(),
            );
            raft_state.client_write(operation).await?;
        }
    }

    tracing::info!(
        duration_millis = (Instant::now() - t).as_millis(),
        "Finished bootstrapping."
    );

    Ok(())
}
