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
    OutputDirectory::managed_dir(
        "clients/java/src/main/java/com/svix/coyote/apis",
        &["java/api_resource.java.jinja"],
    ),
    OutputDirectory::managed_dir(
        "clients/java/src/main/java/com/svix/coyote/models",
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
    OutputDirectory::unmanaged_dir("clients/python/coyote", &["python/summary.py.jinja"]),
    OutputDirectory::managed_dir(
        "clients/python/coyote/apis",
        &[
            "python/api_summary.py.jinja",
            "python/api_resource.py.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "clients/python/coyote/models",
        &[
            "python/component_type_summary.py.jinja",
            "python/component_type.py.jinja",
        ],
    ),
];

pub(crate) struct OutputDirectory {
    pub path: &'static str,
    pub templates: &'static [&'static str],
    /// Whether the directory is managed by codegen.
    ///
    /// If this is `true`, the directory will be emptied before generating code.
    pub managed: bool,
}

impl OutputDirectory {
    const fn managed_dir(path: &'static str, templates: &'static [&'static str]) -> Self {
        Self {
            path,
            templates,
            managed: true,
        }
    }

    const fn unmanaged_dir(path: &'static str, templates: &'static [&'static str]) -> Self {
        Self {
            path,
            templates,
            managed: false,
        }
    }
}
