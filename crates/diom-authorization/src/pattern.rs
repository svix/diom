#![expect(unused_qualifications)] // triggered by schema_with

use std::{fmt, str::FromStr};

use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de};

#[derive(JsonSchema)]
#[schemars(schema_with = "String::json_schema")]
pub struct ResourcePattern {
    pub module: Module,
    pub namespace: NamespacePattern,
    pub key: KeyPattern,
}

pub enum Module {
    AuthToken,
    Cache,
    Idempotency,
    Kv,
    Msgs,
    RateLimit,
}

pub enum NamespacePattern {
    Default,
    Named(String),
    Any,
    // FIXME: Do namespaces have any sort of hierarchy?
}

pub enum KeyPattern {
    Exactly(String),
    Prefix(String),
    // FIXME: Add single star wildcards
}

impl fmt::Display for ResourcePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            module,
            namespace,
            key,
        } = self;
        write!(f, "{module}:{namespace}:{key}")
    }
}

impl FromStr for ResourcePattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [module, namespace, key] = s
            .split(':')
            .collect_array()
            .ok_or("invalid resource pattern, must contain exactly two colons")?;
        Ok(Self {
            module: module.parse()?,
            namespace: namespace.parse()?,
            key: key.parse()?,
        })
    }
}

impl Serialize for ResourcePattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourcePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AuthToken => "auth_token",
            Self::Cache => "cache",
            Self::Idempotency => "idempotency",
            Self::Kv => "kv",
            Self::Msgs => "msgs",
            Self::RateLimit => "rate_limit",
        };

        f.write_str(s)
    }
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auth_token" => Ok(Self::AuthToken),
            "cache" => Ok(Self::Cache),
            "idempotency" => Ok(Self::Idempotency),
            "kv" => Ok(Self::Kv),
            "msgs" => Ok(Self::Msgs),
            "rate_limit" => Ok(Self::RateLimit),
            _ => Err("unknown module"),
        }
    }
}

impl fmt::Display for NamespacePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Named(s) => f.write_str(s),
            Self::Any => f.write_str("*"),
        }
    }
}

impl FromStr for NamespacePattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::Default);
        }

        if s == "*" {
            return Ok(Self::Any);
        }

        if s.contains("*") {
            return Err("invalid namespace pattern: wildcard only allowed independently");
        }
        // FIXME: Could forbid more things but they'll just never match anything
        // so skipping that for now.

        Ok(Self::Named(s.to_owned()))
    }
}

impl fmt::Display for KeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exactly(s) => f.write_str(s),
            Self::Prefix(s) => {
                f.write_str(s)?;
                f.write_str("/**")
            }
        }
    }
}

impl FromStr for KeyPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(prefix) = s.strip_suffix("/**") {
            if prefix.contains("*") {
                return Err("single-asterisk key patterns not yet supported");
            }

            return Ok(Self::Prefix(prefix.to_owned()));
        }

        if s.contains("*") {
            return Err("single-asterisk key patterns not yet supported");
        }

        // FIXME: Could forbid special characters other than `*`
        // but they'll just never match anything so skipping that for now.
        Ok(Self::Exactly(s.to_owned()))
    }
}
