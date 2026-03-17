// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use coyote::{cfg, run_with_listeners};
use dotenvy::dotenv;
use mimalloc::MiMalloc;
use tracing_subscriber::util::SubscriberInitExt;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod otel;
mod tracing_panic_hook;

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
    #[clap(long)]
    config_path: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Health check command
    Healthcheck {
        // FIXME: we should make it optional and default to localhost with the settings (when ran
        // in docker)
        /// The server URL, for example http://localhost:8050
        server_url: String,
    },
    /// Run the server (this is also the default if no subcommand is passed)
    Server,
}

#[tokio::main(flavor = "local")]
async fn main() -> anyhow::Result<()> {
    _ = dotenv();

    let args = Args::parse();

    // Handle commands that don't need configuration first
    if let Some(Commands::Healthcheck { server_url }) = args.command {
        let client = reqwest::Client::new();
        let response = client
            .head(format!("{server_url}/api/v1/health"))
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        } else {
            return Err(anyhow::anyhow!(
                "healthcheck failed ({})",
                response.status()
            ));
        }
    }

    let cfg = cfg::load(args.config_path.as_deref())?;

    let (tracing_subscriber, otel_tracer_provider) =
        otel::setup_tracing(&cfg, /* for_test = */ false);
    tracing_subscriber.init();

    tracing_panic_hook::setup_tracing_panic_handler();

    match args.command {
        Some(Commands::Healthcheck { .. }) => {
            unreachable!("Healthcheck command should be handled before config loading")
        }
        Some(Commands::Server) | None => {
            otel::setup_metrics(&cfg);
            run_with_listeners(cfg, None, None).await
        }
    };

    if let Some(provider) = otel_tracer_provider {
        _ = tokio::task::spawn_blocking(move || {
            _ = provider.shutdown();
        })
        .await;
    }

    Ok(())
}
