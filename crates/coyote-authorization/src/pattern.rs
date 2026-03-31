use std::{borrow::Cow, fmt, str::FromStr};

use coyote_id::Module;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de};

use crate::RequestedOperation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePattern {
    pub module: Module,
    pub namespace: NamespacePattern,
    pub key: KeyPattern,
}

impl JsonSchema for ResourcePattern {
    fn schema_name() -> Cow<'static, str> {
        String::schema_name()
    }

    fn inline_schema() -> bool {
        true
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NamespacePattern {
    Default,
    Named(String),
    Any,
    // FIXME: Do namespaces have any sort of hierarchy?
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyPattern {
    Exactly(String),
    Prefix(String),
    Any,
    // FIXME: Add single star wildcards
}

impl ResourcePattern {
    pub fn matches(&self, op: &RequestedOperation<'_>) -> bool {
        self.module == op.module && self.namespace.matches(op.namespace) && self.key.matches(op.key)
    }
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

impl NamespacePattern {
    fn matches(&self, namespace: Option<&str>) -> bool {
        match self {
            Self::Default => namespace.is_none(),
            Self::Named(ns) => namespace == Some(ns),
            Self::Any => true,
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

impl KeyPattern {
    fn matches(&self, key: Option<&str>) -> bool {
        match self {
            Self::Exactly(k) => key == Some(k),
            Self::Prefix(p) => key.is_some_and(|k| k.starts_with(p)),
            Self::Any => true,
        }
    }
}

impl fmt::Display for KeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exactly(s) => f.write_str(s),
            Self::Prefix(s) => {
                f.write_str(s)?;
                f.write_str("*")
            }
            Self::Any => f.write_str("*"),
        }
    }
}

impl FromStr for KeyPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            return Ok(Self::Any);
        }

        if s.ends_with("/*") {
            let prefix = &s[..s.len() - 1]; // keep the /
            return Ok(Self::Prefix(prefix.to_owned()));
        }

        if s.contains("*") {
            return Err("wildcard in key pattern mut be at the end");
        }

        // FIXME: Could forbid special characters other than `*`
        // but they'll just never match anything so skipping that for now.
        Ok(Self::Exactly(s.to_owned()))
    }
}
