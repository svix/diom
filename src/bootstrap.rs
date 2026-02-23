use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    num::NonZeroU64,
};

use crate::cfg::{Configuration as AppConfig, DatabaseConfig};
use anyhow::Context;
use diom_configgroup::{
    BothDatabases,
    entities::{
        CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig, ModuleConfig, StorageType,
        StreamConfig,
    },
    operations::create_configgroup::CreateConfigGroup,
};
use serde::Deserialize;

trait IntoModuleConfig {
    type Output: ModuleConfig;

    fn into_module_config(self) -> Self::Output;
}

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

impl<C: IntoModuleConfig> ConfigGroupIn<C> {
    fn create_config_group(self, name: String) -> CreateConfigGroup<C::Output> {
        CreateConfigGroup::new(
            name,
            self.config.into_module_config(),
            self.storage_type.into(),
            self.max_storage_bytes,
        )
    }
}

#[derive(Debug, Default, Deserialize)]
struct KeyValueConfigIn {}

impl IntoModuleConfig for KeyValueConfigIn {
    type Output = KeyValueConfig;

    fn into_module_config(self) -> Self::Output {
        KeyValueConfig {}
    }
}

#[derive(Debug, Default, Deserialize)]
struct IdempotencyConfigIn {}

impl IntoModuleConfig for IdempotencyConfigIn {
    type Output = IdempotencyConfig;

    fn into_module_config(self) -> Self::Output {
        IdempotencyConfig {}
    }
}

#[derive(Debug, Default, Deserialize)]
struct CacheConfigIn {
    pub eviction_policy: Option<EvictionPolicyIn>,
}

impl IntoModuleConfig for CacheConfigIn {
    type Output = CacheConfig;

    fn into_module_config(self) -> Self::Output {
        CacheConfig {
            eviction_policy: self.eviction_policy.unwrap_or_default().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct StreamConfigIn {
    pub retention_period_seconds: Option<NonZeroU64>,
}

impl IntoModuleConfig for StreamConfigIn {
    type Output = StreamConfig;

    fn into_module_config(self) -> Self::Output {
        StreamConfig {
            retention_period_seconds: self.retention_period_seconds,
        }
    }
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

pub fn run(bootstrap_cfg_path: Option<&str>, app_config: AppConfig) {
    let bootstrap =
        BootstrapConfig::load(bootstrap_cfg_path).expect("Failed to load bootstrap config");

    tracing::debug!(
        ?bootstrap,
        persistent_db=?app_config.persistent_db,
        ephemeral_db=?app_config.ephemeral_db,
        "starting bootstrap"
    );
    let persistent_db =
        DatabaseConfig::persistent(&app_config.persistent_db).expect("persistent db");
    let ephemeral_db = DatabaseConfig::ephemeral(&app_config.ephemeral_db).expect("ephemeral db");
    let state = diom_configgroup::State::init(BothDatabases {
        persistent_db,
        ephemeral_db,
    })
    .expect("configgroup state");

    if let Some(kv) = bootstrap.kv {
        for (name, cfg) in kv {
            tracing::debug!(?name, "bootstrapping kv");
            let create_cmd = cfg.create_config_group(name);
            create_cmd.apply_operation(&state).expect("create config");
        }
    }

    if let Some(cache) = bootstrap.cache {
        for (name, cfg) in cache {
            tracing::debug!(?name, "bootstrapping cache");
            let create_cmd = cfg.create_config_group(name);
            create_cmd.apply_operation(&state).expect("create config");
        }
    }

    if let Some(idempotency) = bootstrap.idempotency {
        for (name, cfg) in idempotency {
            tracing::debug!(?name, "bootstrapping idemptency");
            let create_cmd = cfg.create_config_group(name);
            create_cmd.apply_operation(&state).expect("create config");
        }
    }

    if let Some(stream) = bootstrap.stream {
        for (name, cfg) in stream {
            tracing::debug!(?name, "bootstrapping stream");
            let create_cmd = cfg.create_config_group(name);
            create_cmd.apply_operation(&state).expect("create config");
        }
    }

    state.flush_and_sync().expect("failed to persist DBs");

    tracing::info!("done bootstrapping databases");
}
