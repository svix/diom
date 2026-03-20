use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Id, marker::IdMarker};

/// serde wrapper for ID types that uses a compat format.
pub struct Internal<T>(T);

impl<T> Internal<T> {
    pub(super) fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<M: IdMarker> Serialize for Internal<Id<M>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.inner.serialize(serializer)
    }
}

impl<'de, M: IdMarker> Deserialize<'de> for Internal<Id<M>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Id::from_uuid(Uuid::deserialize(deserializer)?)))
    }
}
