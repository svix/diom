// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args, Clone)]
pub struct IdempotencyAbortOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IdempotencyAbortOptions> for diom_client::api::IdempotencyAbortOptions {
    fn from(value: IdempotencyAbortOptions) -> Self {
        let IdempotencyAbortOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct IdempotencyGetGroupOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IdempotencyGetGroupOptions> for diom_client::api::IdempotencyGetGroupOptions {
    fn from(value: IdempotencyGetGroupOptions) -> Self {
        let IdempotencyGetGroupOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IdempotencyArgs {
    #[command(subcommand)]
    pub command: IdempotencyCommands,
}

#[derive(Subcommand)]
pub enum IdempotencyCommands {
    /// Abandon an idempotent request (remove lock without saving response)
    Abort {
        idempotency_abort_in: crate::json::JsonOf<diom_client::models::IdempotencyAbortIn>,
        #[clap(flatten)]
        options: IdempotencyAbortOptions,
    },
    /// Get idempotency group
    GetGroup {
        idempotency_get_group_in: crate::json::JsonOf<diom_client::models::IdempotencyGetGroupIn>,
        #[clap(flatten)]
        options: IdempotencyGetGroupOptions,
    },
}

impl IdempotencyCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Abort {
                idempotency_abort_in,
                options,
            } => {
                let resp = client
                    .idempotency()
                    .abort(idempotency_abort_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetGroup {
                idempotency_get_group_in,
                options,
            } => {
                let resp = client
                    .idempotency()
                    .get_group(idempotency_get_group_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
