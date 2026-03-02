// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

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
        key: String,
        value: String,
        /// Time to live in milliseconds
        ttl: u64,
        cache_set_in: crate::json::JsonOf<coyote_client::models::CacheSetIn>,
    },
    /// Cache Get
    Get {
        key: String,
        cache_get_in: crate::json::JsonOf<coyote_client::models::CacheGetIn>,
    },
    /// Get cache namespace
    GetNamespace {
        cache_get_namespace_in: crate::json::JsonOf<coyote_client::models::CacheGetNamespaceIn>,
    },
    /// Cache Delete
    Delete {
        key: String,
        cache_delete_in: crate::json::JsonOf<coyote_client::models::CacheDeleteIn>,
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
                key,
                value,
                ttl,
                cache_set_in,
            } => {
                let resp = client
                    .cache()
                    .set(key, value.into(), ttl, cache_set_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { key, cache_get_in } => {
                let resp = client.cache().get(key, cache_get_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetNamespace {
                cache_get_namespace_in,
            } => {
                let resp = client
                    .cache()
                    .get_namespace(cache_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                key,
                cache_delete_in,
            } => {
                let resp = client
                    .cache()
                    .delete(key, cache_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
