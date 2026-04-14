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

#[derive(Subcommand)]
pub enum RateLimitCommands {
    Namespace(RateLimitNamespaceArgs),
    /// Rate Limiter Check and Consume
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"tokens\": \"...\",
  \"config\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"allowed\": \"...\",
  \"remaining\": \"...\",
  \"retry_after_ms\": \"...\"
}")]
    Limit {
        rate_limit_check_in: crate::json::JsonOf<diom::models::RateLimitCheckIn>,
    },
    /// Rate Limiter Get Remaining
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"config\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"remaining\": \"...\",
  \"retry_after_ms\": \"...\"
}")]
    GetRemaining {
        rate_limit_get_remaining_in: crate::json::JsonOf<diom::models::RateLimitGetRemainingIn>,
    },
    /// Rate Limiter Reset
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"some_namespace\",
  \"key\": \"some_key\",
  \"config\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
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
