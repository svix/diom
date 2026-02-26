// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvArgs {
    #[command(subcommand)]
    pub command: KvCommands,
}

#[derive(Subcommand)]
pub enum KvCommands {
    /// KV Set
    Set {
        kv_set_in: crate::json::JsonOf<coyote_client::models::KvSetIn>,
    },
    /// KV Get
    Get {
        kv_get_in: crate::json::JsonOf<coyote_client::models::KvGetIn>,
    },
    /// Get KV namespace
    GetNamespace {
        kv_get_namespace_in: crate::json::JsonOf<coyote_client::models::KvGetNamespaceIn>,
    },
    /// KV Delete
    Delete {
        kv_delete_in: crate::json::JsonOf<coyote_client::models::KvDeleteIn>,
    },
}

impl KvCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Set { kv_set_in } => {
                let resp = client.kv().set(kv_set_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { kv_get_in } => {
                let resp = client.kv().get(kv_get_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetNamespace {
                kv_get_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .get_namespace(kv_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { kv_delete_in } => {
                let resp = client.kv().delete(kv_delete_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
