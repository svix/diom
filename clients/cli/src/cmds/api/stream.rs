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

#[derive(Args, Clone)]
pub struct StreamFetchOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamFetchOptions> for coyote_client::api::StreamFetchOptions {
    fn from(value: StreamFetchOptions) -> Self {
        let StreamFetchOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamFetchLockingOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamFetchLockingOptions> for coyote_client::api::StreamFetchLockingOptions {
    fn from(value: StreamFetchLockingOptions) -> Self {
        let StreamFetchLockingOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamAckOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamAckOptions> for coyote_client::api::StreamAckOptions {
    fn from(value: StreamAckOptions) -> Self {
        let StreamAckOptions { idempotency_key } = value;
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
    /// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
    ///
    /// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
    /// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
    /// until the visibility timeout expires, or the messages are acked.
    Fetch {
        fetch_from_stream_in: crate::json::JsonOf<coyote_client::models::FetchFromStreamIn>,
        #[clap(flatten)]
        options: StreamFetchOptions,
    },
    /// Fetches messages from the stream, locking over the consumer group.
    ///
    /// This call prevents other consumers within the same consumer group from reading from the stream
    /// until either the visibility timeout expires, or the last message in the batch is acknowledged.
    FetchLocking {
        fetch_from_stream_in: crate::json::JsonOf<coyote_client::models::FetchFromStreamIn>,
        #[clap(flatten)]
        options: StreamFetchLockingOptions,
    },
    /// Acks the messages for the consumer group, allowing more messages to be consumed.
    Ack {
        ack_in: crate::json::JsonOf<coyote_client::models::AckIn>,
        #[clap(flatten)]
        options: StreamAckOptions,
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
            Self::Fetch {
                fetch_from_stream_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .fetch(fetch_from_stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::FetchLocking {
                fetch_from_stream_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .fetch_locking(fetch_from_stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Ack { ack_in, options } => {
                let resp = client
                    .stream()
                    .ack(ack_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
