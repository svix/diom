use openapi_codegen::api::Api;

use crate::utils::{ContainerizedFormatter, OutputDirectory, generate_outputs};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    OutputDirectory::unmanaged_dir(
        "z-clients/go",
        &[
            "go/api_summary.go.jinja",
            "go/component_type_summary.go.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "z-clients/go/internal/models",
        &["go/component_type.go.jinja"],
    ),
    OutputDirectory::managed_dir("z-clients/go/internal/apis", &["go/api_resource.go.jinja"]),
];

pub(crate) async fn generate(api: &Api) -> anyhow::Result<()> {
    generate_outputs(api, OUTPUTS)?;

    ContainerizedFormatter {
        container: "goimports",
        mounts: &[("z-clients/go", "/go/coyote")],
        cmd: &["goimports", "-w", "/go/coyote"],
    }
    .run()
    .await?;

    Ok(())
}
