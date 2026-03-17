use std::collections::BTreeMap;

/// Labels applied to all resources managed by the operator for a given cluster instance.
pub(crate) fn common(cluster_name: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("app.kubernetes.io/name".into(), "coyote".into()),
        ("app.kubernetes.io/instance".into(), cluster_name.into()),
        (
            "app.kubernetes.io/managed-by".into(),
            "coyote-operator".into(),
        ),
    ])
}

/// Selector labels used to identify pods belonging to a cluster.
/// Must be a stable subset of `common` — these are written into the StatefulSet selector
/// and cannot be changed after creation.
pub(crate) fn selector(cluster_name: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("app.kubernetes.io/name".into(), "coyote".into()),
        ("app.kubernetes.io/instance".into(), cluster_name.into()),
    ])
}
