use std::{fmt, marker::PhantomData};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::{Builder, Uuid};

mod public;
#[macro_use]
mod marker;
mod module;

use self::marker::{IdMarker, PublicIdMarker};
pub use self::{module::Module, public::Public};

const V7_RANDOM_BYTES_LEN: usize = 10;
pub type UuidV7RandomBytes = [u8; V7_RANDOM_BYTES_LEN];

pub type AuthTokenId = Id<m::AuthToken>;
pub type NamespaceId = Id<m::Namespace>;
pub type TopicId = Id<m::Topic>;

pub fn random_v7_bytes() -> UuidV7RandomBytes {
    rand::random()
}

// Don't want these types to be nameable,
// so they're defined in a private submodule
mod m {
    // Markers for internal-only IDs
    id_marker!(Namespace);
    id_marker!(Topic);

    // Markers for public IDs
    id_marker!(AuthToken, "key_");
}

#[derive(PartialEq, Eq, Hash)]
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

    pub fn new(now: Timestamp, random_bytes: UuidV7RandomBytes) -> Self {
        let millis = now.as_millisecond() as u64;
        Self::from_uuid(Builder::from_unix_timestamp_millis(millis, &random_bytes).into_uuid())
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

    /// Wrap this Id in `Public<_>`, changing its serialization format to a less compact one
    /// meant for use in the public API.
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

impl<M: IdMarker> Serialize for Id<M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, M: IdMarker> Deserialize<'de> for Id<M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_uuid(Uuid::deserialize(deserializer)?))
    }
}

// DO NOT implement JsonSchema for Id<M> (Public<Id<M>> should be used)

#[cfg(test)]
#[allow(unreachable_pub)]
mod tests {
    use super::{Id, V7_RANDOM_BYTES_LEN};

    id_marker!(Private);
    id_marker!(Public, "pub_");

    type PrivateId = Id<Private>;
    type PublicId = Id<Public>;

    #[test]
    fn private_id_serde() {
        let id = PrivateId::new(jiff::Timestamp::UNIX_EPOCH, [0; V7_RANDOM_BYTES_LEN]);

        assert_eq!(size_of_val(&id), 16); // 16 bytes, i.e. just a UUID

        let serialized = rmp_serde::to_vec_named(&id).unwrap();
        assert_eq!(serialized.len(), 18);
        assert_eq!(&serialized[2..], id.as_bytes());
    }

    #[test]
    fn public_id_serde() {
        let id = PublicId::new(jiff::Timestamp::UNIX_EPOCH, [0; V7_RANDOM_BYTES_LEN]);

        assert_eq!(size_of_val(&id), 16); // 16 bytes, also just a UUID

        let serialized = rmp_serde::to_vec_named(&id).unwrap();
        assert_eq!(serialized.len(), 18);
        assert_eq!(&serialized[2..], id.as_bytes());

        let serialized = rmp_serde::to_vec_named(&id.public()).unwrap();
        assert_eq!(serialized.len(), 31);
        assert_eq!(&serialized[1..][..4], b"pub_");
    }

    #[test]
    fn new_is_deterministic() {
        let now = jiff::Timestamp::UNIX_EPOCH;
        let random_bytes = [7; V7_RANDOM_BYTES_LEN];

        assert_eq!(
            PrivateId::new(now, random_bytes),
            PrivateId::new(now, random_bytes)
        );
    }
}
