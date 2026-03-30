// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct TransformationsArgs {
    #[command(subcommand)]
    pub command: TransformationsCommands,
}

#[derive(Subcommand)]
pub enum TransformationsCommands {
    /// Execute a JavaScript transformation script against a payload and return the result.
    Execute {
        transform_in: crate::json::JsonOf<coyote_client::models::TransformIn>,
    },
}

impl TransformationsCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Execute { transform_in } => {
                let resp = client
                    .transformations()
                    .execute(transform_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
