use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use serde::{Deserialize, Serialize, de};
use uuid::Uuid;

use super::{Id, PublicIdMarker};

/// serde wrapper for ID types that uses a verbose format (using `M`s prefix).
pub struct Public<T>(T);

impl<T> Public<T> {
    pub(super) fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<M: PublicIdMarker> Serialize for Public<Id<M>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        encode_base32(M::PREFIX, &self.0.inner).serialize(serializer)
    }
}

impl<'de, M: PublicIdMarker> Deserialize<'de> for Public<Id<M>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Id::from_uuid(
            deserializer.deserialize_str(IdVisitor(M::PREFIX))?,
        )))
    }
}

struct IdVisitor(&'static str);

impl<'de> de::Visitor<'de> for IdVisitor {
    type Value = Uuid;

    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "an ID prefixed by {}", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        decode_base32(self.0, v).map_err(de::Error::custom)
    }
}

/// Base32 encoding with a non-standard, but good dictionary.
///
/// - The decoded and encoded form have the same sorting behavior
/// - No confusables
/// - The first 16 chars are the same as in lowercase hex
const BASE32_ENCODING: Encoding = new_encoding! {
    symbols: "0123456789abcdefghjkmnpqrstvwxyz",
};

fn encode_base32(prefix: &'static str, uuid: &Uuid) -> String {
    let mut result = String::with_capacity(prefix.len() + BASE32_ENCODING.encode_len(16));
    result.push_str(prefix);
    BASE32_ENCODING.encode_append(uuid.as_bytes(), &mut result);
    result
}

fn decode_base32(prefix: &'static str, input: &str) -> Result<Uuid, String> {
    let rest = input
        .strip_prefix(prefix)
        .ok_or_else(|| format!("expected prefix `{prefix}` not found"))?;
    if rest.len() != 26 {
        return Err("invalid ID: wrong length".to_owned());
    }

    let mut bytes = [0; 16];
    BASE32_ENCODING
        .decode_mut(rest.as_bytes(), &mut bytes)
        .map_err(|e| format!("invalid ID: {}", e.error))?;
    Ok(Uuid::from_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use uuid::uuid;

    use super::{decode_base32, encode_base32};

    #[test]
    fn test_custom_base32() {
        let encoded = [
            "prefix_00000000000000000000000000",
            "prefix_71kvdwyvp17fb03r6s49e5afv4",
            "prefix_71kvdwyvp17fb03r6s49e5afv8",
            "prefix_71kvdwyvp17fb03r6s49e5afvw",
            "prefix_n1kvdwyvp17fb03r6s49e5afv4",
            "prefix_s1kvdwyvp17fb03r6s49e5afv4",
            "prefix_zzzzzzzzzzzzzzzzzzzzzzzzzw",
        ];

        let decoded = [
            uuid!("00000000-0000-0000-0000-000000000000"),
            uuid!("3867b6f3-dbb0-4ef5-8078-364897154fd9"),
            uuid!("3867b6f3-dbb0-4ef5-8078-364897154fda"),
            uuid!("3867b6f3-dbb0-4ef5-8078-364897154fdf"),
            uuid!("a867b6f3-dbb0-4ef5-8078-364897154fd9"),
            uuid!("c867b6f3-dbb0-4ef5-8078-364897154fd9"),
            uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff"),
        ];

        assert!(encoded.is_sorted());
        assert!(decoded.is_sorted());

        for (e, d) in zip(encoded, decoded) {
            assert_eq!(e, encode_base32("prefix_", &d));
            assert_eq!(decode_base32("prefix_", e).unwrap(), d);
        }
    }
}
