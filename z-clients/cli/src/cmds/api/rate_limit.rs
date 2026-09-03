// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

use super::RateLimitNamespaceArgs;
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimitArgs {
    #[command(subcommand)]
    pub command: RateLimitCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum RateLimitCommands {
    Namespace(RateLimitNamespaceArgs),
    /// Rate Limiter Check and Consume
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom rate-limit limit {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"tokens\": 123, // Number of tokens to consume (default: 1)
  \"config\": {\"capacity\": 123, \"refill_amount\": 123, \"refill_interval_ms\": 60000} // Rate limiter configuration
}\n\nExample response:
{
  \"allowed\": true, // Whether the request is allowed
  \"remaining\": 123, // Number of tokens remaining
  \"retry_after_ms\": 60000 // Milliseconds until enough tokens are available (only present when allowed is false)
}\n")]
    Limit {
        rate_limit_check_in: crate::json::JsonOf<diom::models::RateLimitCheckIn>,
    },
    /// Rate Limiter Get Remaining
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom rate-limit get-remaining {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"config\": {\"capacity\": 123, \"refill_amount\": 123, \"refill_interval_ms\": 60000} // Rate limiter configuration
}\n\nExample response:
{
  \"remaining\": 123, // Number of tokens remaining
  \"retry_after_ms\": 60000 // Milliseconds until at least one token is available (only present when remaining is 0)
}\n")]
    GetRemaining {
        rate_limit_get_remaining_in: crate::json::JsonOf<diom::models::RateLimitGetRemainingIn>,
    },
    /// Rate Limiter Reset
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom rate-limit reset {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"config\": {\"capacity\": 123, \"refill_amount\": 123, \"refill_interval_ms\": 60000} // Rate limiter configuration
}\n\nExample response:
{
}\n")]
    Reset {
        rate_limit_reset_in: crate::json::JsonOf<diom::models::RateLimitResetIn>,
    },
}

impl RateLimitCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Limit {
                rate_limit_check_in,
            } => {
                let resp = client
                    .rate_limit()
                    .limit(rate_limit_check_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::GetRemaining {
                rate_limit_get_remaining_in,
            } => {
                let resp = client
                    .rate_limit()
                    .get_remaining(rate_limit_get_remaining_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Reset {
                rate_limit_reset_in,
            } => {
                let resp = client
                    .rate_limit()
                    .reset(rate_limit_reset_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
