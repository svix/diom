// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct RateLimiterArgs {
    #[command(subcommand)]
    pub command: RateLimiterCommands,
}

#[derive(Subcommand)]
pub enum RateLimiterCommands {
    /// Rate Limiter Check and Consume
    Limit {
        rate_limiter_check_in: crate::json::JsonOf<diom_client::models::RateLimiterCheckIn>,
    },
    /// Rate Limiter Get Remaining
    GetRemaining {
        rate_limiter_get_remaining_in:
            crate::json::JsonOf<diom_client::models::RateLimiterGetRemainingIn>,
    },
}

impl RateLimiterCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Limit {
                rate_limiter_check_in,
            } => {
                let resp = client
                    .rate_limiter()
                    .limit(rate_limiter_check_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetRemaining {
                rate_limiter_get_remaining_in,
            } => {
                let resp = client
                    .rate_limiter()
                    .get_remaining(rate_limiter_get_remaining_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
