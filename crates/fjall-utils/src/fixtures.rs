//! Test helper for reading a stored row from bytes for an earlier version of a row.
//!
//! A byte fixture freezes the on-disk encoding of a row as it existed at some past version, then
//! decodes it with the current code. This is the test that tells you whether a change is safe. If a
//! new field is added as a plain field, the frozen bytes stop decoding and the test fails loudly (old
//! data can no longer be read). If it is added as `#[since(n)]` on a `PersistableVersioned` row, the
//! frozen bytes still decode with the new field defaulted and the test stays green.

use crate::TableRow;

/// Decodes a stored row from committed hex bytes written by an earlier version.
///
/// Panics if the bytes no longer decode, i.e. a backwards INcompatible change was made.
pub fn decode_row<T: TableRow>(frozen_hex: &str) -> T {
    let bytes = hex::decode(frozen_hex).expect("fixture hex is valid");
    T::from_fjall_value(bytes.into()).unwrap_or_else(|e| {
        panic!(
            "`{}` can no longer decode bytes written by an earlier version ({e:?}). A newly added \
             field must be `#[since(n)]` on a `PersistableVersioned` row so existing data stays readable.",
            std::any::type_name::<T>(),
        )
    })
}

/// Serializes a sample row to its hex encoding. Use it to mint the frozen hex for a new fixture, then
/// paste the result into a `decode_row` test.
pub fn row_hex<T: TableRow>(sample: &T) -> String {
    hex::encode(sample.to_fjall_value().expect("row serializes"))
}
