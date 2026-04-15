use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    num::{NonZeroU16, NonZeroU32, NonZeroU64},
};

/// Marker trait for whether something can be
/// safely represented in fjall.
///
/// # Safety
///
/// Implementers must guarantee the following:
///
/// 1. The serde implementation for this object doesn't do
///    `#[serde(skip_serializing_if)]` or `#[serde(flatten)]`
/// 2. The serialization of this object is stable across version
///
/// Generally, you should prefer to use the `PersistableValue` derive to implement this safely.
pub trait PersistableValue {}

impl PersistableValue for bool {}
impl PersistableValue for u8 {}
impl PersistableValue for u16 {}
impl PersistableValue for u32 {}
impl PersistableValue for u64 {}
impl PersistableValue for u128 {}
impl PersistableValue for i8 {}
impl PersistableValue for i16 {}
impl PersistableValue for i32 {}
impl PersistableValue for i64 {}
impl PersistableValue for i128 {}
// TODO: use std::num::NonZero<T> when stabilized
impl PersistableValue for NonZeroU16 {}
impl PersistableValue for NonZeroU32 {}
impl PersistableValue for NonZeroU64 {}
impl PersistableValue for String {}
impl PersistableValue for &str {}
impl PersistableValue for &[u8] {}
impl PersistableValue for uuid::Uuid {}
impl PersistableValue for crate::types::ByteString {}
impl PersistableValue for crate::types::DurationMs {}
impl PersistableValue for crate::types::UnixTimestampMs {}
impl PersistableValue for crate::types::Metadata {}

impl<T: PersistableValue> PersistableValue for Option<T> {}
impl<T: PersistableValue> PersistableValue for Vec<T> {}
impl<T: PersistableValue> PersistableValue for HashSet<T> {}
impl<T: PersistableValue> PersistableValue for BTreeSet<T> {}
impl<K: PersistableValue, T: PersistableValue> PersistableValue for HashMap<K, T> {}
impl<K: PersistableValue, T: PersistableValue> PersistableValue for BTreeMap<K, T> {}

impl<const N: usize, T: PersistableValue> PersistableValue for [T; N] {}

impl PersistableValue for () {}

macro_rules! impl_persistable_value_for_tuple {
    ( $( $N:literal )+ ) => {
        pastey::paste! {
            impl<$([< T $N >],)+> PersistableValue for ($([< T $N >],)+)
                where $([< T $N >]: PersistableValue,)+ {}
        }
    };
}

impl_persistable_value_for_tuple! { 0 }
impl_persistable_value_for_tuple! { 0 1 }
impl_persistable_value_for_tuple! { 0 1 2 }
impl_persistable_value_for_tuple! { 0 1 2 3 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 7 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 7 8 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 7 8 9 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 7 8 9 10 }
impl_persistable_value_for_tuple! { 0 1 2 3 4 5 6 7 8 9 10 11 }

/// Helper trait for making sure that composite structures (structs and enums) implement
/// PersistableValue for all of their fields.
///
/// # Safety
///
/// This should only be implemented via the PersistableValue derive
pub trait PersistableStruct {
    type INNER: PersistableValue;
}

impl<F: PersistableStruct> PersistableValue for F {}
