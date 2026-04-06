// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthTokenArgs {
    #[command(subcommand)]
    pub command: AdminAuthTokenCommands,
}

#[derive(Subcommand)]
pub enum AdminAuthTokenCommands {
    /// Create an auth token
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"...\",
  \"role\": \"...\",
  \"expiry_ms\": \"...\",
  \"enabled\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"token\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Create {
        admin_auth_token_create_in:
            crate::json::JsonOf<diom_client::models::AdminAuthTokenCreateIn>,
    },
    /// Expire an auth token
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"expiry_ms\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Expire {
        admin_auth_token_expire_in:
            crate::json::JsonOf<diom_client::models::AdminAuthTokenExpireIn>,
    },
    /// Rotate an auth token, invalidating the old one and issuing a new secret
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"token\": \"...\",
  \"created\": \"...\",
  \"updated\": \"...\"
}")]
    Rotate {
        admin_auth_token_rotate_in:
            crate::json::JsonOf<diom_client::models::AdminAuthTokenRotateIn>,
    },
    /// Delete an auth token
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": \"...\"
}")]
    Delete {
        admin_auth_token_delete_in:
            crate::json::JsonOf<diom_client::models::AdminAuthTokenDeleteIn>,
    },
    /// List auth tokens for a given owner
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"limit\": \"...\",
  \"iterator\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"data\": \"...\",
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": \"...\"
}")]
    List {
        admin_auth_token_list_in:
            Option<crate::json::JsonOf<diom_client::models::AdminAuthTokenListIn>>,
    },
    /// Update an auth token's properties
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"name\": \"...\",
  \"expiry_ms\": \"...\",
  \"enabled\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Update {
        admin_auth_token_update_in:
            crate::json::JsonOf<diom_client::models::AdminAuthTokenUpdateIn>,
    },
    /// Return the role of the currently authenticated token
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"role\": \"...\"
}")]
    Whoami {
        admin_auth_token_whoami_in:
            Option<crate::json::JsonOf<diom_client::models::AdminAuthTokenWhoamiIn>>,
    },
}

impl AdminAuthTokenCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
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
