// this file is @generated
use clap::{Args, Subcommand};
use diom_client::DiomClient;

#[derive(Args, Clone)]
pub struct KvSetOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<KvSetOptions> for diom_client::api::KvSetOptions {
    fn from(value: KvSetOptions) -> Self {
        let KvSetOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct KvGetOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<KvGetOptions> for diom_client::api::KvGetOptions {
    fn from(value: KvGetOptions) -> Self {
        let KvGetOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct KvGetGroupOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<KvGetGroupOptions> for diom_client::api::KvGetGroupOptions {
    fn from(value: KvGetGroupOptions) -> Self {
        let KvGetGroupOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct KvDeleteOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<KvDeleteOptions> for diom_client::api::KvDeleteOptions {
    fn from(value: KvDeleteOptions) -> Self {
        let KvDeleteOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct KvArgs {
    #[command(subcommand)]
    pub command: KvCommands,
}

#[derive(Subcommand)]
pub enum KvCommands {
    /// KV Set
    Set {
        kv_set_in: crate::json::JsonOf<diom_client::models::KvSetIn>,
        #[clap(flatten)]
        options: KvSetOptions,
    },
    /// KV Get
    Get {
        kv_get_in: crate::json::JsonOf<diom_client::models::KvGetIn>,
        #[clap(flatten)]
        options: KvGetOptions,
    },
    /// Get KV store
    GetGroup {
        kv_get_group_in: crate::json::JsonOf<diom_client::models::KvGetGroupIn>,
        #[clap(flatten)]
        options: KvGetGroupOptions,
    },
    /// KV Delete
    Delete {
        kv_delete_in: crate::json::JsonOf<diom_client::models::KvDeleteIn>,
        #[clap(flatten)]
        options: KvDeleteOptions,
    },
}

impl KvCommands {
    pub async fn exec(
        self,
        client: &DiomClient,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Set { kv_set_in, options } => {
                let resp = client
                    .kv()
                    .set(kv_set_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { kv_get_in, options } => {
                let resp = client
                    .kv()
                    .get(kv_get_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetGroup {
                kv_get_group_in,
                options,
            } => {
                let resp = client
                    .kv()
                    .get_group(kv_get_group_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                kv_delete_in,
                options,
            } => {
                let resp = client
                    .kv()
                    .delete(kv_delete_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
