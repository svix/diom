// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::CacheNamespaceArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[derive(Subcommand)]
pub enum CacheCommands {
    Namespace(CacheNamespaceArgs),
    /// Cache Set
    Set {
        key: String,
        cache_set_in: crate::json::JsonOf<diom_client::models::CacheSetIn>,
    },
    /// Cache Get
    Get {
        key: String,
        cache_get_in: crate::json::JsonOf<diom_client::models::CacheGetIn>,
    },
    /// Cache Delete
    Delete {
        cache_delete_in: crate::json::JsonOf<diom_client::models::CacheDeleteIn>,
    },
}

impl CacheCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Set { key, cache_set_in } => {
                let resp = client.cache().set(key, cache_set_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { key, cache_get_in } => {
                let resp = client.cache().get(key, cache_get_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { cache_delete_in } => {
                let resp = client.cache().delete(cache_delete_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
