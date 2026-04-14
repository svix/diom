use std::collections::BTreeMap;

use k8s_openapi::{
    api::{core::v1::PersistentVolumeClaim, storage::v1::StorageClass},
    apimachinery::pkg::api::resource::Quantity,
};
use kube::{
    Api,
    api::{ListParams, Patch, PatchParams},
};
use kube_quantity::{ParseQuantityError, ParsedQuantity};

use crate::{
    context::ClusterCtx,
    error::{Error, Result},
};

/// Reconcile persistent volume claims.
pub(crate) async fn reconcile(ctx: &ClusterCtx) -> Result<()> {
    let storage = &ctx.cluster.spec.diom.storage;
    let nodes = ctx.cluster.spec.diom.replicas;

    reconcile_volume(
        ctx,
        "persistent",
        &storage.persistent.size,
        storage.persistent.storage_class.as_deref(),
        nodes,
    )
    .await?;

    if let Some(logs) = &storage.logs {
        reconcile_volume(
            ctx,
            "logs",
            &logs.size,
            logs.storage_class.as_deref(),
            nodes,
        )
        .await?;
    }

    if let Some(snaps) = &storage.snapshots {
        reconcile_volume(
            ctx,
            "snapshots",
            &snaps.size,
            snaps.storage_class.as_deref(),
            nodes,
        )
        .await?;
    }

    Ok(())
}

async fn reconcile_volume(
    ctx: &ClusterCtx,
    volume_name: &str,
    desired_size: &Quantity,
    storage_class: Option<&str>,
    nodes: i32,
) -> Result<()> {
    let pvc_api: Api<PersistentVolumeClaim> = Api::namespaced(ctx.client.clone(), &ctx.ns);

    for ordinal in 0..nodes {
        let name = pvc_name(volume_name, &ctx.name, ordinal);

        let Some(pvc) = pvc_api.get_opt(&name).await? else {
            tracing::warn!(name, "PVC not found; skipping reconcile.");
            continue;
        };

        let desired_size: ParsedQuantity = desired_size
            .try_into()
            .map_err(|e: ParseQuantityError| Error::Storage(e.to_string()))?;
        let current_size = pvc_requested_size(&pvc)?;

        if desired_size <= current_size {
            if desired_size < current_size {
                tracing::warn!(name, "Desired size smaller than current; skipping resize.");
            }
            continue;
        }

        let sc_name = pvc
            .spec
            .as_ref()
            .and_then(|s| s.storage_class_name.as_deref())
            .or(storage_class);

        if !storage_class_allows_expansion(&ctx.client, sc_name).await? {
            tracing::warn!(
                name,
                sc_name,
                "StorageClass does not support resize; skipping."
            );
            continue;
        }

        patch_pvc_size(&pvc_api, &name, &desired_size).await?;
    }

    Ok(())
}

async fn patch_pvc_size(
    pvc_api: &Api<PersistentVolumeClaim>,
    name: &str,
    desired: &ParsedQuantity,
) -> Result<()> {
    let mut requests: BTreeMap<String, String> = BTreeMap::new();
    requests.insert("storage".into(), desired.to_string());
    let patch = serde_json::json!({
        "spec": {
            "resources": {
                "requests": requests
            }
        }
    });
    tracing::info!(
        name,
        %desired,
        "Patching PVC with new size"
    );
    pvc_api
        .patch(name, &PatchParams::default(), &Patch::Merge(&patch))
        .await?;
    Ok(())
}

async fn storage_class_allows_expansion(
    client: &kube::Client,
    sc_name: Option<&str>,
) -> Result<bool> {
    let sc_api: Api<StorageClass> = Api::all(client.clone());

    let sc = match sc_name {
        Some(name) => sc_api.get_opt(name).await?,
        None => {
            let list = sc_api.list(&ListParams::default()).await?;
            list.items.into_iter().find(|sc| {
                sc.metadata
                    .annotations
                    .as_ref()
                    .and_then(|a| a.get("storageclass.kubernetes.io/is-default-class"))
                    .map(|v| v == "true")
                    .unwrap_or(false)
            })
        }
    };

    Ok(sc
        .as_ref()
        .and_then(|sc| sc.allow_volume_expansion)
        .unwrap_or(false))
}

fn pvc_requested_size(pvc: &PersistentVolumeClaim) -> Result<ParsedQuantity> {
    pvc.spec
        .as_ref()
        .and_then(|s| s.resources.as_ref()?.requests.as_ref()?.get("storage"))
        .cloned()
        .ok_or_else(|| Error::Storage("Storage not found".to_owned()))?
        .try_into()
        .map_err(|e: ParseQuantityError| Error::Storage(e.to_string()))
}

pub(crate) fn pvc_name(volume_name: &str, cluster_name: &str, ordinal: i32) -> String {
    format!("{volume_name}-{cluster_name}-{ordinal}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_openapi::api::core::v1::{PersistentVolumeClaimSpec, VolumeResourceRequirements};

    fn pvc_with_storage(size: &str) -> PersistentVolumeClaim {
        let mut requests = BTreeMap::new();
        requests.insert("storage".to_string(), Quantity(size.to_string()));
        PersistentVolumeClaim {
            spec: Some(PersistentVolumeClaimSpec {
                resources: Some(VolumeResourceRequirements {
                    requests: Some(requests),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_pvc_name() {
        assert_eq!(
            pvc_name("persistent", "mycluster", 0),
            "persistent-mycluster-0"
        );
        assert_eq!(pvc_name("logs", "mycluster", 2), "logs-mycluster-2");
    }

    #[test]
    fn test_pvc_requested_size() {
        let size = pvc_requested_size(&pvc_with_storage("10Gi")).unwrap();
        assert_eq!(size, Quantity("10Gi".to_string()).try_into().unwrap());

        assert!(pvc_requested_size(&PersistentVolumeClaim::default()).is_err());

        let pvc = PersistentVolumeClaim {
            spec: Some(PersistentVolumeClaimSpec::default()),
            ..Default::default()
        };
        assert!(pvc_requested_size(&pvc).is_err());
    }
}
