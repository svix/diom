// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

use super::{AdminAuthPolicyArgs, AdminAuthRoleArgs, AdminAuthTokenArgs, AdminClusterArgs};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommands,
}

#[derive(Subcommand)]
pub enum AdminCommands {
    AuthPolicy(AdminAuthPolicyArgs),
    AuthRole(AdminAuthRoleArgs),
    AuthToken(AdminAuthTokenArgs),
    Cluster(AdminClusterArgs),
}

impl AdminCommands {
    pub async fn exec(self, client: &DiomClient) -> anyhow::Result<()> {
        match self {
            Self::AuthPolicy(args) => {
                args.command.exec(client).await?;
            }
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
