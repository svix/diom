mod table_row;

pub use table_row::{TableRow, WriteBatchExt};

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
}
