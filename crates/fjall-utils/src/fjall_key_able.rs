use std::{
    borrow::Cow,
    ops::{Bound, RangeBounds},
};

use crate::{TableKey, TableRow};

/// Marks a struct as a key used in Fjall queries, and generates a bunch
/// of efficient helper methods for better querying.
///
/// # Usage
///
/// ```ignore
/// #[derive(FjallKeyAble)]
/// #[table_key(prefix = 10)]
/// struct MessageKey {
///     #[key(0)]
///     queue_id: diom_id::Id<QueueMarker>,
///     #[key(1)]
///     seq: u64,
///     #[key(2)]
///     payload: Vec<u8>, // variable-size, must be last
/// }
/// ```
///
/// ## Requirements
///
/// - `prefix` must be a constant expression that fits in a `u8` (e.g. a literal
///   like `3` or an enum variant like `RowType::Msg`), unique among types sharing
///   a keyspace.
/// - Every field needs a `#[key(N)]` attribute specifying its position. Changing a key's position
///   is a breaking disk change and should be avoided.
/// - All field types must implement [`KeyComponent`]. This is already implemented for most native types.
/// - Only the **last** field (by key index) may be variable-size
///   (`String` or `Vec<u8>`). All others must be fixed-size.
///
/// ## Generated code
///
/// The macro generates:
///
/// - **`FjallKeyAble` trait impl** — `fjall_key()`, `from_fjall_key()`,
///   `range()` (provided default).
/// - **`extract_<field>()` methods** on the struct — read a single field
///   from a raw `fjall::UserKey` without constructing the full struct.
///   Returns the borrowed form where possible (`&str` for `String`,
///   `&[u8]` for `Vec<u8>`, the value directly for fixed-size types).
/// - **`prefix_<field>()` methods** on the struct — build a key prefix
///   from the leading fields up to and including the named field.
///   Generated for each field except the last (use `fjall_key()` for
///   full keys). Only generated when the struct has 2+ fields.
///
/// ## Binary layout
///
/// ```text
/// [prefix: u8][field_0 bytes][field_1 bytes]...[field_N bytes]
/// ```
///
/// Fixed-size fields occupy a constant number of bytes (numeric types use
/// big-endian encoding to preserve sort order). The trailing variable-size
/// field, if any, consumes the remaining bytes.
pub trait FjallKeyAble: Sized {
    /// The prefix byte that identifies this key type in the binary layout.
    const PREFIX: u8;

    fn fjall_key(&self) -> crate::UserKey;

    fn from_fjall_key(key: crate::UserKey) -> Result<Self, Cow<'static, str>>;

    fn range<RB: RangeBounds<Self>>(bounds: RB) -> (Bound<crate::UserKey>, Bound<crate::UserKey>) {
        let start = match bounds.start_bound() {
            Bound::Included(v) => Bound::Included(v.fjall_key()),
            Bound::Excluded(v) => Bound::Excluded(v.fjall_key()),
            Bound::Unbounded => Bound::Unbounded,
        };
        let end = match bounds.end_bound() {
            Bound::Included(v) => Bound::Included(v.fjall_key()),
            Bound::Excluded(v) => Bound::Excluded(v.fjall_key()),
            Bound::Unbounded => Bound::Unbounded,
        };
        (start, end)
    }
}

/// FIXME(@svix-gabriel)
///
/// This backdoor is just to accommodate the transition (and eventual removal)
/// of TableKey, replacing it with structs that implement FjallKeyAble.
/// That's gonna involve reworking the TableRow trait, enable a gradual
/// transition, we need this backdoor.
impl<K, T> From<K> for TableKey<T>
where
    K: FjallKeyAble,
    T: TableRow,
{
    fn from(value: K) -> Self {
        TableKey::init_from_bytes(&value.fjall_key())
    }
}

impl<T: TableRow> From<crate::UserKey> for TableKey<T> {
    fn from(key: crate::UserKey) -> Self {
        TableKey::init_from_bytes(&key)
    }
}

/// Trait for types that can be serialized as a component of a fjall key.
///
/// Implementations must use a deterministic encoding. Numeric types use
/// big-endian byte order to preserve sort order.
///
/// Variable-size types (`FIXED_SIZE = false`) must be the last field in a key
/// struct. They consume all remaining bytes on read.
pub trait KeyComponent {
    /// Whether this type always occupies a fixed number of bytes in a key.
    /// Variable-size components (e.g. `String`, `Vec<u8>`) must be the last
    /// field in a key struct.
    const FIXED_SIZE: bool;

