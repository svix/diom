use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use colored_json::{ColorMode, Output};
use concolor_clap::{Color, ColorChoice};
use diom_client::{DiomClient, DiomOptions};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use self::{
    cmds::{
        api::{CacheArgs, HealthArgs, IdempotencyArgs, KvArgs, MsgsArgs, RateLimiterArgs},
        benchmark::BenchmarkArgs,
    },
    config::Config,
};

mod cmds;
mod config;
mod json;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &str = env!("CARGO_BIN_NAME");

#[derive(Parser)]
#[command(version, about, long_about = None, bin_name = BIN_NAME)]
#[clap(color = concolor_clap::color_choice())]
struct Cli {
    #[command(flatten)]
    color: Color,
    #[arg(
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Log more. This option may be repeated up to 3 times"
    )]
    verbose: u8,
    #[command(subcommand)]
    command: RootCommands,
}

impl Cli {
    /// Converts the selected `ColorChoice` from the CLI to a `ColorMode` as used by the JSON printer.
    ///
    /// When the color choice is "auto", this considers whether stdout is a tty or not so that
    /// color codes are only produced when actually writing directly to a terminal.
    fn color_mode(&self) -> ColorMode {
        match self.color.color {
            ColorChoice::Auto => ColorMode::Auto(Output::StdOut),
            ColorChoice::Always => ColorMode::On,
            ColorChoice::Never => ColorMode::Off,
        }
    }

    fn log_level(&self) -> tracing::Level {
        match self.verbose {
            3.. => tracing::Level::TRACE,
            2 => tracing::Level::DEBUG,
            1 => tracing::Level::INFO,
            0 => tracing::Level::WARN,
        }
    }
}

// N.b Ordering matters here for how clap presents the help.
#[derive(Subcommand)]
enum RootCommands {
    Cache(CacheArgs),
    Idempotency(IdempotencyArgs),
    Kv(KvArgs),
    Msgs(MsgsArgs),
    RateLimit(RateLimiterArgs),
    Health(HealthArgs),
    /// Benchmark module throughput
    Benchmark(BenchmarkArgs),
    /// Get the version of the Svix CLI
    Version,
    /// Generate the autocompletion script for the specified shell
    Completion {
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let color_mode = cli.color_mode();

    tracing_subscriber::fmt()
        .with_max_level(cli.log_level())
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .init();

    // rustls requires a crypto backend ("provider") choice to be made explicitly
    // The Svix SDK uses the default provider if a default is not installed, but
    // we use reqwest directly in some code paths, which does not do this.
    _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    // XXX: cfg can give an Err in certain situations.
    // Assigning the variable here since several match arms need a `&Config` but the rest of them
    // won't care/are still usable if the config doesn't exist.
    // To this, the `?` is deferred until the point inside a given match arm needs the config value.
    let cfg = Config::load();
    match cli.command {
        // Local-only things
        RootCommands::Version => println!("{VERSION}"),
        RootCommands::Completion { shell } => cmds::completion::generate(&shell)?,

        // Remote API calls
        RootCommands::Cache(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Idempotency(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Kv(args) => {
            let cfg = cfg?;
            let client = get_client(&cfg)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Msgs(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::RateLimit(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Health(args) => {
            let client = get_client(&cfg?)?;
            args.command.exec(&client, color_mode).await?;
        }
        RootCommands::Benchmark(args) => {
            let cfg = cfg?;
            let mut opts = get_client_options(&cfg)?;
            if let Some(url) = args.server_url.clone() {
                opts.server_url = Some(url);
            }
            let client = Arc::new(DiomClient::new("xxx".to_owned(), Some(opts)));
            args.exec(client).await?;
        }
    }

    Ok(())
}

fn get_client(cfg: &Config) -> Result<DiomClient> {
    // FIXME: Add login functionality once there is authn in the server
    /* let token = cfg.auth_token.clone().ok_or_else(|| {
        anyhow::anyhow!("No auth token set. Try running `{BIN_NAME} login` to get started.")
    })?; */
    let opts = get_client_options(cfg)?;
    Ok(DiomClient::new("xxx".to_owned(), Some(opts)))
}

fn get_client_options(cfg: &Config) -> Result<DiomOptions> {
    Ok(DiomOptions {
        debug: false,
        server_url: cfg.server_url().map(Into::into),
        timeout: None,
        #[cfg(all(feature = "http1", feature = "http2"))]
        http1: cfg.http1,
        ..DiomOptions::default()
    })
}
