// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

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
    },
    /// Get stream with given name.
    Get {
        get_stream_in: crate::json::JsonOf<coyote_client::models::GetStreamIn>,
    },
    /// Appends messages to the stream.
    Append {
        append_to_stream_in: crate::json::JsonOf<coyote_client::models::AppendToStreamIn>,
    },
    /// Fetches messages from the stream, while allowing concurrent access from other consumers in the same group.
    ///
    /// Unlike `stream.fetch-locking`, this does not block other consumers within the same consumer group from reading
    /// messages from the Stream. The consumer will still take an exclusive lock on the messages fetched, and that lock is held
    /// until the visibility timeout expires, or the messages are acked.
    Fetch {
        fetch_from_stream_in: crate::json::JsonOf<coyote_client::models::FetchFromStreamIn>,
    },
    /// Fetches messages from the stream, locking over the consumer group.
    ///
    /// This call prevents other consumers within the same consumer group from reading from the stream
    /// until either the visibility timeout expires, or the last message in the batch is acknowledged.
    FetchLocking {
        fetch_from_stream_in: crate::json::JsonOf<coyote_client::models::FetchFromStreamIn>,
    },
    /// Acks the messages for the consumer group, allowing more messages to be consumed.
    AckRange {
        ack_msg_range_in: crate::json::JsonOf<coyote_client::models::AckMsgRangeIn>,
    },
    /// Acks a single message.
    Ack {
        ack: crate::json::JsonOf<coyote_client::models::Ack>,
    },
    /// Moves a message to the dead letter queue.
    Dlq {
        dlq_in: crate::json::JsonOf<coyote_client::models::DlqIn>,
    },
    /// Redrives messages from the dead letter queue back to the stream.
    Redrive {
        redrive_in: crate::json::JsonOf<coyote_client::models::RedriveIn>,
    },
}

impl StreamCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create { create_stream_in } => {
                let resp = client
                    .stream()
                    .create(create_stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { get_stream_in } => {
                let resp = client.stream().get(get_stream_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Append {
                append_to_stream_in,
            } => {
                let resp = client
                    .stream()
                    .append(append_to_stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Fetch {
                fetch_from_stream_in,
            } => {
                let resp = client
                    .stream()
                    .fetch(fetch_from_stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::FetchLocking {
                fetch_from_stream_in,
            } => {
                let resp = client
                    .stream()
                    .fetch_locking(fetch_from_stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::AckRange { ack_msg_range_in } => {
                let resp = client
                    .stream()
                    .ack_range(ack_msg_range_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Ack { ack } => {
                let resp = client.stream().ack(ack.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Dlq { dlq_in } => {
                let resp = client.stream().dlq(dlq_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Redrive { redrive_in } => {
                let resp = client.stream().redrive(redrive_in.into_inner()).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
