use openapi_codegen::api::Api;

use crate::utils::{ContainerizedFormatter, OutputDirectory, generate_outputs};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    OutputDirectory::unmanaged_dir("z-clients/python/coyote", &["python/summary.py.jinja"]),
    OutputDirectory::managed_dir(
        "z-clients/python/coyote/apis",
        &[
            "python/api_summary.py.jinja",
            "python/api_resource.py.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "z-clients/python/coyote/models",
        &[
            "python/component_type_summary.py.jinja",
            "python/component_type.py.jinja",
        ],
    ),
];

pub(crate) async fn generate(api: &Api) -> anyhow::Result<()> {
    generate_outputs(api, OUTPUTS)?;

    ContainerizedFormatter {
        container: "ruff",
        mounts: &[("z-clients/python", "/z-clients/python")],
        cmd: &[
            "sh",
            "-c",
            "ruff check --fix /z-clients/python/coyote && ruff format /z-clients/python/coyote",
        ],
    }
    .run()
    .await?;

    Ok(())
}
