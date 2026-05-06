use diom_backend::cfg::{LogFormat, build_fmt_layer};
use diom_operator::crd::DiomCluster;
use kube::Client;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use kube::CustomResourceExt;

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    if std::env::args().any(|a| a == "--print-crd") {
        print!("{}", serde_yaml::to_string(&DiomCluster::crd())?);
        return Ok(());
    }

    if std::env::args().any(|a| a == "--print-crd-json") {
        let json = serde_json::to_string_pretty(&DiomCluster::crd())?;
        println!("{json}");
        return Ok(());
    }

    let log_format = std::env::var("LOG_FORMAT")
        .ok()
        .and_then(|s| s.parse::<LogFormat>().ok())
        .unwrap_or_default();

    tracing_subscriber::registry()
        .with(build_fmt_layer(log_format))
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = Client::try_default().await?;

    diom_operator::run(client).await
}
