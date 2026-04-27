// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct HealthArgs {
    #[command(subcommand)]
    pub command: HealthCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum HealthCommands {
    /// Verify the server is up and running.
    #[command(help_template = concat!(
                "{about-with-newline}\n",
                "{usage-heading} {usage}\n",
                "{after-help}",
                "\n",
                "{all-args}",
            ))]
    #[command(after_help = "\x1b[1;4mExample response:\x1b[0m
{
  \"ok\": true
}\n")]
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
