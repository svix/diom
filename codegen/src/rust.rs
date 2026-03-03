use std::io;

use crate::utils::{OutputDirectory, exec};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    // CLI
    OutputDirectory::managed_dir(
        "clients/cli/src/cmds/api",
        &["cli/api_summary.rs.jinja", "cli/api_resource.rs.jinja"],
    ),
    // Rust
    OutputDirectory::managed_dir(
        "clients/rust/src/api",
        &["rust/api_summary.rs.jinja", "rust/api_resource.rs.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/rust/src/models",
        &[
            "rust/component_type_summary.rs.jinja",
            "rust/component_type.rs.jinja",
        ],
    ),
];

pub(crate) async fn format_rust_clients() -> io::Result<()> {
    exec(
        "cargo",
        [
            "+nightly",
            "fmt",
            "--package=diom-client",
            "--package=diom-cli",
        ],
    )
    .await?;
    Ok(())
}
