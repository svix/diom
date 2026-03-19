use std::{fmt, marker::PhantomData};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod internal;
mod public;
#[macro_use]
mod marker;

use self::marker::{IdMarker, InternalUse, PublicIdMarker};
pub use self::{internal::Internal, public::Public};

pub type NamespaceId = Id<m::Namespace>;
pub type TopicId = Id<m::Topic>;

// Don't want these types to be nameable,
// so they're defined in a private submodule
mod m {
    // Markers for internal-only IDs
    id_marker!(Namespace);
    id_marker!(Topic);

    // Markers for public IDs
    // id_marker!(Foo, "foo_");
}

pub struct Id<M> {
    inner: Uuid,
    _marker: PhantomData<M>,
}

impl<M: IdMarker> Id<M> {
    fn from_uuid(uuid: Uuid) -> Id<M> {
        Self {
            inner: uuid,
            _marker: PhantomData,
        }
    }

    pub fn new(now: Timestamp) -> Self {
        Self::from_uuid(Uuid::new_v7(uuid::Timestamp::from_unix(
            uuid::NoContext,
            now.as_second() as u64,
            now.subsec_nanosecond() as u32,
        )))
    }

    pub fn nil() -> Self {
        Self::from_uuid(Uuid::nil())
    }

    pub fn max() -> Self {
        Self::from_uuid(Uuid::max())
    }

    pub fn from_slice(s: &[u8]) -> Result<Self, uuid::Error> {
        Uuid::from_slice(s).map(Self::from_uuid)
    }

    pub fn internal(self) -> Internal<Self> {
        Internal::new(self)
    }

    pub fn public(self) -> Public<Self>
    where
        M: PublicIdMarker,
    {
        Public::new(self)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        self.inner.as_bytes()
    }
}

impl<M> Clone for Id<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for Id<M> {}

impl<M> fmt::Debug for Id<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<M> Serialize for Id<M>
where
    // if you get a trait solving error about this bound,
    // wrap the ID in `id::Public<_>` or `id::Internal<_>`
    // to select a serialization format explicitly.
    M: IdMarker<Use = InternalUse>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.internal().serialize(serializer)
    }
}

impl<'de, M> Deserialize<'de> for Id<M>
where
    // if you get a trait solving error about this bound,
    // wrap the ID in `id::Public<_>` or `id::Internal<_>`
    // to select a serialization format explicitly.
    M: IdMarker<Use = InternalUse>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Id::from_uuid(Uuid::deserialize(deserializer)?))
    }
}

#[cfg(test)]
#[allow(unreachable_pub)]
mod tests {
    use super::Id;

    id_marker!(Private);
    id_marker!(Public, "pub_");

    type PrivateId = Id<Private>;
    type PublicId = Id<Public>;

    #[test]
    fn private_marker_serde() {
        let id = PrivateId::new(jiff::Timestamp::now());

        assert_eq!(size_of_val(&id), 16); // 16 bytes, i.e. just a UUID

        let serialized = rmp_serde::to_vec_named(&id).unwrap();
        assert_eq!(serialized.len(), 18);
        assert_eq!(&serialized[2..], id.as_bytes());
    }

    #[test]
    fn public_marker_serde() {
        let id = PublicId::new(jiff::Timestamp::now());

        assert_eq!(size_of_val(&id), 16); // 16 bytes, also just a UUID

        let serialized = rmp_serde::to_vec_named(&id.internal()).unwrap();
        assert_eq!(serialized.len(), 18);
        assert_eq!(&serialized[2..], id.as_bytes());

        let serialized = rmp_serde::to_vec_named(&id.public()).unwrap();
        assert_eq!(serialized.len(), 38);
        assert_eq!(&serialized[2..][..4], b"pub_");
    }
}