    /// The number of bytes a fixed-size component always occupies.
    /// Only meaningful when `FIXED_SIZE` is `true`.
    const BYTE_SIZE: usize;

    /// The borrowed form of this component. For types that own heap data
    /// (`String` → `&str`, `Vec<u8>` → `&[u8]`), this avoids allocation
    /// when extracting a single field from a key. For fixed-size types
    /// this is just `Self`.
    type Ref<'a>;

    /// Returns the number of bytes this component occupies in a key.
    fn key_len(&self) -> usize;

    /// Writes key bytes into `buf`, returning the number of bytes written.
    ///
    /// `buf` must be at least [`key_len`](Self::key_len) bytes long.
    fn write_to_key(&self, buf: &mut [u8]) -> usize;

    /// Parses a value from the beginning of `buf`. Returns the parsed value
    /// and the number of bytes consumed.
    ///
    /// Fixed-size types consume exactly their known byte width. Variable-size
    /// types consume the entire remaining buffer.
    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>>
    where
        Self: Sized;

    /// Like [`read_from_key`](Self::read_from_key) but returns the borrowed
    /// form, avoiding allocation for heap-backed types.
    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>>;
}

impl KeyComponent for u8 {
    const FIXED_SIZE: bool = true;
    const BYTE_SIZE: usize = size_of::<Self>();
    type Ref<'a> = u8;

    fn key_len(&self) -> usize {
        Self::BYTE_SIZE
    }

    fn write_to_key(&self, buf: &mut [u8]) -> usize {
        buf[0] = *self;
        Self::BYTE_SIZE
    }

    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
        buf.first()
            .copied()
            .map(|b| (b, 1))
            .ok_or(Cow::Borrowed("key buffer empty when reading u8"))
    }

    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
        Self::read_from_key(buf)
    }
}

macro_rules! impl_key_component_int {
    ($($ty:ty),*) => {
        $(
            impl KeyComponent for $ty {
                const FIXED_SIZE: bool = true;
                const BYTE_SIZE: usize = std::mem::size_of::<$ty>();
                type Ref<'a> = $ty;

                fn key_len(&self) -> usize {
                    Self::BYTE_SIZE
                }

                fn write_to_key(&self, buf: &mut [u8]) -> usize {
                    let bytes = self.to_be_bytes();
                    buf[..bytes.len()].copy_from_slice(&bytes);
                    bytes.len()
                }

                fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
                    if buf.len() < Self::BYTE_SIZE {
                        return Err(Cow::Owned(format!(
                            "key buffer too short for {}: expected {} bytes, got {}",
                            stringify!($ty),
                            Self::BYTE_SIZE,
                            buf.len()
                        )));
                    }
                    let bytes: [u8; Self::BYTE_SIZE] = buf[..Self::BYTE_SIZE].try_into().expect("checked length");
                    Ok((<$ty>::from_be_bytes(bytes), Self::BYTE_SIZE))
                }

                fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
                    Self::read_from_key(buf)
                }
            }
        )*
    };
}

// IMPORTANT - we can't implement KeyComponent for i8-i128,
// as the order of those types when converted to big endian bytes
// doesn't match the intrinsic order of the types
impl_key_component_int!(u16, u32, u64, u128);

impl KeyComponent for String {
    const FIXED_SIZE: bool = false;
    const BYTE_SIZE: usize = 0;
    type Ref<'a> = &'a str;

    fn key_len(&self) -> usize {
        self.len()
    }

    fn write_to_key(&self, buf: &mut [u8]) -> usize {
        buf[..self.len()].copy_from_slice(self.as_bytes());
        self.len()
    }

    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
        let s = std::str::from_utf8(buf)
            .map_err(|e| Cow::Owned(format!("invalid utf8 in String key component: {e}")))?;
        Ok((s.to_string(), buf.len()))
    }

    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
        let s = std::str::from_utf8(buf)
            .map_err(|e| Cow::Owned(format!("invalid utf8 in String key component: {e}")))?;
        Ok((s, buf.len()))
    }
}

