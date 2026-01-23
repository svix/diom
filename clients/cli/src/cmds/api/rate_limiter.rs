// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args, Clone)]
pub struct RateLimiterLimitOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<RateLimiterLimitOptions> for coyote_client::api::RateLimiterLimitOptions {
    fn from(value: RateLimiterLimitOptions) -> Self {
        let RateLimiterLimitOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct RateLimiterGetRemainingOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<RateLimiterGetRemainingOptions> for coyote_client::api::RateLimiterGetRemainingOptions {
    fn from(value: RateLimiterGetRemainingOptions) -> Self {
        let RateLimiterGetRemainingOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

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
        rate_limiter_check_in: crate::json::JsonOf<coyote_client::models::RateLimiterCheckIn>,
        #[clap(flatten)]
        options: RateLimiterLimitOptions,
    },
    /// Rate Limiter Get Remaining
    GetRemaining {
        rate_limiter_get_remaining_in:
            crate::json::JsonOf<coyote_client::models::RateLimiterGetRemainingIn>,
        #[clap(flatten)]
        options: RateLimiterGetRemainingOptions,
    },
}

impl RateLimiterCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Limit {
                rate_limiter_check_in,
                options,
            } => {
                let resp = client
                    .rate_limiter()
                    .limit(rate_limiter_check_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetRemaining {
                rate_limiter_get_remaining_in,
                options,
            } => {
                let resp = client
                    .rate_limiter()
                    .get_remaining(
                        rate_limiter_get_remaining_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
