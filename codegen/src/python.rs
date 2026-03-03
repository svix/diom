use std::io;

use crate::utils::{ContainerizedFormatter, OutputDirectory};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
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

pub(crate) async fn format_python_client() -> io::Result<()> {
    ContainerizedFormatter {
        container: "ruff",
        mounts: &[("clients/python", "/clients/python")],
        cmd: &[
            "sh",
            "-c",
            "ruff check --fix /clients/python/diom && ruff format /clients/python/diom",
        ],
    }
    .run()
    .await
}
