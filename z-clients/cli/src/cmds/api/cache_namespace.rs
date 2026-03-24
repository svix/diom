// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct CacheNamespaceArgs {
    #[command(subcommand)]
    pub command: CacheNamespaceCommands,
}

#[derive(Subcommand)]
pub enum CacheNamespaceCommands {
    /// Create cache namespace
    Create {
        cache_create_namespace_in:
            crate::json::JsonOf<diom_client::models::CacheCreateNamespaceIn>,
    },
    /// Get cache namespace
    Get {
        cache_get_namespace_in: crate::json::JsonOf<diom_client::models::CacheGetNamespaceIn>,
    },
}

impl CacheNamespaceCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                cache_create_namespace_in,
            } => {
                let resp = client
                    .cache()
                    .namespace()
                    .create(cache_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                cache_get_namespace_in,
            } => {
                let resp = client
                    .cache()
                    .namespace()
                    .get(cache_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
