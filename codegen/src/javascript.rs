use openapi_codegen::api::Api;

use crate::utils::{OutputDirectory, generate_outputs};

/// Map of output directory => list of templates that should write there.
pub(crate) const OUTPUTS: &[OutputDirectory] = &[
    OutputDirectory::unmanaged_dir(
        "z-clients/javascript/src",
        &[
            "javascript/api_summary.ts.jinja",
            "javascript/summary.ts.jinja",
        ],
    ),
    OutputDirectory::managed_dir(
        "z-clients/javascript/src/apis",
        &["javascript/api_resource.ts.jinja"],
    ),
    OutputDirectory::managed_dir(
        "z-clients/javascript/src/models",
        &[
            "javascript/component_type_summary.ts.jinja",
            "javascript/component_type.ts.jinja",
        ],
    ),
];

pub(crate) async fn generate(api: &Api) -> anyhow::Result<()> {
    generate_outputs(api, OUTPUTS)?;
    Ok(())
}
