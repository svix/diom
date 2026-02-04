use std::{collections::HashMap, num::NonZeroU64};

use crate::cfg::{Configuration as AppConfig, DatabaseConfig};
use anyhow::Context;
use config::ConfigBuilder;
use diom_configgroup::{
    BothDatabases,
    entities::{
        CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig, ModuleConfig, StorageType,
        StreamConfig,
    },
    operations::create_configgroup::CreateConfigGroup,
};
use serde::Deserialize;
use tap::Pipe;

const DEFAULTS: &str = include_str!("../bootstrap.default.yaml");

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

#[derive(Debug, Deserialize)]
struct ConfigGroupIn<C> {
    pub storage_type: Option<StorageTypeIn>,
    pub max_storage_bytes: Option<NonZeroU64>,

    #[serde(flatten)]
    pub config: C,
}

impl<C: IntoModuleConfig> ConfigGroupIn<C> {
    fn create_config_group(self, name: String) -> CreateConfigGroup<C::Output> {
        CreateConfigGroup::new(
            name,
            self.config.into_module_config(),
            self.storage_type.map(|x| x.into()),
            self.max_storage_bytes,
        )
    }
}

#[derive(Debug, Deserialize)]
struct KeyValueConfigIn {}

impl IntoModuleConfig for KeyValueConfigIn {
    type Output = KeyValueConfig;

    fn into_module_config(self) -> Self::Output {
        KeyValueConfig {}
    }
}

#[derive(Debug, Deserialize)]
struct IdempotencyConfigIn {}

impl IntoModuleConfig for IdempotencyConfigIn {
    type Output = IdempotencyConfig;

    fn into_module_config(self) -> Self::Output {
        IdempotencyConfig {}
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct BootstrapConfig {
    cache: Option<HashMap<String, ConfigGroupIn<CacheConfigIn>>>,
    idempotency: Option<HashMap<String, ConfigGroupIn<IdempotencyConfigIn>>>,
    kv: Option<HashMap<String, ConfigGroupIn<KeyValueConfigIn>>>,
    stream: Option<HashMap<String, ConfigGroupIn<StreamConfigIn>>>,
}

impl BootstrapConfig {
    fn load(config_path: Option<&str>) -> anyhow::Result<BootstrapConfig> {
        let config = config::Config::builder()
            .add_source(config::File::from_str(DEFAULTS, config::FileFormat::Yaml))
            .pipe(|config: ConfigBuilder<_>| {
                if let Some(path) = config_path {
                    config.add_source(config::File::with_name(path))
                } else {
                    config
                }
            })
            .add_source(config::Environment::with_prefix("DIOM"))
            .build()?;

        let config: BootstrapConfig = config
            .try_deserialize::<BootstrapConfig>()
            .context("failed to load bootstrap configuration")?;

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

    let db = state.db();
    let keyspace = state.keyspace();

    if let Some(kv) = bootstrap.kv {
        for (name, cfg) in kv {
            tracing::debug!(?name, "bootstrapping kv");
            let create_cmd = cfg.create_config_group(name);
            create_cmd
                .apply_operation(db, keyspace)
                .expect("create config");
        }
    }

    if let Some(cache) = bootstrap.cache {
        for (name, cfg) in cache {
            tracing::debug!(?name, "bootstrapping cache");
            let create_cmd = cfg.create_config_group(name);
            create_cmd
                .apply_operation(db, keyspace)
                .expect("create config");
        }
    }

    if let Some(idempotency) = bootstrap.idempotency {
        for (name, cfg) in idempotency {
            tracing::debug!(?name, "bootstrapping idemptency");
            let create_cmd = cfg.create_config_group(name);
            create_cmd
                .apply_operation(db, keyspace)
                .expect("create config");
        }
    }

    if let Some(stream) = bootstrap.stream {
        for (name, cfg) in stream {
            tracing::debug!(?name, "bootstrapping stream");
            let create_cmd = cfg.create_config_group(name);
            create_cmd
                .apply_operation(db, keyspace)
                .expect("create config");
        }
    }

    state.flush_and_sync().expect("failed to persist DBs");

    tracing::info!("done bootstrapping databases");
}
