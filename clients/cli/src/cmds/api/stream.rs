// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args, Clone)]
pub struct StreamCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamCreateOptions> for coyote_client::api::StreamCreateOptions {
    fn from(value: StreamCreateOptions) -> Self {
        let StreamCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamAppendOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamAppendOptions> for coyote_client::api::StreamAppendOptions {
    fn from(value: StreamAppendOptions) -> Self {
        let StreamAppendOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamArgs {
    #[command(subcommand)]
    pub command: StreamCommands,
}

#[derive(Subcommand)]
pub enum StreamCommands {
    /// Upserts a new Stream with the given name.
    Create {
        create_stream_in: crate::json::JsonOf<coyote_client::models::CreateStreamIn>,
        #[clap(flatten)]
        options: StreamCreateOptions,
    },
    /// Appends messages to the stream.
    Append {
        append_to_stream_in: crate::json::JsonOf<coyote_client::models::AppendToStreamIn>,
        #[clap(flatten)]
        options: StreamAppendOptions,
    },
}

impl StreamCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                create_stream_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .create(create_stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Append {
                append_to_stream_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .append(append_to_stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
