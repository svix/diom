// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::AuthTokenNamespaceArgs;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AuthTokenArgs {
    #[command(subcommand)]
    pub command: AuthTokenCommands,
}

#[derive(Subcommand)]
pub enum AuthTokenCommands {
    Namespace(AuthTokenNamespaceArgs),
    /// Create Auth Token
    Create {
        auth_token_create_in: crate::json::JsonOf<diom_client::models::AuthTokenCreateIn>,
    },
    /// Expire Auth Token
    Expire {
        auth_token_expire_in: crate::json::JsonOf<diom_client::models::AuthTokenExpireIn>,
    },
    /// Delete Auth Token
    Delete {
        auth_token_delete_in: crate::json::JsonOf<diom_client::models::AuthTokenDeleteIn>,
    },
    /// Verify Auth Token
    Verify {
        auth_token_verify_in: crate::json::JsonOf<diom_client::models::AuthTokenVerifyIn>,
    },
    /// List Auth Tokens
    List {
        auth_token_list_in: crate::json::JsonOf<diom_client::models::AuthTokenListIn>,
    },
    /// Update Auth Token
    Update {
        auth_token_update_in: crate::json::JsonOf<diom_client::models::AuthTokenUpdateIn>,
    },
    /// Rotate Auth Token
    Rotate {
        auth_token_rotate_in: crate::json::JsonOf<diom_client::models::AuthTokenRotateIn>,
    },
}

impl AuthTokenCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Namespace(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Create {
                auth_token_create_in,
            } => {
                let resp = client
                    .auth_token()
                    .create(auth_token_create_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Expire {
                auth_token_expire_in,
            } => {
                let resp = client
                    .auth_token()
                    .expire(auth_token_expire_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                auth_token_delete_in,
            } => {
                let resp = client
                    .auth_token()
                    .delete(auth_token_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Verify {
                auth_token_verify_in,
            } => {
                let resp = client
                    .auth_token()
                    .verify(auth_token_verify_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::List { auth_token_list_in } => {
                let resp = client
                    .auth_token()
                    .list(auth_token_list_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                auth_token_update_in,
            } => {
                let resp = client
                    .auth_token()
                    .update(auth_token_update_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Rotate {
                auth_token_rotate_in,
            } => {
                let resp = client
                    .auth_token()
                    .rotate(auth_token_rotate_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
