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
        // FIXME: We may want to switch to a more compact form
        //        (make sure to also update Deserialize, of course)
        format!("{}{}", M::PREFIX, self.0.inner.as_simple()).serialize(serializer)
    }
}

impl<'de, M: PublicIdMarker> Deserialize<'de> for Public<Id<M>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
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
                let rest = v
                    .strip_prefix(self.0)
                    .ok_or_else(|| de::Error::invalid_value(de::Unexpected::Str(v), &self))?;
                Uuid::parse_str(rest).map_err(|e| de::Error::custom(format!("invalid ID: {e}")))
            }
        }

        Ok(Self(Id::from_uuid(
            deserializer.deserialize_str(IdVisitor(M::PREFIX))?,
        )))
    }
}
