// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct HealthArgs {
    #[command(subcommand)]
    pub command: HealthCommands,
}

#[derive(Subcommand)]
pub enum HealthCommands {
    /// Verify the server is up and running.
    #[command(after_long_help = "\x1b[1;4mExample response:\x1b[0m
{
  \"ok\": \"...\"
}")]
    Ping {},
    /// Intentionally return an error
    Error {},
}

impl HealthCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Ping {} => {
                let resp = client.health().ping().await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Error {} => {
                client.health().error().await?;
            }
        }

        Ok(())
    }
}
