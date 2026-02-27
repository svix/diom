// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsNamespaceArgs {
    #[command(subcommand)]
    pub command: MsgsNamespaceCommands,
}

#[derive(Subcommand)]
pub enum MsgsNamespaceCommands {
    /// Creates or updates a msgs namespace with the given name.
    Create {
        create_namespace_in: crate::json::JsonOf<coyote_client::models::CreateNamespaceIn>,
    },
    /// Gets a msgs namespace by name.
    Get {
        get_namespace_in: crate::json::JsonOf<coyote_client::models::GetNamespaceIn>,
    },
}

impl MsgsNamespaceCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                create_namespace_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .create(create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { get_namespace_in } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .get(get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
