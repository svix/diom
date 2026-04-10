#[allow(unused_extern_crates)]
extern crate self as fjall_utils;

mod db;
pub mod duration_millis;
mod fixed_key;
mod fjall_key_able;
mod options;
mod readonly_db;
mod table_row;

pub use self::{
    db::{Databases, ReadonlyConnection, ReadonlyDatabases, StorageType},
    fixed_key::FjallFixedKey,
    fjall_key_able::*,
    options::{SchemaManifest, SerializableKeyspaceCreateOptions},
    readonly_db::{ReadableDatabase, ReadableKeyspace, ReadonlyDatabase, ReadonlyKeyspace},
    table_row::{
        KeyspaceExt, MonotonicTableKey, MonotonicTableRow, TableKey, TableKeyFromFjall,
        TableKeyType, TableRow, WriteBatchExt,
    },
};
pub use diom_derive::FjallKeyAble;

/// Version envelope for values stored in fjall. The serialized form starts
/// with a varint discriminant (0x00 for V0), leaving room for future migrations.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum V0Wrapper<T> {
    V0(T),
}

/// Serialize `value` directly into a [`byteview::ByteView`],
/// avoiding an intermediate `Vec<u8>` allocation.
pub(crate) fn postcard_to_byteview(
    value: &impl serde::Serialize,
) -> Result<byteview::ByteView, postcard::Error> {
    use postcard::ser_flavors;
    let size = postcard::serialize_with_flavor(value, ser_flavors::Size::default())?;
    let mut builder = byteview::ByteView::builder(size);
    postcard::to_slice(value, &mut builder)?;
    Ok(builder.freeze())
}

/// Useful for verifying all table prefixes for a given keyspace are unique,
/// at compile time.
pub const fn are_all_unique(strings: &[&str]) -> bool {
    let mut i = 0;
    while i < strings.len() {
        let mut j = i + 1;
        while j < strings.len() {
            if str_eq(strings[i], strings[j]) {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const fn str_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    let mut i = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_unique() {
        assert!(are_all_unique(&["a", "b", "c"]));
        assert!(are_all_unique(&["foo", "bar", "baz"]));
        assert!(are_all_unique(&[""]));
        assert!(are_all_unique(&[]));
        assert!(!are_all_unique(&["a", "a"]));
        assert!(!are_all_unique(&["foo", "bar", "foo"]));
    }

    #[test]
    fn v0_wrapper_starts_with_zero() {
        let bytes = postcard::to_allocvec(&V0Wrapper::V0(42u32)).unwrap();
        assert_eq!(bytes[0], 0x00, "V0 discriminant must be the first byte");
    }

    #[test]
    fn v0_wrapper_roundtrip() {
        let original = 0xdeadbeef_u32;
        let bytes = postcard::to_allocvec(&V0Wrapper::V0(original)).unwrap();
        let V0Wrapper::V0(recovered) = postcard::from_bytes::<V0Wrapper<u32>>(&bytes).unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn v0_wrapper_payload_matches_bare_encoding() {
        // The bytes after the leading 0x00 should be identical to encoding the
        // inner value directly, so existing logic only needs to handle the prefix.
        let inner = 12345u32;
        let wrapped = postcard::to_allocvec(&V0Wrapper::V0(inner)).unwrap();
        let bare = postcard::to_allocvec(&inner).unwrap();
        assert_eq!(wrapped[0], 0x00);
        assert_eq!(&wrapped[1..], bare.as_slice());
    }

    #[test]
    fn postcard_to_byteview_roundtrip() {
        let original = 0xdeadbeef_u32;
        let slice = postcard_to_byteview(&V0Wrapper::V0(original)).unwrap();
        let V0Wrapper::V0(recovered) = postcard::from_bytes::<V0Wrapper<u32>>(&slice).unwrap();
        assert_eq!(recovered, original);
    }

    #[test]
    fn postcard_to_byteview_matches_allocvec() {
        let original = 42u32;
        let via_slice = postcard_to_byteview(&V0Wrapper::V0(original)).unwrap();
        let via_vec = postcard::to_allocvec(&V0Wrapper::V0(original)).unwrap();
        assert_eq!(&*via_slice, via_vec.as_slice());
    }
}
