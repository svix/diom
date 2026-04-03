use std::collections::BTreeMap;

/// Selector labels used to identify pods belonging to a cluster.
/// Must be stable — these are written into the StatefulSet selector and cannot be changed after creation.
pub(crate) fn selector(cluster_name: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("app.kubernetes.io/name".into(), "coyote".into()),
        ("app.kubernetes.io/instance".into(), cluster_name.into()),
        ("coyote.svix.com/cluster".into(), cluster_name.into()),
    ])
}

/// All labels applied to resources managed by the operator for a given cluster instance.
pub(crate) fn general_labels(cluster_name: &str) -> BTreeMap<String, String> {
    let mut l = selector(cluster_name);
    l.insert(
        "app.kubernetes.io/managed-by".into(),
        "coyote-operator".into(),
    );
    l
}
