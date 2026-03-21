use std::sync::Arc;

use futures_util::StreamExt;
use k8s_openapi::api::apps::v1::StatefulSet;
use kube::{
    Api, Client,
    runtime::{Controller, watcher},
};
use tracing::*;

mod crd;
mod error;
mod labels;
mod reconciler;
mod resources;

use crd::DiomCluster;
use reconciler::{Context, error_policy, reconcile};

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

    let clusters: Api<DiomCluster> = Api::all(client.clone());
    let statefulsets: Api<StatefulSet> = Api::all(client.clone());

    info!("Starting diom-operator");

    let ctx = Arc::new(Context {
        client: client.clone(),
    });

    Controller::new(clusters, watcher::Config::default())
        // Re-reconcile the owning DiomCluster whenever a managed StatefulSet changes.
        .owns(statefulsets, watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, ctx)
        .for_each(|res| async move {
            match res {
                Ok(obj) => info!(
                    "Reconciled {}/{}",
                    obj.0.namespace.as_deref().unwrap_or(""),
                    obj.0.name
                ),
                Err(err) => error!("Reconcile error: {err:?}"),
            }
        })
        .await;

    Ok(())
}
