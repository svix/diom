use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    num::NonZeroU64,
};

use crate::{cfg::Configuration as AppConfig, core::cluster::RaftState};
use anyhow::Context;
use coyote_configgroup::entities::{EvictionPolicy, StorageType};
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
struct ConfigGroupIn<C> {
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

#[derive(Debug, Deserialize)]
struct StreamConfigIn {
    pub retention_period_seconds: Option<NonZeroU64>,
}

#[derive(Debug, Default, Deserialize)]
struct BootstrapConfig {
    cache: Option<HashMap<String, ConfigGroupIn<CacheConfigIn>>>,
    idempotency: Option<HashMap<String, ConfigGroupIn<IdempotencyConfigIn>>>,
    kv: Option<HashMap<String, ConfigGroupIn<KeyValueConfigIn>>>,
    stream: Option<HashMap<String, ConfigGroupIn<StreamConfigIn>>>,
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

        // Configure default config group for cache, if not part of the config file
        if let Entry::Vacant(v) = config
            .cache
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(ConfigGroupIn::default());
        }

        // Configure default config group for idempotency, if not part of the config file
        if let Entry::Vacant(v) = config
            .idempotency
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(ConfigGroupIn::default());
        }

        // Configure default config group for kv, if not part of the config file
        if let Entry::Vacant(v) = config
            .kv
            .get_or_insert_default()
            .entry("default".to_owned())
        {
            v.insert(ConfigGroupIn::default());
        }

        Ok(config)
    }
}

pub async fn run(app_config: AppConfig, raft_state: RaftState) -> anyhow::Result<()> {
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
            let operation = coyote_kv::operations::CreateKvOperation::new(
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
            let operation = coyote_cache::operations::CreateCacheOperation::new(
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
            let operation = coyote_idempotency::operations::CreateIdempotencyOperation::new(
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
            let operation = stream_deprecated::operations::CreateStreamOperation::new(
                name,
                cfg.config.retention_period_seconds,
                cfg.storage_type.into(),
                cfg.max_storage_bytes,
            );
            raft_state.client_write(operation).await?;
        }
    }

    tracing::info!("Finished bootstrapping.");

    Ok(())
}
