use std::{fs, path::PathBuf};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_url: Option<String>,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let cfg_file = get_config_file_path()?;
        let cfg_file = cfg_file
            .as_os_str()
            .to_str()
            .context("non-UTF8 config file path")?;

        let mut builder = config::Config::builder();
        if fs::exists(cfg_file)? {
            builder = builder.add_source(config::File::new(cfg_file, FileFormat::Toml));
        }

        builder
            .add_source(config::Environment::with_prefix("COYOTE"))
            .build()?
            .try_deserialize()
            .context("failed to extract configuration")
    }

    /// Gives the `server_url` for a Svix client with fallback to the legacy `SVIX_DEBUG_URL` variable/config.
    pub fn server_url(&self) -> Option<&str> {
        match self.server_url.as_deref() {
            Some(s) if s.trim().is_empty() => self.debug_url.as_deref(),
            server_url @ Some(_) => server_url,
            None => self.debug_url.as_deref(),
        }
    }
}

const FILE_NAME: &str = "config.toml";

fn get_folder() -> anyhow::Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("unable to find config path")?
        .join("coyote"))
}

pub fn get_config_file_path() -> anyhow::Result<PathBuf> {
    Ok(get_folder()?.join(FILE_NAME))
}
