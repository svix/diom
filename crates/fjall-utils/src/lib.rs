#![warn(clippy::str_to_string)]

#[cfg(test)]
extern crate self as fjall_utils;

mod db;
pub mod duration_millis;
mod fixed_key;
pub mod fixtures;
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
pub use diom_derive::{FjallKey, FjallKeyComponent};
pub use fjall::UserKey;

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

    // `Fixture` and `FixtureOld` model a row serialized in the db prior to this addition
    // of `PersistableVersioned`, and then being swapped to use PersistableVersioned
    // later on, so we can prove backwards compatibility.

    #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
    #[versioned(row_type = 0)]
    struct Fixture {
        a: u32,
        b: u32,
        #[since(1)]
        c: Option<u32>,
    }

    #[derive(serde::Serialize, serde::Deserialize, diom_core::PersistableValue)]
    struct FixtureOld {
        a: u32,
        b: u32,
    }

    impl TableRow for FixtureOld {
        const ROW_TYPE: u8 = 0;
    }

    #[test]
    fn versioned_round_trips_current_version() {
        assert_eq!(Fixture::WRITE_VERSION, 1);

        let value = Fixture {
            a: 1,
            b: 2,
            c: Some(3),
        };
        let bytes = value.to_fjall_value().unwrap();
        let decoded = Fixture::from_fjall_value(bytes).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn versioned_decodes_legacy_v0_with_defaulted_field() {
        let legacy = FixtureOld { a: 10, b: 20 }.to_fjall_value().unwrap();
        let decoded = Fixture::from_fjall_value(legacy).unwrap();
        assert_eq!(
            decoded,
            Fixture {
                a: 10,
                b: 20,
                c: None,
            }
        );
    }

    #[test]
    fn versioned_all_v0_matches_old_encoding() {
        #[derive(diom_core::PersistableVersioned)]
        #[versioned(row_type = 3)]
        struct FixtureAllV0 {
            a: u32,
            b: u32,
        }
        assert_eq!(FixtureAllV0::WRITE_VERSION, 0);

        let old = FixtureOld { a: 10, b: 20 }.to_fjall_value().unwrap();
        let versioned = FixtureAllV0 { a: 10, b: 20 }.to_fjall_value().unwrap();
        assert_eq!(old, versioned);
    }

    /// A non-Option field added in a later version falls back to its `#[since(.., default = ..)]`
    /// expression when reading data that predates the field.
    #[test]
    fn versioned_uses_explicit_default_for_legacy_field() {
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 4)]
        struct WithDefault {
            a: u32,
            #[since(1, default = 99)]
            b: u32,
        }

        // The same row before `b` existed.
        #[derive(serde::Serialize, serde::Deserialize, diom_core::PersistableValue)]
        struct WithDefaultOld {
            a: u32,
        }
        impl TableRow for WithDefaultOld {
            const ROW_TYPE: u8 = 4;
        }

        let legacy = WithDefaultOld { a: 10 }.to_fjall_value().unwrap();
        let decoded = WithDefault::from_fjall_value(legacy).unwrap();
        assert_eq!(decoded, WithDefault { a: 10, b: 99 });

        // A value written at the current version keeps its real `b`.
        let current = WithDefault { a: 10, b: 7 }.to_fjall_value().unwrap();
        assert_eq!(
            WithDefault::from_fjall_value(current).unwrap(),
            WithDefault { a: 10, b: 7 }
        );
    }

    #[test]
    fn versioned_reads_all_prior_versions_at_v2() {
        // The row before any fields were added.
        #[derive(serde::Serialize, serde::Deserialize, diom_core::PersistableValue)]
        struct RowV0 {
            a: u32,
        }
        impl TableRow for RowV0 {
            const ROW_TYPE: u8 = 5;
        }

        // The row after `b` was added in version 1.
        #[derive(diom_core::PersistableVersioned)]
        #[versioned(row_type = 5)]
        struct RowV1 {
            a: u32,
            #[since(1)]
            b: Option<u32>,
        }

        // The current row, with `c` added in version 2.
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 5)]
        struct RowV2 {
            a: u32,
            #[since(1)]
            b: Option<u32>,
            #[since(2)]
            c: Option<u32>,
        }

        assert_eq!(RowV1::WRITE_VERSION, 1);
        assert_eq!(RowV2::WRITE_VERSION, 2);

        let v0 = RowV0 { a: 1 }.to_fjall_value().unwrap();
        assert_eq!(
            RowV2::from_fjall_value(v0).unwrap(),
            RowV2 {
                a: 1,
                b: None,
                c: None,
            }
        );

        let v1 = RowV1 { a: 1, b: Some(2) }.to_fjall_value().unwrap();
        assert_eq!(
            RowV2::from_fjall_value(v1).unwrap(),
            RowV2 {
                a: 1,
                b: Some(2),
                c: None,
            }
        );

        let v2 = RowV2 {
            a: 1,
            b: Some(2),
            c: Some(3),
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowV2::from_fjall_value(v2).unwrap(),
            RowV2 {
                a: 1,
                b: Some(2),
                c: Some(3),
            }
        );
    }

    #[test]
    fn versioned_forward_compatibility() {
        #[derive(diom_core::PersistableVersioned)]
        #[versioned(row_type = 6)]
        struct RowV2 {
            a: u32,
            #[since(1)]
            b: Option<u32>,
            #[since(2)]
            c: Option<u32>,
        }

        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 6)]
        struct RowV1 {
            a: u32,
            #[since(1)]
            b: Option<u32>,
        }

        let newer = RowV2 {
            a: 1,
            b: Some(2),
            c: Some(3),
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowV1::from_fjall_value(newer).unwrap(),
            RowV1 { a: 1, b: Some(2) }
        );
    }

    // A `#[nested]` versioned struct can grow a field even when it is not the last field of the row,
    // in both directions. Without `#[nested]` the new field would bleed into `tail`.
    #[test]
    fn nested_field_grows_mid_row() {
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct ProfileOld {
            x: u32,
        }
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct ProfileNew {
            x: u32,
            #[since(1)]
            y: Option<u32>,
        }

        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 7)]
        struct RowOld {
            #[nested]
            profile: ProfileOld,
            tail: u32,
        }
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 7)]
        struct RowNew {
            #[nested]
            profile: ProfileNew,
            tail: u32,
        }

        // Make sure it's forward compatible
        let new = RowNew {
            profile: ProfileNew { x: 1, y: Some(2) },
            tail: 9,
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowOld::from_fjall_value(new).unwrap(),
            RowOld {
                profile: ProfileOld { x: 1 },
                tail: 9,
            }
        );

        // Make sure it's backwards compatible
        let old = RowOld {
            profile: ProfileOld { x: 1 },
            tail: 9,
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowNew::from_fjall_value(old).unwrap(),
            RowNew {
                profile: ProfileNew { x: 1, y: None },
                tail: 9,
            }
        );
    }

    // A field can be both `#[since(n)]` and `#[nested]`, so a nested struct can be introduced in a
    // later version
    #[test]
    fn nested_field_added_in_later_version() {
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct Profile {
            x: u32,
        }

        // The row before the nested `profile` field existed.
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 8)]
        struct RowV0 {
            a: u32,
        }
        // The row after `profile` was added in version 1.
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 8)]
        struct RowV1 {
            a: u32,
            #[since(1)]
            #[nested]
            profile: Option<Profile>,
        }

        // Reading a v0 row defaults the nested field.
        let v0 = RowV0 { a: 5 }.to_fjall_value().unwrap();
        assert_eq!(
            RowV1::from_fjall_value(v0).unwrap(),
            RowV1 {
                a: 5,
                profile: None
            }
        );

        let v1 = RowV1 {
            a: 5,
            profile: Some(Profile { x: 7 }),
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowV1::from_fjall_value(v1).unwrap(),
            RowV1 {
                a: 5,
                profile: Some(Profile { x: 7 }),
            }
        );
    }

    #[test]
    fn nested_composes_recursively() {
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct InnerOld {
            x: u32,
        }
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct InnerNew {
            x: u32,
            #[since(1)]
            z: Option<u32>,
        }

        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct OuterOld {
            #[nested]
            inner: InnerOld,
            m: u32,
        }
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        struct OuterNew {
            #[nested]
            inner: InnerNew,
            m: u32,
        }

        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 9)]
        struct RowOld {
            #[nested]
            outer: OuterOld,
            tail: u32,
        }
        #[derive(diom_core::PersistableVersioned, PartialEq, Debug)]
        #[versioned(row_type = 9)]
        struct RowNew {
            #[nested]
            outer: OuterNew,
            tail: u32,
        }

        // The innermost struct grows a field. Old code still reads m and tail correctly around it.
        let new = RowNew {
            outer: OuterNew {
                inner: InnerNew { x: 1, z: Some(2) },
                m: 3,
            },
            tail: 9,
        }
        .to_fjall_value()
        .unwrap();
        assert_eq!(
            RowOld::from_fjall_value(new).unwrap(),
            RowOld {
                outer: OuterOld {
                    inner: InnerOld { x: 1 },
                    m: 3,
                },
                tail: 9,
            }
        );
    }
}
