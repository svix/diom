use std::{fs, path::PathBuf};

use super::Cli;

use anyhow::Context as _;
use config::FileFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[cfg(all(feature = "http1", feature = "http2"))]
    #[serde(default)]
    pub http1: bool,
}

impl Config {
    pub fn load(cli: &Cli) -> anyhow::Result<Config> {
        let cfg_file = get_config_file_path()?;
        let cfg_file = cfg_file
            .as_os_str()
            .to_str()
            .context("non-UTF8 config file path")?;

        let mut builder = config::Config::builder();
        if fs::exists(cfg_file)? {
            builder = builder.add_source(config::File::new(cfg_file, FileFormat::Toml));
        }

        let cli_config = config::Config::builder()
            .set_override("server_url", cli.server_url.clone())?
            .set_override("auth_token", cli.auth_token.clone())?
            .build()?;

        builder
            .add_source(cli_config)
            .add_source(config::Environment::with_prefix("DIOM"))
            .build()?
            .try_deserialize()
            .context("failed to extract configuration")
    }

    pub fn server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }

    pub fn auth_token(&self) -> String {
        self.auth_token.as_deref().unwrap_or("xxx").to_owned()
    }
}

const FILE_NAME: &str = "diom-cli-config.toml";

fn get_folder() -> anyhow::Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("unable to find config path")?
        .join("diom"))
}

pub fn get_config_file_path() -> anyhow::Result<PathBuf> {
    Ok(get_folder()?.join(FILE_NAME))
}
