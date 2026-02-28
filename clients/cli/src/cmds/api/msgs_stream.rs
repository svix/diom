// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

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
        stream_receive_in: crate::json::JsonOf<diom_client::models::StreamReceiveIn>,
    },
    /// Commits an offset for a consumer group on a specific partition.
    ///
    /// The topic must be a partition-level topic (e.g. `ns:my-topic~3`). The offset is the last
    /// successfully processed offset; future receives will start after it.
    Commit {
        stream_commit_in: crate::json::JsonOf<diom_client::models::StreamCommitIn>,
    },
}

impl MsgsStreamCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Receive { stream_receive_in } => {
                let resp = client
                    .msgs()
                    .stream()
                    .receive(stream_receive_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Commit { stream_commit_in } => {
                let resp = client
                    .msgs()
                    .stream()
                    .commit(stream_commit_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
