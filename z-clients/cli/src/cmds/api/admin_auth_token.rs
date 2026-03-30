// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthTokenArgs {
    #[command(subcommand)]
    pub command: AdminAuthTokenCommands,
}

#[derive(Subcommand)]
pub enum AdminAuthTokenCommands {
    /// Create an auth token
    Create {
        admin_auth_token_create_in:
            crate::json::JsonOf<coyote_client::models::AdminAuthTokenCreateIn>,
    },
    /// Expire an auth token
    Expire {
        admin_auth_token_expire_in:
            crate::json::JsonOf<coyote_client::models::AdminAuthTokenExpireIn>,
    },
    /// Rotate an auth token, invalidating the old one and issuing a new secret
    Rotate {
        admin_auth_token_rotate_in:
            crate::json::JsonOf<coyote_client::models::AdminAuthTokenRotateIn>,
    },
    /// Delete an auth token
    Delete {
        admin_auth_token_delete_in:
            crate::json::JsonOf<coyote_client::models::AdminAuthTokenDeleteIn>,
    },
    /// List auth tokens for a given owner
    List {
        admin_auth_token_list_in:
            Option<crate::json::JsonOf<coyote_client::models::AdminAuthTokenListIn>>,
    },
    /// Update an auth token's properties
    Update {
        admin_auth_token_update_in:
            crate::json::JsonOf<coyote_client::models::AdminAuthTokenUpdateIn>,
    },
    /// Return the role of the currently authenticated token
    Whoami {
        admin_auth_token_whoami_in:
            Option<crate::json::JsonOf<coyote_client::models::AdminAuthTokenWhoamiIn>>,
    },
}

impl AdminAuthTokenCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::Create {
                admin_auth_token_create_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .create(admin_auth_token_create_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Expire {
                admin_auth_token_expire_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .expire(admin_auth_token_expire_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Rotate {
                admin_auth_token_rotate_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .rotate(admin_auth_token_rotate_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Delete {
                admin_auth_token_delete_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .delete(admin_auth_token_delete_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::List {
                admin_auth_token_list_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .list(admin_auth_token_list_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Update {
                admin_auth_token_update_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .update(admin_auth_token_update_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Whoami {
                admin_auth_token_whoami_in,
            } => {
                let resp = client
                    .admin()
                    .auth_token()
                    .whoami(admin_auth_token_whoami_in.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
