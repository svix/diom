use std::{
    fmt::{self, Write},
    str::FromStr,
};

use diom_core::PersistableValue;

use crate::Context;

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
pub enum NamespacePattern {
    Default,
    Named(String),
    Any,
    // FIXME: Do namespaces have any sort of hierarchy?
}

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
pub struct KeyPattern {
    pub segments: Vec<KeyPatternSegment>,
    /// Whether the pattern has a trailing `*` segment.
    ///
    /// `*` is only allowed at the very end.
    pub trailing_any: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PersistableValue)]
pub enum KeyPatternSegment {
    Fixed(String),
    Placeholder(String),
}

impl NamespacePattern {
    pub(crate) fn is_reserved(&self) -> bool {
        matches!(self, Self::Named(ns) if ns.starts_with('_'))
    }

    pub(crate) fn matches(&self, namespace: Option<&str>) -> bool {
        match self {
            Self::Default => namespace.is_none(),
            Self::Named(ns) => namespace == Some(ns),
            Self::Any => namespace.is_none_or(|ns| !ns.starts_with("_")),
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
    pub fn any() -> Self {
        Self {
            segments: vec![],
            trailing_any: true,
        }
    }

    pub(crate) fn matches(&self, key: Option<&str>, context: Context<'_>) -> bool {
        let Some(key) = key else {
            return self.segments.is_empty() && self.trailing_any;
        };

        let mut pat_segments = self.segments.iter();
        for key_seg in key.split('/') {
            let Some(pat_seg) = pat_segments.next() else {
                // key has more segments than pattern (excl. trailing `*`)
                // if trailing `*` exists, match.
                // if it doesn't, no match.
                return self.trailing_any;
            };

            if !pat_seg.matches(key_seg, context) {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for KeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut segments = self.segments.iter();
        if let Some(first) = segments.next() {
            first.fmt(f)?;

            for segment in segments {
                f.write_char('/')?;
                segment.fmt(f)?;
            }

            if self.trailing_any {
                f.write_str("/*")?;
            }
        } else if self.trailing_any {
            f.write_char('*')?;
        }

        Ok(())
    }
}

impl FromStr for KeyPattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            return Ok(Self {
                segments: vec![],
                trailing_any: true,
            });
        }

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
    fn matches(&self, key_seg: &str, context: Context<'_>) -> bool {
        match self {
            Self::Fixed(s) => s == key_seg,
            Self::Placeholder(p) => context.get(p).is_some_and(|c| c == key_seg),
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
