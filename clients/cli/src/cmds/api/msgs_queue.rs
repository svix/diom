// this file is @generated
use clap::{Args, Subcommand};
use coyote_client::CoyoteClient;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MsgsQueueArgs {
    #[command(subcommand)]
    pub command: MsgsQueueCommands,
}

#[derive(Subcommand)]
pub enum MsgsQueueCommands {
    /// Receives messages from a topic as competing consumers.
    ///
    /// Messages are individually leased for the specified duration. Multiple consumers can receive
    /// different messages from the same topic concurrently. Leased messages are skipped until they
    /// are acked or their lease expires.
    Receive {
        topic: String,
        consumer_group: String,
        msg_queue_receive_in: crate::json::JsonOf<coyote_client::models::MsgQueueReceiveIn>,
    },
    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    Ack {
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: crate::json::JsonOf<coyote_client::models::MsgQueueAckIn>,
    },
}

impl MsgsQueueCommands {
    pub async fn exec(
        self,
        client: &CoyoteClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Receive {
                topic,
                consumer_group,
                msg_queue_receive_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .receive(topic, consumer_group, msg_queue_receive_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Ack {
                topic,
                consumer_group,
                msg_queue_ack_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .ack(topic, consumer_group, msg_queue_ack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
