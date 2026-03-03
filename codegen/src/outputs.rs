use crate::utils::OutputDirectory;

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    // CLI
    OutputDirectory::managed_dir(
        "clients/cli/src/cmds/api",
        &["cli/api_summary.rs.jinja", "cli/api_resource.rs.jinja"],
    ),
    // Go
    OutputDirectory::unmanaged_dir(
        "clients/go",
        &[
            "go/api_summary.go.jinja",
            "go/component_type_summary.go.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "clients/go/internal/models",
        &["go/component_type.go.jinja"],
    ),
    OutputDirectory::managed_dir("clients/go/internal/apis", &["go/api_resource.go.jinja"]),
    // Java
    OutputDirectory::unmanaged_dir(
        "clients/java/src/main/java/com/svix/diom",
        &["java/api_summary.java.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/java/src/main/java/com/svix/diom/apis",
        &["java/api_resource.java.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/java/src/main/java/com/svix/diom/models",
        &["java/component_type.java.jinja"],
    ),
    // JavaScript
    OutputDirectory::unmanaged_dir(
        "clients/javascript/src",
        &["javascript/api_summary.ts.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/javascript/src/apis",
        &["javascript/api_resource.ts.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/javascript/src/models",
        &["javascript/component_type.ts.jinja"],
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
    // Python
    OutputDirectory::unmanaged_dir("clients/python/diom", &["python/summary.py.jinja"]),
    OutputDirectory::managed_dir(
        "clients/python/diom/apis",
        &[
            "python/api_summary.py.jinja",
            "python/api_resource.py.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "clients/python/diom/models",
        &[
            "python/component_type_summary.py.jinja",
            "python/component_type.py.jinja",
        ],
    ),
];
