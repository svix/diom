use std::io;

use crate::utils::{ContainerizedFormatter, OutputDirectory};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
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

pub(crate) async fn format_go_client() -> io::Result<()> {
    ContainerizedFormatter {
        container: "goimports",
        mounts: &[("clients/go", "/go/coyote")],
        cmd: &["goimports", "-w", "/go/coyote"],
    }
    .run()
    .await
}
