// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AuthTokenNamespaceArgs {
    #[command(subcommand)]
    pub command: AuthTokenNamespaceCommands,
}

#[derive(Subcommand)]
pub enum AuthTokenNamespaceCommands {
    /// Create Auth Token namespace
    Create {
        auth_token_create_namespace_in:
            crate::json::JsonOf<diom_client::models::AuthTokenCreateNamespaceIn>,
    },
    /// Get Auth Token namespace
    Get {
        auth_token_get_namespace_in:
            crate::json::JsonOf<diom_client::models::AuthTokenGetNamespaceIn>,
    },
}

impl AuthTokenNamespaceCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                auth_token_create_namespace_in,
            } => {
                let resp = client
                    .auth_token()
                    .namespace()
                    .create(auth_token_create_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                auth_token_get_namespace_in,
            } => {
                let resp = client
                    .auth_token()
                    .namespace()
                    .get(auth_token_get_namespace_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
