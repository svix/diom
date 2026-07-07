use std::{sync::Arc, time::Duration};

use futures_util::StreamExt;
use k8s_openapi::api::apps::v1::StatefulSet;
use kube::{
    Api, Client,
    runtime::{Controller, controller::Error as ControllerError, watcher},
};
use tracing::*;

pub mod context;
pub mod crd;
pub mod error;
pub mod labels;
pub mod reconciler;
pub mod resources;

use crd::DiomCluster;
use reconciler::{Context, STALL_THRESHOLD, error_policy, reconcile};

pub async fn run(client: Client) -> anyhow::Result<()> {
    run_with_requeue(client, Duration::from_secs(60)).await
}

pub async fn run_with_requeue(client: Client, requeue_interval: Duration) -> anyhow::Result<()> {
    run_with_config(client, requeue_interval, STALL_THRESHOLD).await
}

pub async fn run_with_config(
    client: Client,
    requeue_interval: Duration,
    stall_threshold: Duration,
) -> anyhow::Result<()> {
    let clusters: Api<DiomCluster> = Api::all(client.clone());
    let statefulsets: Api<StatefulSet> = Api::all(client.clone());

    info!("Starting diom-operator");

    let ctx = Arc::new(Context {
        client,
        requeue_interval,
        stall_threshold,
    });

    Controller::new(clusters, watcher::Config::default())
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
                Err(ControllerError::QueueError(watcher::Error::WatchError(status)))
                    if status.code == 410 =>
                {
                    warn!("Watch expired, relisting: {status:?}");
                }
                Err(err) => error!("Reconcile error: {err:?}"),
            }
        })
        .await;

    Ok(())
}
