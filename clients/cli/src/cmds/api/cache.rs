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
        cache_set_in: crate::json::JsonOf<coyote_client::models::CacheSetIn>,
    },
    /// Cache Get
    Get {
        cache_get_in: crate::json::JsonOf<coyote_client::models::CacheGetIn>,
    },
    /// Get cache group
    GetGroup {
        cache_get_group_in: crate::json::JsonOf<coyote_client::models::CacheGetGroupIn>,
    },
    /// Cache Delete
    Delete {
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
            Self::Set { cache_set_in } => {
                let resp = client.cache().set(cache_set_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { cache_get_in } => {
                let resp = client.cache().get(cache_get_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetGroup { cache_get_group_in } => {
                let resp = client
                    .cache()
                    .get_group(cache_get_group_in.into_inner())
                    .await?;
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
