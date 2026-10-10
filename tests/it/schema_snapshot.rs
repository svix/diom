//! Guards the shape of everything we persist or replicate.
//!
//! Every `PersistableValue` and `PersistableVersioned` type registers its shape (see
//! `diom_core::schema_shape`). This test renders all of them into a snapshot and compares it to the
//! committed `tests/it/static/schema-snapshot.txt`. A diff means a stored struct or operation
//! changed. Confirm the change keeps old data readable (append only fields, `#[since]`, `#[nested]`
//! for evolvable nested structs), then regenerate the snapshot with
//! `UPDATE_SCHEMA_SNAPSHOT=1 cargo test --test it schema_snapshot`.

use std::path::Path;

fn snapshot_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it/static/schema-snapshot.txt")
}

#[test]
fn schema_snapshot_is_up_to_date() {
    let rendered = diom_core::schema_shape::render_snapshot();
    let path = snapshot_path();

    if std::env::var_os("UPDATE_SCHEMA_SNAPSHOT").is_some() {
        std::fs::write(&path, &rendered).expect("write schema snapshot");
        return;
    }

    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        rendered, committed,
        "\n\nThe schema snapshot changed, which means a persisted or replicated type changed shape. \
         Make sure the change keeps old data readable (append only fields, use #[since(n)], and \
         #[nested] for nested structs that evolve), then regenerate it with:\n    \
         UPDATE_SCHEMA_SNAPSHOT=1 cargo test --test it schema_snapshot\n"
    );
}
