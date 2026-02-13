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
        }

        Ok(())
    }
}
