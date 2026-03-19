# coyote-operator Helm chart

Installs the Coyote CRD, operator and cluster.

## Usage

```yaml
# values.yaml
operator:
  image:
    tag: "0.1.0"

cluster:
  enabled: true
  image:
    tag: "0.1.0"
  spec:
    nodes: 3
    storage:
      persistent:
        size: 64Gi
        storageClass: gp3-encrypted
```

```sh
helm install coyote ./helm-coyote
```

## CRD installation

The `Cluster` CRD is installed by default (`crds.enabled: true`). Set it
to `false` if you want to manage it separately.

### Upgrades

The CRD is bundled in the chart's `crds/` sub-chart and is installed
automatically on `helm install`, but **is not upgraded on `helm upgrade` because Helm**.
When the CRD schema changes across chart versions, you have to apply it manually before upgrading the chart:

```sh
kubectl apply -f {path to crd.json}
```

### Uninstall

Helm will **not** delete the CRD on `helm uninstall`. To fully remove the CRD:

```sh
helm delete coyote
kubectl delete crd clusters.coyote.svix.com
```