impl<const N: usize> KeyComponent for [u8; N] {
    const FIXED_SIZE: bool = true;
    const BYTE_SIZE: usize = N;
    type Ref<'a> = [u8; N];

    fn key_len(&self) -> usize {
        N
    }

    fn write_to_key(&self, buf: &mut [u8]) -> usize {
        buf[..N].copy_from_slice(self);
        N
    }

    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
        if buf.len() < N {
            return Err(Cow::Owned(format!(
                "key buffer too short for [u8; {N}]: expected {N} bytes, got {}",
                buf.len()
            )));
        }
        let mut arr = [0u8; N];
        arr.copy_from_slice(&buf[..N]);
        Ok((arr, N))
    }

    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
        Self::read_from_key(buf)
    }
}

impl KeyComponent for Vec<u8> {
    const FIXED_SIZE: bool = false;
    const BYTE_SIZE: usize = 0;
    type Ref<'a> = &'a [u8];

    fn key_len(&self) -> usize {
        self.len()
    }

    fn write_to_key(&self, buf: &mut [u8]) -> usize {
        buf[..self.len()].copy_from_slice(self);
        self.len()
    }

    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
        Ok((buf.to_vec(), buf.len()))
    }

    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
        Ok((buf, buf.len()))
    }
}

impl<M: diom_id::IdMarker> KeyComponent for diom_id::Id<M> {
    const FIXED_SIZE: bool = true;
    const BYTE_SIZE: usize = size_of::<Self>();
    type Ref<'a> = diom_id::Id<M>;

    fn key_len(&self) -> usize {
        Self::BYTE_SIZE
    }

    fn write_to_key(&self, buf: &mut [u8]) -> usize {
        buf[..Self::BYTE_SIZE].copy_from_slice(self.as_bytes());
        Self::BYTE_SIZE
    }

