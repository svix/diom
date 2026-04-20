#![warn(clippy::str_to_string)]

use std::{fmt, marker::PhantomData};

use diom_core::{PersistableValue, types::AsMillisecond};
use serde::{Deserialize, Serialize};
use uuid::{Builder, Uuid};

mod public;
#[macro_use]
mod marker;
mod module;

pub use self::{
    marker::{IdMarker, PublicIdMarker},
    module::Module,
    public::Public,
};

const V7_RANDOM_BYTES_LEN: usize = 10;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PersistableValue)]
#[serde(transparent)]
pub struct UuidV7RandomBytes([u8; V7_RANDOM_BYTES_LEN]);

impl UuidV7RandomBytes {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}

pub type AuthTokenId = Id<m::AuthToken>;
pub type NamespaceId = Id<m::Namespace>;
pub type TopicId = Id<m::Topic>;

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

    pub fn new<TS: AsMillisecond>(now: TS, random_bytes: UuidV7RandomBytes) -> Self {
        let millis = now.as_millisecond();
        Self::from_uuid(Builder::from_unix_timestamp_millis(millis, &random_bytes.0).into_uuid())
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

// we promise to keep the serialization of Id stable
impl<M: IdMarker> PersistableValue for Id<M> {}

// DO NOT implement JsonSchema for Id<M> (Public<Id<M>> should be used)

#[cfg(test)]
#[allow(unreachable_pub)]
mod tests {
    use super::{Id, UuidV7RandomBytes};

    id_marker!(Private);
    id_marker!(Public, "pub_");

    type PrivateId = Id<Private>;
    type PublicId = Id<Public>;

    #[test]
    fn private_id_serde() {
        let id = PrivateId::new(jiff::Timestamp::UNIX_EPOCH, UuidV7RandomBytes::new_random());

        assert_eq!(size_of_val(&id), 16); // 16 bytes, i.e. just a UUID

        let serialized = rmp_serde::to_vec_named(&id).unwrap();
        assert_eq!(serialized.len(), 18);
        assert_eq!(&serialized[2..], id.as_bytes());
    }

    #[test]
    fn public_id_serde() {
        let id = PublicId::new(jiff::Timestamp::UNIX_EPOCH, UuidV7RandomBytes::new_random());

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
        let random_bytes = UuidV7RandomBytes::new_random();

        assert_eq!(
            PrivateId::new(now, random_bytes),
            PrivateId::new(now, random_bytes)
        );
    }
}
