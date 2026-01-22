// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use diom::{cfg, run, setup_tracing};
use dotenvy::dotenv;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
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

    let cfg = cfg::load()?;

    let (tracing_subscriber, otel_tracer_provider) =
        setup_tracing(&cfg, /* for_test = */ false);
    tracing_subscriber.init();

    match args.command {
        Some(Commands::Healthcheck { .. }) => {
            unreachable!("Healthcheck command should be handled before config loading")
        }
        None => {
            run(cfg).await;
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
