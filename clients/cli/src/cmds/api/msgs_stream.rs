// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

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
    Receive {
        msg_stream_receive_in: crate::json::JsonOf<coyote_client::models::MsgStreamReceiveIn>,
    },
    /// Commits an offset for a consumer group on a specific partition.
    ///
    /// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
    /// successfully processed offset; future receives will start after it.
    Commit {
        msg_stream_commit_in: crate::json::JsonOf<coyote_client::models::MsgStreamCommitIn>,
    },
}

impl MsgsStreamCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Receive {
                msg_stream_receive_in,
            } => {
                let resp = client
                    .msgs()
                    .stream()
                    .receive(msg_stream_receive_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Commit {
                msg_stream_commit_in,
            } => {
                let resp = client
                    .msgs()
                    .stream()
                    .commit(msg_stream_commit_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
