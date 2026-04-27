// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimitNamespaceArgs {
    #[command(subcommand)]
    pub command: RateLimitNamespaceCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum RateLimitNamespaceCommands {
    /// Configure rate limiter namespace
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom rate-limit namespace configure {...}\n",
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
        rate_limit_configure_namespace_in:
            crate::json::JsonOf<diom::models::RateLimitConfigureNamespaceIn>,
    },
    /// Get rate limiter namespace
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom rate-limit namespace get {...}\n",
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
        rate_limit_get_namespace_in: crate::json::JsonOf<diom::models::RateLimitGetNamespaceIn>,
    },
}

impl RateLimitNamespaceCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Configure {
                rate_limit_configure_namespace_in,
            } => {
                let resp = client
                    .rate_limit()
                    .namespace()
                    .configure(rate_limit_configure_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get {
                rate_limit_get_namespace_in,
            } => {
                let resp = client
                    .rate_limit()
                    .namespace()
                    .get(rate_limit_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
