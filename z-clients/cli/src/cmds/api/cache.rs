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
        value: Vec<u8>,
        cache_set_in: crate::json::JsonOf<diom_client::models::CacheSetIn>,
    },
    /// Cache Get
    Get {
        key: String,
        cache_get_in: Option<crate::json::JsonOf<diom_client::models::CacheGetIn>>,
    },
    /// Cache Delete
    Delete {
        key: String,
        cache_delete_in: Option<crate::json::JsonOf<diom_client::models::CacheDeleteIn>>,
    },
}

impl CacheCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client).await?;
            }
            Self::Set {
                key,
                value,
                cache_set_in,
            } => {
                let resp = client
                    .cache()
                    .set(key, value, cache_set_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Get { key, cache_get_in } => {
                let resp = client
                    .cache()
                    .get(key, cache_get_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                key,
                cache_delete_in,
            } => {
                let resp = client
                    .cache()
                    .delete(key, cache_delete_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
