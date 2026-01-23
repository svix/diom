// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args, Clone)]
pub struct QueueSendOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueSendOptions> for diom_client::api::QueueSendOptions {
    fn from(value: QueueSendOptions) -> Self {
        let QueueSendOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueueReceiveOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueReceiveOptions> for diom_client::api::QueueReceiveOptions {
    fn from(value: QueueReceiveOptions) -> Self {
        let QueueReceiveOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueueAckOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueAckOptions> for diom_client::api::QueueAckOptions {
    fn from(value: QueueAckOptions) -> Self {
        let QueueAckOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueueNackOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueNackOptions> for diom_client::api::QueueNackOptions {
    fn from(value: QueueNackOptions) -> Self {
        let QueueNackOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueueRejectOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueRejectOptions> for diom_client::api::QueueRejectOptions {
    fn from(value: QueueRejectOptions) -> Self {
        let QueueRejectOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueuePurgeOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueuePurgeOptions> for diom_client::api::QueuePurgeOptions {
    fn from(value: QueuePurgeOptions) -> Self {
        let QueuePurgeOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct QueueStatsOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<QueueStatsOptions> for diom_client::api::QueueStatsOptions {
    fn from(value: QueueStatsOptions) -> Self {
        let QueueStatsOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct QueueArgs {
    #[command(subcommand)]
    pub command: QueueCommands,
}

#[derive(Subcommand)]
pub enum QueueCommands {
    /// Send messages to the queue
    Send {
        queue_send_in: crate::json::JsonOf<diom_client::models::QueueSendIn>,
        #[clap(flatten)]
        options: QueueSendOptions,
    },
    /// Receive messages from the queue
    Receive {
        queue_receive_in: crate::json::JsonOf<diom_client::models::QueueReceiveIn>,
        #[clap(flatten)]
        options: QueueReceiveOptions,
    },
    /// Acknowledge successful message processing
    Ack {
        queue_ack_in: crate::json::JsonOf<diom_client::models::QueueAckIn>,
        #[clap(flatten)]
        options: QueueAckOptions,
    },
    /// Negative acknowledge - return message to queue or move to DLQ
    Nack {
        queue_nack_in: crate::json::JsonOf<diom_client::models::QueueNackIn>,
        #[clap(flatten)]
        options: QueueNackOptions,
    },
    /// Reject a message - remove from processing and send to DLQ without retry
    Reject {
        queue_reject_in: crate::json::JsonOf<diom_client::models::QueueRejectIn>,
        #[clap(flatten)]
        options: QueueRejectOptions,
    },
    /// Purge all messages from a queue
    Purge {
        queue_purge_in: crate::json::JsonOf<diom_client::models::QueuePurgeIn>,
        #[clap(flatten)]
        options: QueuePurgeOptions,
    },
    /// Get queue statistics
    Stats {
        queue_stats_in: crate::json::JsonOf<diom_client::models::QueueStatsIn>,
        #[clap(flatten)]
        options: QueueStatsOptions,
    },
}

impl QueueCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Send {
                queue_send_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .send(queue_send_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Receive {
                queue_receive_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .receive(queue_receive_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Ack {
                queue_ack_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .ack(queue_ack_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Nack {
                queue_nack_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .nack(queue_nack_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Reject {
                queue_reject_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .reject(queue_reject_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Purge {
                queue_purge_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .purge(queue_purge_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Stats {
                queue_stats_in,
                options,
            } => {
                let resp = client
                    .queue()
                    .stats(queue_stats_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
