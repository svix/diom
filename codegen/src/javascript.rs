use crate::utils::OutputDirectory;

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
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
];
