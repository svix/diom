// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args, Clone)]
pub struct CacheSetOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<CacheSetOptions> for coyote_client::api::CacheSetOptions {
    fn from(value: CacheSetOptions) -> Self {
        let CacheSetOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct CacheGetOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<CacheGetOptions> for coyote_client::api::CacheGetOptions {
    fn from(value: CacheGetOptions) -> Self {
        let CacheGetOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct CacheDeleteOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<CacheDeleteOptions> for coyote_client::api::CacheDeleteOptions {
    fn from(value: CacheDeleteOptions) -> Self {
        let CacheDeleteOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Cache Set
    Set {
        cache_set_in: crate::json::JsonOf<coyote_client::models::CacheSetIn>,
        #[clap(flatten)]
        options: CacheSetOptions,
    },
    /// Cache Get
    Get {
        cache_get_in: crate::json::JsonOf<coyote_client::models::CacheGetIn>,
        #[clap(flatten)]
        options: CacheGetOptions,
    },
    /// Cache Delete
    Delete {
        cache_delete_in: crate::json::JsonOf<coyote_client::models::CacheDeleteIn>,
        #[clap(flatten)]
        options: CacheDeleteOptions,
    },
}

impl CacheCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Set {
                cache_set_in,
                options,
            } => {
                let resp = client
                    .cache()
                    .set(cache_set_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                cache_get_in,
                options,
            } => {
                let resp = client
                    .cache()
                    .get(cache_get_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                cache_delete_in,
                options,
            } => {
                let resp = client
                    .cache()
                    .delete(cache_delete_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
