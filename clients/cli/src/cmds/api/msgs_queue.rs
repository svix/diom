// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

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
        msg_queue_receive_in: crate::json::JsonOf<diom_client::models::MsgQueueReceiveIn>,
    },
    /// Acknowledges messages by their opaque msg_ids.
    ///
    /// Acked messages are permanently removed from the queue and will never be re-delivered.
    Ack {
        topic: String,
        consumer_group: String,
        msg_queue_ack_in: crate::json::JsonOf<diom_client::models::MsgQueueAckIn>,
    },
    /// Rejects messages, sending them to the dead-letter queue.
    ///
    /// Nacked messages will not be re-delivered by `queue/receive`. Use `queue/redrive-dlq` to
    /// move them back to the queue for reprocessing.
    Nack {
        topic: String,
        consumer_group: String,
        msg_queue_nack_in: crate::json::JsonOf<diom_client::models::MsgQueueNackIn>,
    },
    /// Moves all dead-letter queue messages back to the main queue for reprocessing.
    RedriveDlq {
        topic: String,
        consumer_group: String,
        msg_queue_redrive_dlq_in: crate::json::JsonOf<diom_client::models::MsgQueueRedriveDlqIn>,
    },
}

impl MsgsQueueCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
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
            Self::Nack {
                topic,
                consumer_group,
                msg_queue_nack_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .nack(topic, consumer_group, msg_queue_nack_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RedriveDlq {
                topic,
                consumer_group,
                msg_queue_redrive_dlq_in,
            } => {
                let resp = client
                    .msgs()
                    .queue()
                    .redrive_dlq(topic, consumer_group, msg_queue_redrive_dlq_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
