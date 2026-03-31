// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

use super::{AdminAuthRoleArgs, AdminAuthTokenArgs, AdminClusterArgs};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommands,
}

#[derive(Subcommand)]
pub enum AdminCommands {
    AuthRole(AdminAuthRoleArgs),
    AuthToken(AdminAuthTokenArgs),
    Cluster(AdminClusterArgs),
}

impl AdminCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::AuthRole(args) => {
                args.command.exec(client).await?;
            }
            Self::AuthToken(args) => {
                args.command.exec(client).await?;
            }
            Self::Cluster(args) => {
                args.command.exec(client).await?;
            }
        }

        Ok(())
    }
}
