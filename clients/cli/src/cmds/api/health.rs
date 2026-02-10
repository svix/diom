// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct HealthArgs {
    #[command(subcommand)]
    pub command: HealthCommands,
}

#[derive(Subcommand)]
pub enum HealthCommands {
    /// Verify the server is up and running.
    Ping {},
}

impl HealthCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Ping {} => {
                let resp = client.health().ping().await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
