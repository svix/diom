use std::{fmt, ops::Deref, sync::Arc};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq)]
pub struct ByteString(Arc<[u8]>);

impl fmt::Debug for ByteString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ByteString {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u8]> for ByteString {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self(value.into())
    }
}

impl From<Vec<u8>> for ByteString {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self(value.into())
    }
}

impl<const N: usize> From<&[u8; N]> for ByteString {
    #[inline]
    fn from(value: &[u8; N]) -> Self {
        value.as_slice().into()
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for ByteString {
    #[inline]
    fn eq(&self, other: &&[u8; N]) -> bool {
        *self.0 == **other
    }
}

impl Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_bytes::Bytes::new(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde_bytes::deserialize(deserializer).map(|vec: Vec<u8>| Self(vec.into()))
    }
}

impl JsonSchema for ByteString {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ByteString".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <Vec<u8>>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        true
    }
}
