#![allow(clippy::result_large_err)]

use coyote::run;
use once_cell::sync::Lazy;
use svix_ksuid::{KsuidLike, KsuidMs};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub static INSTANCE_ID: Lazy<String> = Lazy::new(|| KsuidMs::new(None, None).to_string());

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server with instance_id: {}", *INSTANCE_ID);

    run().await;
}
