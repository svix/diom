/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    // Rust SDK
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
