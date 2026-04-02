// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

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
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  key
  config — Rate limiter configuration
  namespace (optional)
  tokens (optional) — Number of tokens to consume (default: 1)")]
    Limit {
        rate_limit_check_in: crate::json::JsonOf<coyote_client::models::RateLimitCheckIn>,
    },
    /// Rate Limiter Get Remaining
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  key
  config — Rate limiter configuration
  namespace (optional)")]
    GetRemaining {
        rate_limit_get_remaining_in:
            crate::json::JsonOf<coyote_client::models::RateLimitGetRemainingIn>,
    },
    /// Rate Limiter Reset
    #[command(after_long_help = "\x1b[1;4mJSON body fields:\x1b[0m
  key
  config — Rate limiter configuration
  namespace (optional)")]
    Reset {
        rate_limit_reset_in: crate::json::JsonOf<coyote_client::models::RateLimitResetIn>,
    },
}

impl RateLimitCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
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
