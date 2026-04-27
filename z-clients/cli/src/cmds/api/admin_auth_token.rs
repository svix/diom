// this file is @generated
use clap::{Args, Subcommand};
use diom::DiomClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminAuthTokenArgs {
    #[command(subcommand)]
    pub command: AdminAuthTokenCommands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
pub enum AdminAuthTokenCommands {
    /// Create an auth token
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"name\": \"...\",
  \"role\": \"...\",
  \"expiry_ms\": 60000,
  \"enabled\": true
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"token\": \"...\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Create {
        admin_auth_token_create_in: crate::json::JsonOf<diom::models::AdminAuthTokenCreateIn>,
    },
    /// Expire an auth token
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token expire {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"expiry_ms\": 60000
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    Expire {
        admin_auth_token_expire_in: crate::json::JsonOf<diom::models::AdminAuthTokenExpireIn>,
    },
    /// Rotate an auth token, invalidating the old one and issuing a new secret
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token rotate {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"id\": \"...\",
  \"token\": \"...\",
  \"created\": 1234567890123,
  \"updated\": 1234567890123
}\n")]
    Rotate {
        admin_auth_token_rotate_in: crate::json::JsonOf<diom::models::AdminAuthTokenRotateIn>,
    },
    /// Delete an auth token
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token delete {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"success\": true
}\n")]
    Delete {
        admin_auth_token_delete_in: crate::json::JsonOf<diom::models::AdminAuthTokenDeleteIn>,
    },
    /// List auth tokens for a given owner
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token list {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"limit\": 123,
  \"iterator\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"data\": [{\"id\": \"...\", \"name\": \"...\", \"created\": 1234567890123, \"updated\": 1234567890123, \"expiry\": 1234567890123, \"role\": \"...\", \"enabled\": true}],
  \"iterator\": \"...\",
  \"prev_iterator\": \"...\",
  \"done\": true
}\n")]
    List {
        admin_auth_token_list_in: Option<crate::json::JsonOf<diom::models::AdminAuthTokenListIn>>,
    },
    /// Update an auth token's properties
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token update {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"id\": \"...\",
  \"name\": \"...\",
  \"expiry_ms\": 60000,
  \"enabled\": true
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}\n")]
    Update {
        admin_auth_token_update_in: crate::json::JsonOf<diom::models::AdminAuthTokenUpdateIn>,
    },
    /// Return the role of the currently authenticated token
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: diom admin auth-token whoami {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "\x1b[1;4mExample body:\x1b[0m
{
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"role\": \"...\"
}\n")]
    Whoami {
        admin_auth_token_whoami_in:
            Option<crate::json::JsonOf<diom::models::AdminAuthTokenWhoamiIn>>,
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
