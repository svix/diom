use std::collections::{BTreeMap, HashSet};

use k8s_openapi::{
    api::{apps::v1::StatefulSet, core::v1::PersistentVolumeClaim, storage::v1::StorageClass},
    apimachinery::pkg::api::resource::Quantity,
};
use kube::{
    Api, Client,
    api::{Patch, PatchParams},
};
use tracing::*;

use crate::{crd::DiomCluster, error::Result};

/// Describes a single PVC that failed to expand, together with the size it should be reverted to.
pub(crate) struct PvcExpansionFailure {
    pub(crate) template_name: String,
    pub(crate) pvc_name: String,
    pub(crate) original_size: Quantity,
    pub(crate) reason: String,
}

/// Returns true if the desired StatefulSet's volumeClaimTemplates differ from the existing one
/// in any way that requires orphan-deletion to apply (storage class, size, or a template being
/// added/removed). Comparison is name-keyed rather than positional so that a reordering of
/// templates in the existing STS (e.g. from a manual edit) does not produce a false positive.
pub(crate) fn volume_claim_templates_changed(
    existing: &StatefulSet,
    desired: &StatefulSet,
) -> bool {
    let existing_map: BTreeMap<&str, &PersistentVolumeClaim> = sts_volume_claim_templates(existing)
        .iter()
        .filter_map(|t| t.metadata.name.as_deref().map(|n| (n, t)))
        .collect();

    let desired_map: BTreeMap<&str, &PersistentVolumeClaim> = sts_volume_claim_templates(desired)
        .iter()
        .filter_map(|t| t.metadata.name.as_deref().map(|n| (n, t)))
        .collect();

    if existing_map.len() != desired_map.len() {
        return true;
    }

    // A template was added, or its storage class / size changed.
    desired_map
        .iter()
        .any(|(name, d)| match existing_map.get(name) {
            None => true,
            Some(e) => {
                pvc_storage_class(e) != pvc_storage_class(d)
                    || pvc_storage_request(e) != pvc_storage_request(d)
            }
        })
}

