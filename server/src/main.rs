// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use comfy_table::{Cell, Table};
use coyote::{
    cfg::{self, Configuration},
    run,
};
use dotenvy::dotenv;
use mimalloc::MiMalloc;
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};
use tracing_subscriber::util::SubscriberInitExt;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod otel;
mod tracing_panic_hook;

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
    /// Path to a TOML configuration file
    #[clap(short = 'C', long)]
    config_path: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Health check command
    Healthcheck {
        /// If not passed, will use the port in the config
        server_url: Option<String>,
    },
    /// Run the server (this is also the default if no subcommand is passed)
    Server,
    /// Write the current config out as a TOML file
    ///
    /// This will take into account any environment variables passed, as well as
    /// the contents of --config-path.
    WriteConfig {
        /// Path to write to; if not specified, writes to stdout
        path: Option<PathBuf>,
    },
    /// Describe environment variables honored by this service
    DescribeEnvironmentVariables,
}

fn dump_config(cfg: Configuration, path: Option<PathBuf>) -> anyhow::Result<()> {
    let str = toml::to_string_pretty(&cfg)?;
    if let Some(path) = path
        && path.to_str() != Some("-")
    {
        let f = fs_err::File::open(path)?;
        let mut bf = BufWriter::new(f);
        write!(bf, "{str}")?;
    } else {
        print!("{str}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenv();

    let args = Args::parse();

    if let Some(Commands::Healthcheck { server_url }) = args.command {
        let server_url = if let Some(url) = server_url {
            url
        } else {
            // only load the cfg if we didn't pass --server-url
            let cfg = cfg::load(args.config_path.as_deref())?;
            let server_address = if cfg.listen_address.ip().is_unspecified() {
                format!("http://localhost:{}", cfg.listen_address.port())
            } else {
                cfg.listen_address.to_string()
            };
            format!("http://{server_address}")
        };
        let client = reqwest::Client::new();
        let response = client
            .head(format!("{server_url}/api/v1.health"))
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
        Some(Commands::DescribeEnvironmentVariables) => {
            let mut table = Table::new();
            let rows = cfg::describe_environment()
                .into_iter()
                .map(|var| {
                    [
                        Cell::new(var.env_var),
                        Cell::new(var.docstring.unwrap_or_default()),
                    ]
                })
                .collect::<Vec<_>>();
            table
                .load_preset(comfy_table::presets::UTF8_FULL)
                .set_header(["Environment Variable", "Description"])
                .add_rows(rows);
            println!("{table}");
            cfg::describe_environment();
        }
        Some(Commands::Server) | None => {
            otel::setup_metrics(&cfg);
            run(cfg).await
        }
        Some(Commands::WriteConfig { path }) => dump_config(cfg, path)?,
    };

    #[allow(clippy::disallowed_methods)]
    if let Some(provider) = otel_tracer_provider {
        _ = tokio::task::spawn_blocking(move || {
            _ = provider.shutdown();
        })
        .await;
    }

    Ok(())
}
