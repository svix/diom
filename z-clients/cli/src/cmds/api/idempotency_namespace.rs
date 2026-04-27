// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IdempotencyNamespaceArgs {
    #[command(subcommand)]
    pub command: IdempotencyNamespaceCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum IdempotencyNamespaceCommands {
    /// Configure idempotency namespace
    #[command(help_template = concat!(
                "{about-with-newline}\n",
                "{usage-heading} {usage}\n",
                "{after-help}",
                "\n",
                "{all-args}",
            ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Configure {
        idempotency_configure_namespace_in:
            crate::json::JsonOf<diom::models::IdempotencyConfigureNamespaceIn>,
    },
    /// Get idempotency namespace
    #[command(help_template = concat!(
                "{about-with-newline}\n",
                "{usage-heading} {usage}\n",
                "{after-help}",
                "\n",
                "{all-args}",
            ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"some_namespace\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"name\": \"some_namespace\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Get {
        idempotency_get_namespace_in: crate::json::JsonOf<diom::models::IdempotencyGetNamespaceIn>,
    },
}

impl IdempotencyNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                idempotency_configure_namespace_in,
            } => {
                let resp = client
                    .idempotency()
                    .namespace()
                    .configure(idempotency_configure_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                idempotency_get_namespace_in,
            } => {
                let resp = client
                    .idempotency()
                    .namespace()
                    .get(idempotency_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
