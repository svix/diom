// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsNamespaceArgs {
    #[command(subcommand)]
    pub command: MsgsNamespaceCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum MsgsNamespaceCommands {
    /// Configures a msgs namespace with the given name.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs namespace configure NAME {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"retention\": {\"period_ms\": 60000}
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"retention\": {\"period_ms\": 60000},
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Configure {
        name: String,
        msg_namespace_configure_in:
            Option<crate::json::JsonOf<diom::models::MsgNamespaceConfigureIn>>,
    },
    /// Gets a msgs namespace by name.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom msgs namespace get NAME {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"retention\": {\"period_ms\": 60000},
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Get {
        name: String,
        msg_namespace_get_in: Option<crate::json::JsonOf<diom::models::MsgNamespaceGetIn>>,
    },
}

impl MsgsNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                name,
                msg_namespace_configure_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .configure(
                        name,
                        msg_namespace_configure_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                name,
                msg_namespace_get_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .get(name, msg_namespace_get_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
