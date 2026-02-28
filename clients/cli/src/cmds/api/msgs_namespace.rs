// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

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
        msg_namespace_create_in: crate::json::JsonOf<diom_client::models::MsgNamespaceCreateIn>,
    },
    /// Gets a msgs namespace by name.
    Get {
        msg_namespace_get_in: crate::json::JsonOf<diom_client::models::MsgNamespaceGetIn>,
    },
}

impl MsgsNamespaceCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                msg_namespace_create_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .create(msg_namespace_create_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                msg_namespace_get_in,
            } => {
                let resp = client
                    .msgs()
                    .namespace()
                    .get(msg_namespace_get_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