    fn read_from_key(buf: &[u8]) -> Result<(Self, usize), Cow<'static, str>> {
        if buf.len() < 16 {
            return Err(Cow::Owned(format!(
                "key buffer too short for Id<M>: expected 16 bytes, got {}",
                buf.len()
            )));
        }
        let id = diom_id::Id::from_slice(&buf[..Self::BYTE_SIZE])
            .map_err(|e| Cow::Owned(format!("invalid uuid in key: {e}")))?;
        Ok((id, 16))
    }

    fn read_ref_from_key(buf: &[u8]) -> Result<(Self::Ref<'_>, usize), Cow<'static, str>> {
        Self::read_from_key(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::FjallKeyAble;

    #[derive(FjallKeyAble)]
    #[table_key(prefix = 1)]
    struct ExampleSingleKey {
        #[key(0)]
        id: u32,
    }

    #[derive(FjallKeyAble)]
    #[table_key(prefix = 2)]
    struct ExampleCompositeKey {
        #[key(0)]
        id: u32,
        #[key(1)]
        group: String,
    }

    #[test]
    fn test_single_key_bytes() {
        let key = ExampleSingleKey { id: 42 };
        let bytes = key.fjall_key();
        let mut expected = Vec::new();
        expected.push(1u8);
        expected.extend_from_slice(&42u32.to_be_bytes());
        assert_eq!(&*bytes, &expected);

        let parsed = ExampleSingleKey::from_fjall_key(bytes).unwrap();
        assert_eq!(parsed.id, 42);
    }

    #[test]
    fn test_composite_key_bytes() {
        let key = ExampleCompositeKey {
            group: "hello".to_string(),
            id: 7,
        };
        let bytes = key.fjall_key();
        // key order: id (key 0, fixed) then group (key 1, variable, last)
        let mut expected = Vec::new();
        expected.push(2u8);
        expected.extend_from_slice(&7u32.to_be_bytes());
        expected.extend_from_slice(b"hello");
        assert_eq!(&*bytes, &expected);

        let parsed = ExampleCompositeKey::from_fjall_key(bytes).unwrap();
        assert_eq!(parsed.group, "hello");
        assert_eq!(parsed.id, 7);
    }

    #[test]
    fn test_range() {
        let (start, end) =
            ExampleSingleKey::range(ExampleSingleKey { id: 1 }..ExampleSingleKey { id: 10 });
        let mut start_expected = Vec::new();
        start_expected.push(1u8);
        start_expected.extend_from_slice(&1u32.to_be_bytes());
        let mut end_expected = Vec::new();
        end_expected.push(1u8);
        end_expected.extend_from_slice(&10u32.to_be_bytes());
        let std::ops::Bound::Included(start_key) = start else {
            panic!()
        };
        let std::ops::Bound::Excluded(end_key) = end else {
            panic!()
        };
        assert_eq!(&*start_key, &start_expected);
        assert_eq!(&*end_key, &end_expected);
    }

    #[test]
    fn test_composite_range() {
        let (start, _end) = ExampleCompositeKey::range(
            ExampleCompositeKey {
                group: "a".to_string(),
                id: 0,
            }..ExampleCompositeKey {
                group: "z".to_string(),
                id: 100,
            },
        );
        let mut start_expected = Vec::new();
        start_expected.push(2u8);
        start_expected.extend_from_slice(&0u32.to_be_bytes());
        start_expected.extend_from_slice(b"a");
        let std::ops::Bound::Included(start_key) = start else {
            panic!()
        };
        assert_eq!(&*start_key, &start_expected);
    }

    #[test]
    fn test_extract_single_field() {
        let key = ExampleSingleKey { id: 42 };
        let bytes = key.fjall_key();
        assert_eq!(ExampleSingleKey::extract_id(&bytes).unwrap(), 42);
    }

    #[test]
    fn test_extract_composite_fields() {
        let key = ExampleCompositeKey {
            id: 7,
            group: "hello".to_string(),
        };
        let bytes = key.fjall_key();
        assert_eq!(ExampleCompositeKey::extract_id(&bytes).unwrap(), 7);
        assert_eq!(ExampleCompositeKey::extract_group(&bytes).unwrap(), "hello");
    }

    #[derive(FjallKeyAble)]
    #[table_key(prefix = 3)]
    struct ExampleTripleKey {
        #[key(0)]
        a: u32,
        #[key(1)]
        b: u16,
        #[key(2)]
        tag: String,
    }

    #[test]
    fn test_prefix_composite_key() {
        let prefix = ExampleCompositeKey::prefix_id(&7u32);
        let mut expected = Vec::new();
        expected.push(2u8);
        expected.extend_from_slice(&7u32.to_be_bytes());
        assert_eq!(prefix, expected);
    }

    #[test]
    fn test_prefix_is_prefix_of_full_key() {
        let key = ExampleCompositeKey {
            id: 7,
            group: "hello".to_string(),
        };
        let full = key.fjall_key();
        let prefix = ExampleCompositeKey::prefix_id(&7u32);
        assert!(full.starts_with(&prefix));
    }

    #[test]
    fn test_prefix_triple_key() {
        let prefix_a = ExampleTripleKey::prefix_a(&10u32);
        let mut expected = Vec::new();
        expected.push(3u8);
        expected.extend_from_slice(&10u32.to_be_bytes());
        assert_eq!(prefix_a, expected);

        let prefix_b = ExampleTripleKey::prefix_b(&10u32, &5u16);
        let mut expected = Vec::new();
        expected.push(3u8);
        expected.extend_from_slice(&10u32.to_be_bytes());
        expected.extend_from_slice(&5u16.to_be_bytes());
        assert_eq!(prefix_b, expected);
    }

    #[test]
    fn test_prefix_nesting() {
        let prefix_a = ExampleTripleKey::prefix_a(&10u32);
        let prefix_b = ExampleTripleKey::prefix_b(&10u32, &5u16);
        let key = ExampleTripleKey {
            a: 10,
            b: 5,
            tag: "x".to_string(),
        };
        let full = key.fjall_key();
        assert!(prefix_b.starts_with(&prefix_a));
        assert!(full.starts_with(&prefix_b));
    }

    #[test]
    fn test_build_key_matches_fjall_key() {
        let key = ExampleCompositeKey {
            id: 42,
            group: "test".to_string(),
        };
        let from_struct = key.fjall_key();
        let from_build = ExampleCompositeKey::build_key(&42u32, &"test".to_string());
        assert_eq!(&*from_struct, &*from_build);
    }

    #[test]
    fn test_build_key_triple() {
        let key = ExampleTripleKey {
            a: 1,
            b: 2,
            tag: "abc".to_string(),
        };
        let from_struct = key.fjall_key();
        let from_build = ExampleTripleKey::build_key(&1u32, &2u16, &"abc".to_string());
        assert_eq!(&*from_struct, &*from_build);
    }
}
