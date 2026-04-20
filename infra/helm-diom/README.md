# Diom Helm chart
Diom is the backend survival toolkit. It’s a set of well integrated infrastructure primitives for backend and data engineers such as caching, kv-store, rate-limiting, idempotency, queue, and stream. Please see the [Diom documentation](https://diom.svix.io/) for more details.

This chart installs components required for running Diom as a standalone instance or as a cluster, including a CRD, an operator, and a DiomCluster resource.

## Usage
The chart is distributed as an OCI artifact.

### Installation
```console
helm install [RELEASE_NAME] oci://ghcr.io/svix/charts/diom
```

## Parameters

### Configuring CRD Installation
The CRD is enabled by default. The chart will attempt to manage upgrades of the CRD, if enabled, using Helm hooks to manually apply CRD updates.

| Parameter | Description | Default |
|---|---|---|
| `crds.enabled` | Install the `DiomCluster` CRD. Set to `false` to manage the CRD yourself. | `true` |
| `crds.upgrade.enabled` | If enabled, the Helm chart will attempt to apply updates to the CRD. | `false` |
| `crds.upgrade.forceConflicts` | Passes `--force-conflicts` to `kubectl apply` to resolve CRD conflicts. | `false` |

### Configuring the Operator

| Parameter | Description | Default |
|---|---|---|
| `operator.image.repository` | Operator image repository. | `ghcr.io/svix/diom-operator` |
| `operator.image.tag` | Operator image tag. | Chart pre-populates current tag. |
| `operator.image.pullPolicy` | Operator image pull policy. | `IfNotPresent` |
| `operator.imagePullSecrets` | Operator image pull secrets. | `[]` |
| `operator.serviceAccount.create` | Create a ServiceAccount for the operator. | `true` |
| `operator.serviceAccount.name` | ServiceAccount name. Defaults to the release fullname. | `""` |
| `operator.serviceAccount.annotations` | Annotations for the ServiceAccount. | `{}` |
| `operator.rbac.create` | Create ClusterRole and ClusterRoleBinding for the operator. | `true` |
| `operator.logLevel` | Operator log level (`info`, `debug`, `trace`). | `info` |
| `operator.podAnnotations` | Annotations to add to the operator pod. | `{}` |
| `operator.resources` | Resource requests/limits for the operator pod. | request cpu: 100m, memory: 128Mi, no limits |
| `operator.nodeSelector` | Node selector for the operator pod. | `{}` |
| `operator.tolerations` | Tolerations for the operator pod. | `[]` |
| `operator.affinity` | Affinity rules for the operator pod. | `{}` |

### Configuring the Cluster

Example configuration for a 3-node cluster:

```yaml
cluster:
  spec:
    replicas: 3
    storage:
      persistent:
        size: 10Gi
    adminToken:
      valueFrom:
        name: diom-secrets
        key: admin-token
    internodeSecret:
      valueFrom:
        name: diom-secrets
        key: internode-secret
    logLevel: info
    logFormat: json
    opentelemetry:
      address: grpc://otel-collector.monitoring.svc.cluster.local:4317
```

| Parameter | Description | Default |
|---|---|---|
| `cluster.enabled` | Create a `DiomCluster` resource. Set to `false` to manage the cluster separately. | `true` |
| `cluster.name` | Name of the `DiomCluster` resource. Defaults to the release name. | `diom` |
| `cluster.image.repository` | Diom server image repository. | `ghcr.io/svix/diom-server` |
| `cluster.image.tag` | Diom server image tag. | Chart pre-populates current tag. |
| `cluster.spec.replicas` | Number of Diom replicas. Should be an odd number. Recommended value is 3 for a cluster, or 1 for a single node. | `1` |
| `cluster.spec.apiPort` | Port for the external API and service. | `8624` |
| `cluster.spec.envVar` | Additional environment variables to inject into pods (list of `{name, value}`). | `[]` |
| `cluster.spec.bootstrap` | Newline-delimited bootstrap script to run on cluster startup. | `""` |
| `cluster.spec.logLevel` | The log level to run the service with. Supported: info, debug, trace. | `""` |
| `cluster.spec.logFormat` | Log format for the Diom server (`default`, `json`). | `""` |
| `cluster.spec.opentelemetry.address` | OpenTelemetry tracing endpoint address (GRPC). | `""` |
| `cluster.spec.opentelemetry.metricsAddress` | OpenTelemetry metrics endpoint address, if different from `address`. | `""` |
| `cluster.spec.opentelemetry.metricsProtocol` | Protocol for OpenTelemetry metrics export (`grpc`, `http`). | `grpc` |
| `cluster.spec.adminToken.value` | Plaintext token for privileged API access, as a plain string. Only recommended for testing. | `""` |
| `cluster.spec.adminToken.valueFrom.name` | Name of the Kubernetes Secret containing the admin token. | `""` |
| `cluster.spec.adminToken.valueFrom.key` | Key within the Secret to use as the admin token. | `""` |
| `cluster.spec.internodeSecret.value` | Plaintext inter-node authentication secret. Only recommended for testing. | `""` |
| `cluster.spec.internodeSecret.valueFrom.name` | Name of the Kubernetes Secret containing the inter-node secret. | `""` |
| `cluster.spec.internodeSecret.valueFrom.key` | Key within the Secret to use as the inter-node secret. | `""` |
| `cluster.spec.storage.persistent.size` | Size of the persistent database volume **Required**. | `""` |
| `cluster.spec.storage.persistent.storageClass` | Storage class for the persistent volume. Uses the cluster default if unset. | `""` |
| `cluster.spec.storage.logs.size` | Size of the separate Raft commit log volume. | `""` |
| `cluster.spec.storage.logs.storageClass` | Storage class for the logs volume. | `""` |
| `cluster.spec.storage.snapshots.size` | Size of the separate Raft snapshot volume. Should be at least as large as the persistent volume. | `""` |
| `cluster.spec.storage.snapshots.storageClass` | Storage class for the snapshots volume. | `""` |
| `cluster.spec.imagePullPolicy` | Image pull policy (`Always`, `IfNotPresent`, `Never`). | `""` |
| `cluster.spec.podAnnotations` | Additional annotations to add to pods. | `{}` |
| `cluster.spec.nodeSelector` | Node selector labels for pod scheduling. | `{}` |
| `cluster.spec.tolerations` | Pod tolerations. | `[]` |
| `cluster.spec.affinity` | Pod affinity rules. | `{}` |
| `cluster.spec.topologySpreadConstraints` | Topology spread constraints. | `[]` |
| `cluster.spec.resources` | CPU and memory resource requests and limits for the diom pods. | `{}` |
| `cluster.spec.service` | Configuration for the externally-facing Service. | `{type: ClusterIP}` |


### Uninstall
Note that Helm will not delete the CRD or any persistent volumes associated with removed installations.
