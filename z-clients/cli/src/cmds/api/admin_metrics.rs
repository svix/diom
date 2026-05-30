// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminMetricsArgs {
    #[command(subcommand)]
    pub command: AdminMetricsCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum AdminMetricsCommands {
    /// Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin metrics get\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"metrics\": [{\"label\": \"...\", \"description\": \"...\", \"attributes\": {\"key\": \"...\"}, \"value\": \"...\", \"metric_type\": \"counter\", \"timestamp\": 1234567890123, \"unit\": \"...\"}]
}\n")]
    Get {},
}

impl AdminMetricsCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Get {} => {
                let resp = client.admin().metrics().get().await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
