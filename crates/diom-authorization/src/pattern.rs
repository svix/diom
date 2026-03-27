use std::{
    borrow::Cow,
    fmt::{self, Write},
    str::FromStr,
};

use diom_id::Module;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de};

use crate::RequestedOperation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePattern {
    pub module: ModulePattern,
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
pub enum ModulePattern {
    /// Wildcard (`*`).
    ///
    /// Does not match admin modules.
    Any,
    Exactly(Module),
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
    Segmented(SegmentedKeyPattern),
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SegmentedKeyPattern {
    pub segments: Vec<KeyPatternSegment>,
    /// Whether the pattern has a trailing `*` segment.
    ///
    /// `*` is only allowed at the very end.
    pub trailing_any: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyPatternSegment {
    Fixed(String),
    Placeholder(String),
}

impl ResourcePattern {
    pub fn matches(&self, op: &RequestedOperation<'_>) -> bool {
        self.module.matches(op.module)
            && self.namespace.matches(op.namespace)
            && self.key.matches(op.key)
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

impl ModulePattern {
    fn matches(&self, module: Module) -> bool {
        match self {
            Self::Any => !module.is_admin_module(),
            Self::Exactly(m) => module == *m,
        }
    }
}

impl fmt::Display for ModulePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => f.write_str("*"),
            Self::Exactly(m) => m.fmt(f),
        }
    }
}

impl FromStr for ModulePattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Any),
            _ => s.parse().map(Self::Exactly),
        }
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
            Self::Segmented(s) => key.is_some_and(|k| s.matches(k)),
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
            Self::Segmented(s) => s.fmt(f),
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
            if s.contains(['$', '{', '}']) {
                return s.parse::<SegmentedKeyPattern>().map(Self::Segmented);
            }

            let prefix = &s[..s.len() - 1]; // keep the /
            return Ok(Self::Prefix(prefix.to_owned()));
        }

        if s.contains(['$', '*', '{', '}']) {
            return s.parse::<SegmentedKeyPattern>().map(Self::Segmented);
        }

        // FIXME: Could forbid special characters other than `*`
        // but they'll just never match anything so skipping that for now.
        Ok(Self::Exactly(s.to_owned()))
    }
}

impl SegmentedKeyPattern {
    fn matches(&self, key: &str) -> bool {
        let mut pat_segments = self.segments.iter();
        for key_seg in key.split('/') {
            let Some(pat_seg) = pat_segments.next() else {
                // key has more segments than pattern (excl. trailing `*`)
                // if trailing `*` exists, match.
                // if it doesn't, no match.
                return self.trailing_any;
            };

            if !pat_seg.matches(key_seg) {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for SegmentedKeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = self.segments.iter();
        segments
            .next()
            .expect("SegmentedKeyPattern always consists of at least one segment")
            .fmt(f)?;

        for segment in segments {
            f.write_char('/')?;
            segment.fmt(f)?;
        }

        if self.trailing_any {
            f.write_str("/*")?;
        }

        Ok(())
    }
}

impl FromStr for SegmentedKeyPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, trailing_any) = match s.strip_suffix("/*") {
            Some(rest) => (rest, true),
            None => (s, false),
        };

        let segments = s
            .split('/')
            .map(KeyPatternSegment::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            segments,
            trailing_any,
        })
    }
}

impl KeyPatternSegment {
    fn matches(&self, key_seg: &str) -> bool {
        match self {
            Self::Fixed(s) => s == key_seg,
            // FIXME: Add support for context stuff
            Self::Placeholder(_) => false,
        }
    }
}

impl fmt::Display for KeyPatternSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed(s) => f.write_str(s),
            Self::Placeholder(p) => write!(f, "${{{p}}}"),
        }
    }
}

impl FromStr for KeyPatternSegment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('*') {
            return Err("asterisk may only be used as a standalone slash-separated segment");
        }

        if let Some(rest) = s.strip_prefix("${")
            && let Some(placeholder) = rest.strip_suffix('}')
        {
            // FIXME: probably disallow most characters inside?
            // Though the pattern will just never match if invalid anyways.
            return Ok(Self::Placeholder(placeholder.to_owned()));
        }

        if s.contains(['$', '{', '}']) {
            // FIXME: could use a better error message
            return Err("invalid key pattern segment");
        }

        Ok(Self::Fixed(s.to_owned()))
    }
}
