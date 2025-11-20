// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{borrow::Cow, collections::HashMap, fmt, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{bail, Context};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use ipnet::IpNet;
use serde::{Deserialize, Deserializer};
use tracing::Level;
use url::Url;
use validator::{Validate, ValidationError};

use crate::{
    core::security::JwtSigningConfig,
    error::Result,
    v1::utils::validation_error,
};

const DEFAULTS: &str = include_str!("../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, Deserialize, Validate)]
#[validate(
    schema(function = "validate_config_complete"),
    skip_on_field_errors = false
)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: SocketAddr,

    /// Contains the secret and algorithm for signing JWTs
    #[serde(flatten)]
    pub jwt_signing_config: Arc<JwtSigningConfig>,

    /// The log level to run the service with. Supported: info, debug, trace
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    pub log_format: LogFormat,
    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,
    /// The ratio at which to sample spans when sending to OpenTelemetry. When not given it defaults
    /// to always sending. If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,
    /// The service name to use for OpenTelemetry. If not provided, it defaults to "diom".
    pub opentelemetry_service_name: String,
    /// The environment (dev, staging, or prod) that the server is running in.
    pub environment: Environment,

    /// Optional configuration for sending webhooks through a proxy.
    #[serde(flatten)]
    pub proxy_config: Option<ProxyConfig>,

    #[serde(flatten)]
    pub internal: InternalConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProxyConfig {
    /// Proxy address.
    ///
    /// Currently supported proxy types are:
    /// - `socks5://`, i.e. a SOCKS5 proxy, with domain name resolution being
    ///   done before the proxy gets involved
    /// - `http://` or `https://` proxy, sending HTTP requests to the proxy;
    ///   both HTTP and HTTPS targets are supported
    #[serde(rename = "proxy_addr")]
    pub addr: ProxyAddr,
}

#[derive(Clone, Debug)]
pub enum ProxyAddr {
    /// A SOCKS5 proxy.
    Socks5(http::Uri),
    /// An HTTP / HTTPs proxy.
    Http(http::Uri),
}

impl ProxyAddr {
    pub fn new(raw: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = raw.into();
        let parsed: http::Uri = raw.parse()?;
        match parsed.scheme_str().unwrap_or("") {
            "socks5" => Ok(Self::Socks5(parsed)),
            "http" | "https" => Ok(Self::Http(parsed)),
            _ => Err("Unsupported proxy scheme. \
                Supported schemes are `socks5://`, `http://` and `https://`."
                .into()),
        }
    }
}

impl<'de> Deserialize<'de> for ProxyAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::new(raw).map_err(serde::de::Error::custom)
    }
}

fn validate_config_complete(_config: &ConfigurationInner) -> Result<(), ValidationError> {
    Ok(())
}

impl ConfigurationInner {
}

#[derive(Clone, Debug, Deserialize)]
pub struct InternalConfig {
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Default,
    Json,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Dev => "dev",
                Environment::Staging => "staging",
                Environment::Prod => "prod",
            }
        )
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
            Self::Trace => Level::TRACE,
        }
        .fmt(f)
    }
}

/// Try to extract a [`ConfigurationInner`] from the provided [`Figment`]. Any error message should
/// indicate the missing required field(s).
fn try_extract(figment: Figment) -> anyhow::Result<ConfigurationInner> {
    // Explicitly override error if `jwt_secret` is not set, as the default error does not mention
    // the field name due it coming from an inlined field `ConfigurationInner::jwt_signing_config`
    // See: <https://github.com/SergioBenitez/Figment/issues/80>
    if !figment.contains("jwt_secret") {
        bail!("missing field `jwt_secret`");
    }

    Ok(figment.extract()?)
}

pub fn load() -> anyhow::Result<Arc<ConfigurationInner>> {
    let merged = Figment::new()
        .merge(Toml::string(DEFAULTS))
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("DIOM_"));

    let config = try_extract(merged).context("failed to extract configuration")?;

    config
        .validate()
        .context("failed to validate configuration")?;
    Ok(Arc::from(config))
}
