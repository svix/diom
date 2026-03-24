// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvNamespaceArgs {
    #[command(subcommand)]
    pub command: KvNamespaceCommands,
}

#[derive(Subcommand)]
pub enum KvNamespaceCommands {
    /// Create KV namespace
    Create {
        kv_create_namespace_in: crate::json::JsonOf<coyote_client::models::KvCreateNamespaceIn>,
    },
    /// Get KV namespace
    Get {
        kv_get_namespace_in: crate::json::JsonOf<coyote_client::models::KvGetNamespaceIn>,
    },
}

impl KvNamespaceCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                kv_create_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .namespace()
                    .create(kv_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                kv_get_namespace_in,
            } => {
                let resp = client
                    .kv()
                    .namespace()
                    .get(kv_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
