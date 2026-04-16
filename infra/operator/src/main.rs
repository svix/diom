use diom_operator::crd::DiomCluster;
use kube::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use kube::CustomResourceExt;

    if std::env::args().any(|a| a == "--print-crd") {
        print!("{}", serde_yaml::to_string(&DiomCluster::crd())?);
        return Ok(());
    }

    if std::env::args().any(|a| a == "--print-crd-json") {
        let json = serde_json::to_string_pretty(&DiomCluster::crd())?;
        println!("{json}");
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = Client::try_default().await?;

    diom_operator::run(client).await
}