/// For each volume claim template whose desired size differs from the existing one, attempt to
/// expand each PVC for every replica. Returns the list of [`PvcExpansionFailure`]s and the set
/// of template names that had at least one successful expansion.
///
/// The caller should only orphan-delete the StatefulSet when at least one expansion succeeded;
/// if every attempt failed the CR will have been reverted and STS recreation is unnecessary.
///
/// Templates whose storage class does not support volume expansion are recorded as failures
/// (not silently skipped) so the CR can be reverted and an event emitted.
pub(crate) async fn try_expand_pvcs(
    existing_sts: &StatefulSet,
    desired_sts: &StatefulSet,
    client: &Client,
    ns: &str,
) -> Result<(Vec<PvcExpansionFailure>, HashSet<String>)> {
    let sts_name = existing_sts.metadata.name.as_deref().unwrap_or("");
    let existing_replicas = existing_sts
        .spec
        .as_ref()
        .and_then(|s| s.replicas)
        .unwrap_or(0);

    let pvc_api: Api<PersistentVolumeClaim> = Api::namespaced(client.clone(), ns);
    let sc_api: Api<StorageClass> = Api::all(client.clone());

    let existing_templates = sts_volume_claim_templates(existing_sts);
    let mut failures: Vec<PvcExpansionFailure> = Vec::new();
    let mut expanded_templates: HashSet<String> = HashSet::new();

    // Lazily resolved at most once per call — avoids a full StorageClass list for every PVC
    // that happens to have no explicit storageClassName.
    let mut default_sc_cache: Option<Option<String>> = None;

    for desired_template in sts_volume_claim_templates(desired_sts) {
        let template_name = match desired_template.metadata.name.as_deref() {
            Some(n) => n,
            None => continue,
        };

        let desired_size = match pvc_storage_request(desired_template) {
            Some(s) => s,
            None => continue,
        };

        // Only attempt expansion when the desired size differs from the existing template size.
        let existing_size = existing_templates
            .iter()
            .find(|t| t.metadata.name.as_deref() == Some(template_name))
            .and_then(pvc_storage_request);

        if existing_size == Some(desired_size) {
            continue;
        }

        // Warn early if the user reduced the size — Kubernetes does not allow PVC storage
        // to be shrunk, so the change will have no effect on existing volumes.
        if let Some(existing) = existing_size {
            match (
                parse_quantity_bytes(desired_size),
                parse_quantity_bytes(existing),
            ) {
                (Some(d), Some(e)) => {
                    if d < e {
                        warn!(
                            template = template_name,
                            existing_size = %existing.0,
                            desired_size = %desired_size.0,
                            "Desired storage size is smaller than the current size; \
                             Kubernetes does not allow PVC storage to be shrunk — this change is a no-op"
                        );
                        continue;
                    }
                }
                (d, e) => {
                    warn!(
                        template = template_name,
                        desired_size = %desired_size.0,
                        existing_size = %existing.0,
                        desired_parsed_bytes = ?d,
                        existing_parsed_bytes = ?e,
                        "Could not parse one or both storage quantities into bytes; \
                         skipping shrink guard and proceeding with resize"
                    );
                }
            }
        }

        for ordinal in 0..existing_replicas {
            let pvc_name = format!("{template_name}-{sts_name}-{ordinal}");

            let existing_pvc = match pvc_api.get_opt(&pvc_name).await? {
                Some(pvc) => pvc,
                None => {
                    debug!(pvc = pvc_name, "PVC not found; skipping resize");
                    continue;
                }
            };

            // Resolve the storage class name. Kubernetes normally populates storageClassName on
            // the created PVC even when the template omitted it (via the admission controller),
            // but fall back to the cluster default if it's absent or empty.
            let sc_name_owned: String;
            let sc_name: &str = match existing_pvc
                .spec
                .as_ref()
                .and_then(|s| s.storage_class_name.as_deref())
                .filter(|s| !s.is_empty())
            {
                Some(name) => name,
                None => {
                    // Resolve the default SC at most once per try_expand_pvcs call.
                    if default_sc_cache.is_none() {
                        default_sc_cache = Some(find_default_storage_class_name(&sc_api).await?);
                    }
                    match default_sc_cache.as_ref().unwrap() {
                        Some(name) => {
                            sc_name_owned = name.clone();
                            &sc_name_owned
                        }
                        None => {
                            warn!(
                                pvc = pvc_name,
                                "PVC has no storage class and no cluster default is \
                                 configured; skipping resize"
                            );
                            continue;
                        }
                    }
                }
            };

            // A storage class without allowVolumeExpansion is recorded as a failure so the
            // caller can revert the CR spec and emit an event — not silently skipped.
            if !storage_class_allows_expansion(&sc_api, sc_name).await? {
                warn!(
                    pvc = pvc_name,
                    storage_class = sc_name,
                    "Storage class does not support volume expansion"
                );
                if let Some(orig) = existing_size {
                    failures.push(PvcExpansionFailure {
                        template_name: template_name.to_string(),
                        pvc_name: pvc_name.clone(),
                        original_size: orig.clone(),
                        reason: format!(
                            "storage class '{sc_name}' does not support volume expansion"
                        ),
                    });
                }
                continue;
            }

            let patch = serde_json::json!({
                "spec": {
                    "resources": {
                        "requests": {
                            "storage": desired_size.0
                        }
                    }
                }
            });

            match pvc_api
                .patch(&pvc_name, &PatchParams::default(), &Patch::Merge(patch))
                .await
            {
                Ok(_) => {
                    info!(pvc = pvc_name, size = %desired_size.0, "Expanded PVC storage");
                    expanded_templates.insert(template_name.to_string());
                }
                Err(e) => {
                    warn!(pvc = pvc_name, error = %e, "Failed to expand PVC storage");
                    if let Some(orig) = existing_size {
                        failures.push(PvcExpansionFailure {
                            template_name: template_name.to_string(),
                            pvc_name: pvc_name.clone(),
                            original_size: orig.clone(),
                            reason: e.to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok((failures, expanded_templates))
}

/// Patches the `DiomCluster` spec to restore a volume's `size` field to `original_size`.
/// Called when PVC expansion fails so the CR reflects the actual on-disk state.
pub(crate) async fn revert_cluster_storage_size(
    cluster_api: &Api<DiomCluster>,
    name: &str,
    template_name: &str,
    original_size: &Quantity,
) -> Result<()> {
    let patch = match template_name {
        "persistent" => serde_json::json!({
            "spec": { "storage": { "persistent": { "size": original_size.0 } } }
        }),
        "logs" => serde_json::json!({
            "spec": { "storage": { "logs": { "size": original_size.0 } } }
        }),
        "snapshots" => serde_json::json!({
            "spec": { "storage": { "snapshots": { "size": original_size.0 } } }
        }),
        _ => {
            warn!(
                template = template_name,
                "Unknown volume template name; cannot revert CR spec storage size"
            );
            return Ok(());
        }
    };

    cluster_api
        .patch(name, &PatchParams::default(), &Patch::Merge(patch))
        .await?;

    info!(
        template = template_name,
        size = %original_size.0,
        "Reverted CR spec storage size to original value after expansion failure"
    );
    Ok(())
}

/// Returns true if the named storage class has `allowVolumeExpansion: true`.
async fn storage_class_allows_expansion(sc_api: &Api<StorageClass>, sc_name: &str) -> Result<bool> {
    Ok(sc_api
        .get_opt(sc_name)
        .await?
        .and_then(|sc| sc.allow_volume_expansion)
        .unwrap_or(false))
}

/// Returns the name of the cluster-default StorageClass (the one annotated with
/// `storageclass.kubernetes.io/is-default-class: "true"`), or `None` if none is configured.
async fn find_default_storage_class_name(sc_api: &Api<StorageClass>) -> Result<Option<String>> {
    let scs = sc_api.list(&Default::default()).await?;
    Ok(scs
        .items
        .into_iter()
        .find(|sc| {
            sc.metadata
                .annotations
                .as_ref()
                .and_then(|a| a.get("storageclass.kubernetes.io/is-default-class"))
                .is_some_and(|v| v == "true")
        })
        .and_then(|sc| sc.metadata.name))
}

fn sts_volume_claim_templates(sts: &StatefulSet) -> &[PersistentVolumeClaim] {
    sts.spec
        .as_ref()
        .and_then(|s| s.volume_claim_templates.as_deref())
        .unwrap_or(&[])
}

fn pvc_storage_request(pvc: &PersistentVolumeClaim) -> Option<&Quantity> {
    pvc.spec
        .as_ref()
        .and_then(|s| s.resources.as_ref())
        .and_then(|r| r.requests.as_ref())
        .and_then(|reqs| reqs.get("storage"))
}

fn pvc_storage_class(pvc: &PersistentVolumeClaim) -> Option<&str> {
    pvc.spec
        .as_ref()
        .and_then(|s| s.storage_class_name.as_deref())
}

/// Parses a Kubernetes [`Quantity`] string into bytes, returning `None` if the format is
/// not recognised. Handles binary SI suffixes (`Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`),
/// decimal SI suffixes (`k`, `M`, `G`, `T`, `P`, `E`), and plain integer bytes.
fn parse_quantity_bytes(q: &Quantity) -> Option<u64> {
    let s = q.0.trim();
    // Binary suffixes must be checked before decimal ones (e.g. "Mi" before "M").
    let suffixes: &[(&str, u64)] = &[
        ("Ei", 1u64 << 60),
        ("Pi", 1u64 << 50),
        ("Ti", 1u64 << 40),
        ("Gi", 1u64 << 30),
        ("Mi", 1u64 << 20),
        ("Ki", 1u64 << 10),
        ("E", 1_000_000_000_000_000_000),
        ("P", 1_000_000_000_000_000),
        ("T", 1_000_000_000_000),
        ("G", 1_000_000_000),
        ("M", 1_000_000),
        ("k", 1_000),
    ];
    for (suffix, factor) in suffixes {
        if let Some(num) = s.strip_suffix(suffix) {
            return num
                .trim()
                .parse::<u64>()
                .ok()
                .and_then(|n| n.checked_mul(*factor));
        }
    }
    s.parse::<u64>().ok()
}
