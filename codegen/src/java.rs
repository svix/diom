use crate::utils::OutputDirectory;

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
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
];
