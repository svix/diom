// this file is @generated
use clap::{Args, Subcommand};
use coyote::CoyoteClient;

#[allow(unused)]
use crate::prelude::*;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsStreamArgs {
    #[command(subcommand)]
    pub command: MsgsStreamCommands,
}

#[derive(Subcommand)]
pub enum MsgsStreamCommands {
    /// Receives messages from a topic using a consumer group.
    ///
    /// Each consumer in the group reads from all partitions. Messages are locked by leases for the
    /// specified duration to prevent duplicate delivery within the same consumer group.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"batch_size\": \"...\",
  \"lease_duration_ms\": \"...\",
  \"default_starting_position\": \"...\",
  \"batch_wait_ms\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
  \"msgs\": \"...\"
}")]
    Receive {
        topic: String,
        consumer_group: String,
        msg_stream_receive_in: Option<crate::json::JsonOf<coyote::models::MsgStreamReceiveIn>>,
    },
    /// Commits an offset for a consumer group on a specific partition.
    ///
    /// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
    /// successfully processed offset; future receives will start after it.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"offset\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Commit {
        topic: String,
        consumer_group: String,
        msg_stream_commit_in: crate::json::JsonOf<coyote::models::MsgStreamCommitIn>,
    },
    /// Repositions a consumer group's read cursor on a topic.
    ///
    /// Provide exactly one of `offset` or `position`. When using `offset`, the topic must include a
    /// partition suffix (e.g. `ns:my-topic~0`). The `position` field accepts `"earliest"` or
    /// `"latest"` and may be used with or without a partition suffix.
    #[command(after_long_help = "\x1b[1;4mExample body:\x1b[0m
{
  \"namespace\": \"...\",
  \"offset\": \"...\",
  \"position\": \"...\"
}\n\n\x1b[1;4mExample response:\x1b[0m
{
}")]
    Seek {
        topic: String,
        consumer_group: String,
        msg_stream_seek_in: Option<crate::json::JsonOf<coyote::models::MsgStreamSeekIn>>,
    },
}

impl MsgsStreamCommands {
    pub async fn exec(self, client: &CoyoteClient) -> anyhow::Result<()> {
        match self {
            Self::Receive {
                topic,
                consumer_group,
                msg_stream_receive_in,
            } => {
                let resp = client
                    .msgs()
                    .stream()
                    .receive(
                        topic,
                        consumer_group,
                        msg_stream_receive_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Commit {
                topic,
                consumer_group,
                msg_stream_commit_in,
            } => {
                let resp = client
                    .msgs()
                    .stream()
                    .commit(topic, consumer_group, msg_stream_commit_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
            Self::Seek {
                topic,
                consumer_group,
                msg_stream_seek_in,
            } => {
                let resp = client
                    .msgs()
                    .stream()
                    .seek(
                        topic,
                        consumer_group,
                        msg_stream_seek_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp)?;
            }
        }

        Ok(())
    }
}
