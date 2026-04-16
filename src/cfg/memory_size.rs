use std::{collections::HashMap, fmt, sync::LazyLock};

use serde::de::{self, Visitor};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemorySize {
    Bytes(u64),
    Percent(u8),
}

static SYSTEM_MEMORY: LazyLock<u64> = LazyLock::new(|| {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    if let Some(limit) = sys.cgroup_limits() {
        limit.total_memory
    } else {
        sys.total_memory()
    }
});

impl MemorySize {
    pub fn as_bytes(&self) -> u64 {
        match self {
            Self::Percent(pct) => {
                if *SYSTEM_MEMORY == 0 {
                    tracing::warn!(
                        pct,
                        "unable to determine system memory, falling back from percentage to fixed size for cache"
                    );
                    Self::fjall_default_cache().as_bytes()
                } else {
                    (*pct as u64) * *SYSTEM_MEMORY / 100
                }
            }
            Self::Bytes(bytes) => *bytes,
        }
    }

    pub fn fjall_default_cache() -> Self {
        Self::Bytes(32 * 1024 * 1024) // match fjall default of 32MB
    }
}

impl serde::ser::Serialize for MemorySize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bytes(bval) => bval.to_string().serialize(serializer),
            Self::Percent(pct) => format!("{pct}%").serialize(serializer),
        }
    }
}

static UNIT_SUFFIXES: LazyLock<HashMap<&'static str, u64>> = LazyLock::new(|| {
    maplit::hashmap! {
        "b" => 1,
        "kb" => 1000,
        "kib" => 1024,
        "mb" => 1_000_000,
        "mib" => 1024 * 1024,
        "gb" => 1_000_000_000,
        "gib" => 1024 * 1024 * 1024,
        "tb" => 1_000_000_000_000,
        "tib" => 1024 * 1024 * 1024 * 1024
    }
});

struct MemorySizeVisitor;

impl<'de> Visitor<'de> for MemorySizeVisitor {
    type Value = MemorySize;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid byte size")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // strip any whitespace
        let v = v.replace(|c: char| c == '_' || c.is_ascii_whitespace(), "");
        // look for the unit part
        if let Some(unit_index) = v.find(|f: char| !f.is_ascii_digit()) {
            let prefix = &v[..unit_index];
            let suffix = &v[unit_index..];
            if suffix == "%" {
                let numeric_part: u8 = prefix
                    .parse()
                    .map_err(|_| de::Error::custom("prefix could not be parsed as an integer"))?;
                Ok(MemorySize::Percent(numeric_part))
            } else {
                let Some(multiplier) = UNIT_SUFFIXES.get(suffix.to_lowercase().as_str()) else {
                    return Err(de::Error::custom(format!(
                        "suffix {suffix} is not recognized"
                    )));
                };
                let numeric_part: u64 = prefix
                    .parse()
                    .map_err(|_| de::Error::custom("prefix could not be parsed as an integer"))?;
                Ok(MemorySize::Bytes(numeric_part * *multiplier))
            }
        } else {
            // great, they're all digits
            let number: u64 = v
                .parse()
                .map_err(|_| de::Error::custom("could not be parsed as an integer"))?;
            Ok(MemorySize::Bytes(number))
        }
    }
}

impl<'de> de::Deserialize<'de> for MemorySize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(MemorySizeVisitor)
    }
}

crate::from_str_via_serde!(MemorySize);

#[cfg(test)]
mod tests {
    use super::MemorySize;
    use std::str::FromStr;

    const VECTORS: &[(&str, MemorySize)] = &[
        ("1", MemorySize::Bytes(1)),
        ("1_024_000", MemorySize::Bytes(1024000)),
        ("1kb", MemorySize::Bytes(1000)),
        ("1KiB", MemorySize::Bytes(1024)),
        ("1mb", MemorySize::Bytes(1000000)),
        ("1MiB", MemorySize::Bytes(1048576)),
        ("1_024 KiB", MemorySize::Bytes(1048576)),
        ("50%", MemorySize::Percent(50)),
    ];

    const INVALID_STRINGS: &[&str] = &[
        "all of it",
        "1 byte",
        "1024MB and also some garbage",
        "1.44MB",
    ];

    #[test]
    fn test_fromstr() {
        for (input, expected) in VECTORS {
            let parsed: MemorySize = input.parse().expect("should parse");
            assert_eq!(parsed, *expected);
        }
        for input in INVALID_STRINGS {
            MemorySize::from_str(input).expect_err("should fail to parse");
        }
    }

    #[test]
    fn test_serde_deserialize() {
        for (input, expected) in VECTORS {
            let wrapped = serde_json::json! { input };
            let parsed: MemorySize = serde_json::value::from_value(wrapped).expect("should parse");
            assert_eq!(parsed, *expected);
            let serialized = serde_json::to_string(&parsed).expect("should re-serialize");
            let deserialized: MemorySize =
                serde_json::from_str(&serialized).expect("should re-deserialize");
            assert_eq!(deserialized, *expected);
        }
        for input in INVALID_STRINGS {
            let wrapped = serde_json::json! { input };
            serde_json::value::from_value::<MemorySize>(wrapped).expect_err("should fail to parse");
        }
    }

    #[test]
    fn test_as_bytes() {
        assert_eq!(MemorySize::Bytes(2048).as_bytes(), 2048);
        assert!(MemorySize::Percent(100).as_bytes() > 32 * 1024 * 1024);
    }
}
